use std::net::SocketAddr;
use std::env;
use std::string::String;
use std::vec::Vec;

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::Incoming;
use hyper::http::request::Parts;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Error;
use hyper::{body::Body, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};
use auxiliary_functions::*;
use auxiliary_functions::sequence::models::Sequence;


const NUMBER: u8 = 0; // Change this to 1 or 2 as needed

const NORMAL: Server = Server {
    port: 0, // Placeholder, not used
    keyword: "",
    name: ""
};

const AMONG_US: Server = Server {
    port: 0, // Placeholder, not used
    keyword: "_Imposter",
    name: " & AmongUs"
};

const ELVES: Server = Server {
    port: 0, // Placeholder, not used
    keyword: "_Elves",
    name: " & Elves"
};


static MY: Server = select_server(NUMBER);

const fn select_server<'a>(number: u8) -> Server<'a> {
    match number {
        0 => NORMAL,
        1 => AMONG_US,
        2 => ELVES,
        _ => NORMAL, // Default to NORMAL if NUMBER is out of range
    }
}


static KEYWORD : &str = MY.keyword;
static NAME : &str = MY.name;
const REGISTER : &str =  "http://127.0.0.1:7878/project";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    let mut k = KEYWORD;
    if NUMBER == 2 {
        k = "_Imposter"
    }

    // Read the port number from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <port>", args[0]);
        return Err("Port number not provided".into());
    }
    let port: u16 = args[1].parse().expect("Invalid port number");

    let addr: SocketAddr = ([127, 0, 0, 1], port).into();

    let b = send_get("http://127.0.0.1:7878/project".to_string()).await?;
    println!("HERE {}", b);

    let b = send_post(
        "http://127.0.0.1:7878/project".to_string(),
        serde_json::to_string(&get_project()).unwrap(),
    )
    .await?;
    println!("HERE {}", b);

    let b = send_get("http://127.0.0.1:7878".to_string()).await?;
    println!("HERE {}", b);

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    let proj_name = get_project().name;
    println!("Server name : {}", proj_name);

    let create_404 = || {
        let mut not_found = Response::new(empty());
        *not_found.status_mut() = StatusCode::NOT_FOUND;
        Ok(not_found)
    };

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let service = service_fn(move |req| {
            async move {
                
                let method = req.method().clone();
                
                let path = req.uri().path().to_owned();
                let path = path.as_str();
                
                //Helps us return a sequence when the sequence is not on our server...
                let mut condition = false;

                let mut result : Vec<Option<f64>> = Vec::new();
                
                match (method, path) {
                    (Method::GET, "/ping") => {
                        println!("****************************-BEGIN_REQUEST\n");
                        println!("Got a GET /ping request. Sending info about my project.\n");
                        println!("****************************-END_REQUEST\n");
                        Ok::<_, Error>(Response::new(full(
                        
                            serde_json::to_string(&get_project()).unwrap(),
                        )))
                    },
                    (Method::GET, "/sequence") => {
                        //
                        println!("****************************-BEGIN_REQUEST\n");
                        println!("Got a GET /sequence request. Sending a list of my sequences.\n");
                        println!("****************************-END_REQUEST\n");
                        let sequences = sequences();
                        Ok(Response::new(full(
                            serde_json::to_string(&sequences).unwrap(),
                        )))
                    }
                    (Method::POST, r) => {
                        // r is a path to some sequence in the project. For example r might be something like r = /sequence/Arithmetic
                        let seqs = sequences();
                        let mut finding_sequence = seqs
                            .iter()
                            .find(|&x| ("/sequence/".to_string() + &x.name) == r);
                        let body = collect_body(req).await?;
                        let request: SequenceRequest = serde_json::from_str(&body).unwrap();                       
                        let seqs = sequences();
                        let mut sgn_is_ok = true;
                        let mut maybe_sgn_error = Vec::new();
                        match finding_sequence {
                            Some(_) => {
                                for s in seqs {
                                    if let Some(ref fs) = finding_sequence {
                                        if s.name == fs.name {
                                            // Check whether the signature on our server is correct....
                                            
                                            let no_parameters = s.parameters;
                                            let no_sequence_parameters = s.sequences;
                                            let no_requested_parameters = request.parameters.len() as u32;
                                            let no_requested_sequence_parameters = request.sequences.len() as u32;
                        
                                            // Check that the signature of this 'found' sequence is the same as the requested signature....
                                                      
                                            if no_parameters != no_requested_parameters || no_sequence_parameters != no_requested_sequence_parameters {
                                                finding_sequence = None;
                                                sgn_is_ok = false;
                                                let r_info = format!("Requested signature \n Number of parameters : {}\n Number of sequence parameters : {}\n\n", no_requested_parameters, no_requested_sequence_parameters);
                                                let f_info = format!("The found signature \n Number of parameters : {}\n Number of sequence parameters : {}\n\n", no_parameters, no_sequence_parameters);
                                                maybe_sgn_error.push(r_info);
                                                maybe_sgn_error.push(f_info);
                                            }
                                        }
                                    }
                                }
                            }
                            None => (),
                        }
                        
                        match finding_sequence {
                            None => {
                                
                                println!("****************************-BEGIN_REQUEST\n");
                                if sgn_is_ok {
                                    println!("Got a POST {} request. The desired sequence is not available on this server. Looking around the hood if anyone has it.\n", r);
                                } else {
                                    println!("Got a POST {} request. The desired sequence is available on this server, but not with the requested signature.\n", r);
                                    println!("The signaturs did not match.\n");
                                    println!("{}", maybe_sgn_error[0]);
                                    println!("{}", maybe_sgn_error[1]);
                                    
                                    println!(" Looking around the hood if anyone has it.\n");        
                                }
                                let all_projects : String = send_get(REGISTER.to_string()).await.unwrap();
                                let all_projects: Vec<Project> = serde_json::from_str(&all_projects).unwrap();
                            'outer: for project in all_projects.iter() {
                                    
                                    let url_for_get : String = format!("http://{}:{}/sequence", project.ip, project.port);
                                    
                                    if project.port != port {
                                        let sequences_in_this_project = send_get(url_for_get).await.unwrap();
                                        let sequences_in_this_project : Vec<SequenceInfo> = serde_json::from_str(&sequences_in_this_project).unwrap();
                                        for s in sequences_in_this_project {
                                            
                                            let no_parameters = s.parameters;
                                            let no_sequence_parameters = s.sequences;
                                            let name = s.name.clone();

                                            if ("/sequence/".to_string() + &name) == r {
                                                //We found the sequence! But is the signature ok????
                                                println!("----------BEGIN_FINDING");
                                                println!("Found the sequence on the project : {:?}\n", project);
                                                println!("We shall check whether the signature of this found sequence matches the signature of the requested sequence.\n");
                                                condition = true;
                                                let url_for_post = format!("http://{}:{}{}", project.ip, project.port, r);

                                                //let body = collect_body(req).await.unwrap();
                                                
                                                
                                                let no_requested_parameters = request.parameters.len() as u32;
                                                let no_requested_sequence_parameters = request.sequences.len() as u32;
                                                
                                                //Check that the signature of this 'found' sequence is the same as the requested signature....
                                                if no_parameters == no_requested_parameters && no_sequence_parameters == no_requested_sequence_parameters {
                                                    println!("The signatures match.\n");
                                                    println!("Sending a POST request to this project. The url is {}\n", url_for_post);
                                                    let post = send_post(url_for_post, body).await.unwrap();
                                                    
                                                    let post = post.as_str();
                                                    
                                                    let values = parse_string_to_vec(post);
                                                    for x in values {
                                                        //Give it into the 'result' vector...
                                                        result.push(x)
                                                    }
                                                    break 'outer
                                                }
                                                println!("The signaturs did not match.\n");
                                                println!("Requested signature \n Number of parameters : {}\n Number of sequence parameters : {}\n\n", no_requested_parameters, no_requested_sequence_parameters);
                                                println!("The found signature \n Number of parameters : {}\n Number of sequence parameters : {}\n\n", no_parameters, no_sequence_parameters);
                                                println!("Do not lose hope, the search shall go on!");
                                                println!("----------END_FINDING\n");
                                            }
                                        }
                                        
                                    }
                                
                                                                   
                                }
                                if condition {
                                    println!("Returning the desired range.\n");
                                    println!("We sent this : {:?}", result);
                                    println!("----------END_FINDING\n");
                                    println!("****************************-END_REQUEST\n");
                                    Ok(Response::new(full(
                                        serde_json::to_string(&result).unwrap(),
                                    )))

                                } else {
                                    println!("Got a POST {} request. No server had this sequence. Returning 404.\n", r);
                                    
                                    println!("****************************-END_REQUEST\n");
                                    create_404()
                                }
                            },
                            Some(s) if *s.name == ("Arithmetic".to_owned() + KEYWORD).to_string() => {
                                println!("****************************-BEGIN_REQUEST\n");
                                println!("Got a POST {} request. This sequence is available on this server with the requested signature. Returning the desired range.\n", r);
                                
                                
                                let range = request.range;
                                let seq =
                                    sequence::arithmetic::Arithmetic::new(request.parameters[0], request.parameters[1]);
                                
                                let ra = range.clone();
                                let alfa = &seq.range(ra);
                                println!("We sent this : {:?}\n", alfa);

                                println!("****************************-END_REQUEST\n");
                                Ok(Response::new(full(
                                    serde_json::to_string(&seq.range(range)).unwrap(),
                                )))
                            }
                            //tule imamo posebej k, za primer ko imam Elves....
                            
                            Some(s) if *s.name == ("Constant".to_owned() + k).to_string() => {
                                println!("****************************-BEGIN_REQUEST\n");
                                println!("Got a POST {} request. This sequence is available on this server with the requested signature. Returning the desired range.\n", r);
                                
                                
                                let range = request.range;
                                let seq =
                                    sequence::constant::Constant::new(request.parameters[0]);
                                let ra = range.clone();
                                let alfa = &seq.range(ra);
                                println!("We sent this : {:?}\n", alfa);

                                println!("****************************-END_REQUEST\n");
                                Ok(Response::new(full(
                                    serde_json::to_string(&seq.range(range)).unwrap(),
                                )))
                            }
                            Some(s) if *s.name == ("Lin Comb".to_owned() + KEYWORD).to_string() => {
                                println!("****************************-BEGIN_REQUEST\n");
                                println!("Got a POST {} request. This sequence is available on this server with the requested signature. Returning the desired range.\n", r);
                                
                                
                                let range = request.range;
                                let mut sequences : Vec<& dyn Sequence<f64>> = vec![];
                                
                                let seq =
                                    //sequence::linearcombination::linear_combination(request.parameters[0], request.parameters[1]);
                                    sequence::constant::Constant::new(request.parameters[0]);
                                let ra = range.clone();
                                let alfa = &seq.range(ra);
                                println!("We sent this : {:?}\n", alfa);

                                println!("****************************-END_REQUEST\n");
                                Ok(Response::new(full(
                                    serde_json::to_string(&seq.range(range)).unwrap(),
                                )))
                            }
                            _ => panic!("Not implemented"),
                        }
                    }

                    _ => create_404(),
                }
            }
        });

        if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
            println!("Error serving connection: {:?}", err);
        }
    }
}