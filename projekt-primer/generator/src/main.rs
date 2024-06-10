use std::net::SocketAddr;

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Error;
use hyper::{body::Body, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use sequence::models::Range;
use serde::{Deserialize, Serialize};
use sequence::constant::Constant;
use sequence::models::Sequence;
use sequence::arithmetic::Arithmetic;
use sequence::linearcombination::LinearCombination;
use sequence::linearcombination;


use hyper::http::request::Parts;
use futures::executor::block_on;

use std::string::String;
use std::vec::Vec;




const PORT: u16 = 12345;
const REGISTER : &str =  "http://127.0.0.1:7878/project";

pub mod expression;
pub mod sequence;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub ip: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceSyntax {
    pub name: String,
    pub parameters: Vec<f64>,
    pub sequences: Vec<Box<SequenceSyntax>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceRequest {
    pub range: Range,
    pub parameters: Vec<f64>,
    pub sequences: Vec<Box<SequenceSyntax>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceInfo {
    name: String,
    description: String,
    parameters: u32,
    sequences: u32,
}



fn sequences() -> Vec<SequenceInfo> {
    let mut sequences = Vec::new();
    sequences.push(SequenceInfo {
        name: "Arithmetic".to_string(),
        description: "Arithmetic sequence".to_string(),
        parameters: 2,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "Constant".to_string(),
        description: "Constant sequence".to_string(),
        parameters: 1,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "Lin Comb".to_string(),
        description: "".to_string(),
        parameters: 3,
        sequences: 2,
    });
    sequences
}

fn get_project() -> Project {
    return Project {
        name: "Binarni Banditi".to_string(),
        ip: "127.0.0.1".to_string(),
        port: PORT,
    };
}





fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
async fn collect_body(req: Request<Incoming>) -> Result<String, hyper::Error> {
    let max = req.body().size_hint().upper().unwrap_or(u64::MAX);
    if max > 1024 * 64 {
        panic!("Body too big");
    }

    let whole_body = req.collect().await?.to_bytes();
    let whole_body = std::str::from_utf8(&whole_body).unwrap().to_string();
    return Ok(whole_body);
}




fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

async fn send_post(url: String, body: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post(url).body(body).send().await?.text().await?;
    return Ok(res);
}

fn parse_string_to_vec(input: &str) -> Vec<Option<f64>> {
    // Remove the brackets and trim whitespace
    
    let trimmed_input = input.trim().trim_start_matches('[').trim_end_matches(']').trim();
    
    // Split the string by commas and trim whitespace from each part
    let parts: Vec<&str> = trimmed_input.split(',').map(|s| s.trim().trim_matches('"')).collect();
    
    // Convert parts to Vec<Option<f64>>
    let result: Vec<Option<f64>> = parts.iter().map(|&s| {
        if s == "None" {
            None
        } else {
            s.parse::<f64>().ok()
        }
    }).collect();
    
    result
}



async fn send_get(url: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?.text().await?;
    return Ok(res);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    //let scalers = vec![2.0, 3.0, 1.5];
    //let scalers2 = scalers.clone();
    //let s1 = Arithmetic::new(1.2, 2.6);
    //let s2 = Constant::new(5.0);
    //let s1 : &dyn Sequence<f64> = &s1;
    //let s2 : &dyn Sequence<f64> = &s2;
    //let indeksi : Vec<usize> = vec![1, 2, 3];
    
    //let s3t: &dyn Sequence<i64> = &s3_
    //let lin_primer = linearcombination::linear_combination(vec![Box::new(s1), Box::new(s2)], scalers);
    //println!("{}", lin_primer.name());
    //for k in indeksi {
    //    println!("Summing the {k}-th");
    //    let od_konstantnega = s2.k_th(k);
    //    let od_aritmeticnega = s1.k_th(k);
    //    println!("{:?} * 1.0 + {:?} * {:?} + {:?} * {:?} = {:?}", scalers2[0], scalers2[1], od_aritmeticnega, scalers2[2], od_konstantnega, lin_primer.k_th(k));
    //}
    //generator::my_fun();
    //let primer = Constant::new(1);
    //println!("{:?}", primer);
    //let range = Range {
    //    from : 3,
    //    to : 6,
    //    step : 1
    //};
    //let r = primer.range(range);
    //println!("{:?}", r);
    //
    let addr: SocketAddr = ([127, 0, 0, 1], PORT).into();

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
                        let finding_sequence = seqs
                            .iter()
                            .find(|&x| ("/sequence/".to_string() + &x.name) == r);
                        match finding_sequence {
                            None => {
                                let body = collect_body(req).await.unwrap();
                                                
                                println!("****************************-BEGIN_REQUEST\n");
                                println!("Got a POST {} request. The desired sequence is not available on this server. Looking around the hood if anyone has it.\n", r);
                                let all_projects : String = send_get(REGISTER.to_string()).await.unwrap();
                                let all_projects: Vec<Project> = serde_json::from_str(&all_projects).unwrap();
                            'outer: for project in all_projects.iter() {
                                    
                                    let url_for_get : String = format!("http://{}:{}/sequence", project.ip, project.port);
                                    
                                    if project.port != PORT {
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
                                                
                                                let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                                                
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
                                    println!("----------END_FINDING\n");
                                    println!("****************************-END_REQUEST\n");
                                    create_404()
                                }
                            },
                            Some(s) if *s.name == "Arithmetic".to_string() => {
                                println!("****************************-BEGIN_REQUEST\n");
                                println!("Got a POST {} request. This sequence is available on this server. Returning the desired range.\n", r);
                                
                                let body = collect_body(req).await?;
                                let request: SequenceRequest = serde_json::from_str(&body).unwrap();
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
                            Some(s) if *s.name == "Constant".to_string() => {
                                println!("****************************-BEGIN_REQUEST\n");
                                println!("Got a POST {} request. This sequence is available on this server. Returning the desired range.\n", r);
                                
                                let body = collect_body(req).await?;
                                let request: SequenceRequest = serde_json::from_str(&body).unwrap();
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
                            Some(s) if *s.name == "Lin Comb".to_string() => {
                                println!("****************************-BEGIN_REQUEST\n");
                                println!("Got a POST {} request. This sequence is available on this server. Returning the desired range.\n", r);
                                
                                let body = collect_body(req).await?;
                                let request: SequenceRequest = serde_json::from_str(&body).unwrap();
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
