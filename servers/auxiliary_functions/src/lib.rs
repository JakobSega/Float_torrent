//use std::net::SocketAddr;
use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::Incoming;
//use hyper::server::conn::http1;
//use hyper::service::service_fn;
//use hyper::Error;
use hyper::{body::Body, Request};
//use sequence::story::StorySequence;
//use hyper::{body::Body, Method, Request, Response, StatusCode};
//use hyper_util::rt::TokioIo;
//use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::string::String;
use std::vec::Vec;
//use std::str::FromStr;
use dotenv::dotenv;
use std::env;

use std::num::ParseIntError;

use async_recursion::async_recursion;
use crate::sequence::models::Range;
use crate::sequence::models::Sequence;
//use crate::sequence::story::generate_story;

pub fn string_to_u16(s: &String) -> Result<u16, ParseIntError> {
    s.parse::<u16>()
}

//const NUMBER: u8 = 0; // Change this to 1 or 2 as needed. 1 is the Imposter server and 2 is the Elves server.
pub const NORMAL : Server = Server {
    port : 12345,
    keyword : "",
    name : ""
};
pub const AMONG_US : Server = Server {
    port : 12346,
    keyword : "_Imposter",
    name : " & AmongUs"
};
pub const ELVES : Server = Server {
    port : 12347,
    keyword : "_Elves",
    name : " & Elves"
};

//static MY: Server = select_server(NUMBER);

pub const fn select_server<'a>(number: u8) -> Server<'a> {
    match number {
        0 => NORMAL,
        1 => AMONG_US,
        2 => ELVES,
        _ => NORMAL, // Default to NORMAL if NUMBER is out of range
    }
}
//static server.port: u16 = MY.port;
//static server.keyword : &str = MY.keyword;
//static server.name : &str = MY.name;

const REGISTER : &str =  "http://127.0.0.1:7878/project";


#[derive(Debug)]
pub struct Server<'a> {
    pub port : u16,
    pub keyword : &'a str,
    pub name : &'a str,
}

// Re-export StorySyntax to make it accessible from other modules


// Module declarations
pub mod expression {
    pub mod evaluation;
    pub mod models;
}

pub mod sequence {
    pub mod arithmetic;
    pub mod combined;
    pub mod constant;
    pub mod em_sequence;
    pub mod fibonacci;
    pub mod geometric;
    pub mod hofstadter;
    pub mod linearcombination;
    pub mod models;
    pub mod recaman;
    pub mod shifted;
    pub mod story;
    pub mod ai;
    
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub ip: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

//?pub?
fn make_seq_req_from_syntax(range: Range, seq_syn: SequenceSyntax) -> (SequenceRequest, String) {
    (
        SequenceRequest {
            range: range,
            parameters: seq_syn.parameters,
            sequences: seq_syn.sequences,
        },
        seq_syn.name,
    )
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
pub struct SequenceInfo {
    name: String,
    description: String,
    parameters: u32,
    sequences: u32,
}

pub fn get_groq_api_key() -> String {
    dotenv().ok(); // Load environment variables from .env file
    env::var("GROQ_API_KEY").expect("GROQ_API_KEY not found in environment variables")
}

async fn get_sequence(request : &SequenceRequest, i : usize, range: Range, error_message : &mut Box<String>,my_url : &str) -> Option<Vec<Option<f64>>> {
    let s = &request.sequences[i];
    let (path1, cond1, body1) = from_seq_syntax_to_recursive_call((*(*s)).clone(), range);
    let result1 = Box::pin(handle_post(error_message,&path1, cond1, my_url, body1));
    let vec1 = *Pin::into_inner(result1.await);
    vec1

}

pub fn sequences(server: &Server, number: u8) -> Vec<SequenceInfo> {
    let mut sequences = Vec::new();
    sequences.push(SequenceInfo {
        name: ("Arithmetic".to_owned() + server.keyword).to_string(),
        description: "Arithmetic sequence. The first parameter is the starting element, the second is the difference. So a_n = a_o + n * d.".to_string(),
        parameters: 2,
        sequences: 0,
    });
    //V primeru serverja 'Elves' hočemo, da ima ime konstanega zaporedja enako kot ime konstantnega na serverju AmongUs, da prečekiramo, da vse deluje ok....
    let mut k = server.keyword;
    if number == 2 {
        k = "_Imposter"
    }
    let mut m = 0;
    if number == 2 {
        m = 1
    }
    sequences.push(SequenceInfo {
        name: ("Constant".to_owned() + k).to_string(),
        description: "Constant sequence. The parameter determines the constant.".to_string(),
        parameters: 1,
        sequences: m,
    });
    sequences.push(SequenceInfo {
        name: ("LinearCombination".to_owned() + server.keyword).to_string(),
        description: "LinearCombination accepts 3 scaler parameters : l_0, l_1, l_2 and 2 sequence parameters : (a_n), (b_n). This sequence is then defined as c_n = l_0 + l_1 * a_n + l_2 * b_n".to_string(),
        parameters: 3,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: ("Sum".to_owned() + server.keyword).to_string(),
        description: "Sum accepts two sequence parameters : (a_n) and (b_n). The sum is then defined by c_n = a_n + b_n".to_string(),
        parameters: 0,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: ("Product".to_owned() + server.keyword).to_string(),
        description: "Product accepts 2 sequence parameters : (a_n) and (b_n). The product is then defined by c_n = a_n * b_n".to_string(),
        parameters: 0,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: ("Drop".to_owned() + server.keyword).to_string(),
        description: "Drop takes 1 parameter i and 1 sequence parameter (a_n). The drop is then defined by c_n = a_(n + i)".to_string(),
        parameters: 1,
        sequences: 1,
    });
    sequences.push(SequenceInfo {
        name: ("Geometric".to_owned() + server.keyword).to_string(),
        description: "Geometric sequence take 2 scaler parameters : a and q. The geometric sequence is then defined by c_n = a * q^n.".to_string(),
        parameters: 2,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: ("Fibonacci".to_owned() + server.keyword).to_string(),
        description: "Fibonacci takes two scaler parameters a_0 and a_1. The sequence is defined recursively as : c_0 = a_0, c_1 = a_1 and c_n = c_(n - 1) + c_(n - 2), where n > 1.".to_string(),
        parameters: 2,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: ("EMSequence".to_owned() + server.keyword).to_string(),
        description: "Euler-Mascheroni sequence approximation takes no parameters. It is an approximation of the Euler-Mascheroni constant. The sequence is defined by c_n = H(n) - log(n), where H(n) is the partial sum of the harmonic series: H(n) = 1 + 1/2 + 1/3 + ... + 1/n.".to_string(),
        parameters: 0,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: ("NthRootSequence".to_owned() + server.keyword).to_string(),
        description: ("NthRootSequence takes 1 sequence parameter (a_n). It is defined by c_n = (a_n)^(1/n)").to_string(),
        parameters: 0,
        sequences: 1,
    });
    sequences.push(SequenceInfo {
        name: ("Hofstadter".to_owned() + server.keyword).to_string(),
        description: ("Hofstadter sequence takes no parameters. It is defined recursively : G_0 = 0 and G_n =n − G_(G_(n - 1)) wher n > 0.").to_string(),
        parameters: 0,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: ("Recaman".to_owned() + server.keyword).to_string(),
        description: ("Recaman sequence takes no parameters. It is defined as follows: c_0 = 0 and let n_n = a_(n - 1) - n and p_n = a_(n - 1) + n. Then a_n = n_n if n_n is positive and has not yet occurd in the sequence, otherwise a_n = p_n.").to_string(),
        parameters: 0,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: ("Story".to_owned() + server.keyword).to_string(),
        description: "Story sequence takes two parameters which represent an author and a genre and a vector of sequences and uses groq api to generate a short story as if writen by the author in the genre and encodes it in binary.".to_string(),
        parameters: 2,
        sequences: 1,
    });
    sequences.push(SequenceInfo {
        name: ("Ai".to_owned() + server.keyword).to_string(),
        description: "Ai takes 1 sequence parameter imput_sequence and 1 scalar parameter which represents the desired predicted range to be generated by AI.".to_string(),
        parameters: 1,
        sequences: 1,
    });
    sequences
}

pub fn get_project(server: &Server) -> Project {
    return Project {
        name: ("Binarni Banditi".to_owned() + server.name).to_string(),
        ip: "127.0.0.1".to_string(),
        port: server.port,
    }
}

pub fn get_project_new(ip: String, port: u16, server: &Server) -> Project {
    Project {
        name: ("Binarni Banditi".to_owned() + server.name).to_string(),
        ip,
        port,
    }
}



pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

pub async fn collect_body(req: Request<Incoming>) -> Result<String, hyper::Error> {
    let max = req.body().size_hint().upper().unwrap_or(u64::MAX);
    if max > 1024 * 64 {
        panic!("Body too big");
    }

    let whole_body = req.collect().await?.to_bytes();
    let whole_body = std::str::from_utf8(&whole_body).unwrap().to_string();
    return Ok(whole_body);
}

pub fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

pub async fn send_post(url: String, body: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post(url).body(body).send().await?.text().await?;
    return Ok(res);
}

pub async fn send_get(url: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?.text().await?;
    return Ok(res);
}

pub async fn search_generators<'a>(r :&str, request : SequenceRequest, body : String, mut condition : bool, sgn_is_ok : bool, maybe_sgn_error : Vec<String>, server: &Server<'a>) -> (Vec<Option<f64>>, bool) {
    //r is a path to the sequence on the server...
    let mut result : Vec<Option<f64>> = Vec::new();
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
        
        if project.port != server.port {
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
                    
                    let url_for_post = format!("http://{}:{}{}", project.ip, project.port, r);

                    //let body = collect_body(req).await.unwrap();
                    
                    
                    let no_requested_parameters = request.parameters.len() as u32;
                    let no_requested_sequence_parameters = request.sequences.len() as u32;
                    
                    //Check that the signature of this 'found' sequence is the same as the requested signature....
                    if no_parameters == no_requested_parameters && no_sequence_parameters == no_requested_sequence_parameters {
                        condition = true;
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
    return (result, condition)
    }


//?pub?
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

//?pub?
fn lin_comb_from_vec(
    vec1: Vec<Option<f64>>, 
    vec2: Vec<Option<f64>>, 
    a0: f64, 
    a1: f64, 
    a2: f64
) -> Vec<Option<f64>> {
    assert_eq!(vec1.len(), vec2.len());
    let mut vec = Vec::with_capacity(vec1.len()); // Allocate vector with capacity for efficiency

    for (x, y) in vec1.iter().zip(vec2.iter()) {
        let result = match (x, y) {
            (Some(x_val), Some(y_val)) => Some(a0 + a1 * x_val + a2 * y_val),
            _ => None,
        };
        vec.push(result);
    }

    vec
}

//?pub?
fn pointwise_multiply(vec1: Vec<Option<f64>>, vec2: Vec<Option<f64>>) -> Vec<Option<f64>> {
    // Check if the vectors have the same length
    if vec1.len() != vec2.len() {
        panic!("Vectors must be of the same length");
    }

    vec1.into_iter()
        .zip(vec2.into_iter())
        .map(|(a, b)| match (a, b) {
            (Some(x), Some(y)) => Some(x * y),
            _ => None,
        })
        .collect()
}

//?pub?
fn from_seq_syntax_to_recursive_call(s1 : SequenceSyntax, range : Range) -> (String, bool, String) {
    let (r1, name1) = make_seq_req_from_syntax(range, s1);
            
    let path1 = "/sequence/".to_owned() + &name1;
    
    let body1 = serde_json::to_string(&r1).unwrap();
    let c1 = false;
    let y = (path1, c1, body1);
    y
}   


#[async_recursion]
pub async fn handle_post(
    error_message: &mut Box<String>,
    r: &str,
    condition: bool,
    my_url: &str,
    body: String,
    server: &Server,
    number: u8
) -> Option<Vec<Option<f64>>>  {
    
    let mut k = server.keyword;
    if number == 2 {
        k = "_Imposter"
    };
    
    
    // r is a path to some sequence in the project. For example r might be something like r = /sequence/Arithmetic
    let seqs = sequences(server, number);
    let mut finding_sequence = seqs
        .iter()
        .find(|&x| ("/sequence/".to_string() + &x.name) == r);
    
    let request: SequenceRequest = serde_json::from_str(&body).unwrap();                       
    
    let seqs = sequences(server, number);
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
            let (result, condition) = search_generators(r, request, body, condition, sgn_is_ok, maybe_sgn_error, server).await;
            println!("{}", condition);
            if condition {
                println!("Returning the desired range.\n");
                println!("We sent this : {:?}", result);
                println!("----------END_FINDING\n");
                println!("****************************-END_REQUEST\n");
               
                
                Some(result)
                

            } else {
                println!("Got a POST {} request. No server had this sequence. Returning 404.\n", r);
                let er_msg = "None of the available servers had the requested sequence with the requested signature.".to_string();
                **error_message = er_msg;
                println!("****************************-END_REQUEST\n");
                
                None
            }
        },
        Some(s) if *s.name == ("Geometric".to_owned() + server.keyword).to_string() => {
            println!("****************************-BEGIN_REQUEST\n");
            println!("Got a POST {} request. This sequence is available on this server with the requested signature. Returning the desired range.\n", r);
            
            
            let range = request.range;
            let seq =
                sequence::geometric::Geometric::new(request.parameters[0], request.parameters[1]);
            
            let ra = range.clone();
            let alfa = &seq.range(ra);
            println!("We sent this : {:?}\n", alfa);

            println!("****************************-END_REQUEST\n");
            
            let result = seq.range(range);
            Some(result)
        }
        Some(s) if *s.name == ("Fibonacci".to_owned() + server.keyword).to_string() => {
            println!("****************************-BEGIN_REQUEST\n");
            println!("Got a POST {} request. This sequence is available on this server with the requested signature. Returning the desired range.\n", r);
            
            
            let range = request.range;
            let seq =
                sequence::fibonacci::Fibonacci::new(request.parameters[0], request.parameters[1]);
            
            let ra = range.clone();
            let alfa = &seq.range(ra);
            println!("We sent this : {:?}\n", alfa);

            println!("****************************-END_REQUEST\n");
            
            let result = seq.range(range);
            Some(result)
        }
        Some(s) if *s.name == ("Arithmetic".to_owned() + server.keyword).to_string() => {
            println!("****************************-BEGIN_REQUEST\n");
            println!("Got a POST {} request. This sequence is available on this server with the requested signature. Returning the desired range.\n", r);
            
            
            let range = request.range;
            let seq =
                sequence::arithmetic::Arithmetic::new(request.parameters[0], request.parameters[1]);
            
            let ra = range.clone();
            let alfa = &seq.range(ra);
            println!("We sent this : {:?}\n", alfa);

            println!("****************************-END_REQUEST\n");
            
            let result = seq.range(range);
            Some(result)
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
            
            let result = seq.range(range);
            Some(result)
        }
        Some(s) if *s.name == ("EMSequence".to_owned() + server.keyword).to_string() => {
            println!("****************************-BEGIN_REQUEST\n");
            println!("Got a POST {} request. This sequence is available on this server with the requested signature. Returning the desired range.\n", r);
            
            
            let range = request.range;
            let seq =
                sequence::em_sequence::EMSequence::new();
            let ra = range.clone();
            let alfa = &seq.range(ra);
            println!("We sent this : {:?}\n", alfa);

            println!("****************************-END_REQUEST\n");
            
            let result = seq.range(range);
            Some(result)
        }
        Some(s) if *s.name == ("Hofstadter".to_owned() + server.keyword).to_string() => {
            println!("****************************-BEGIN_REQUEST\n");
            println!("Got a POST {} request. This sequence is available on this server with the requested signature. Returning the desired range.\n", r);
            
            
            let range = request.range;
            let seq =
                sequence::hofstadter::HofstadterSequence::new();
            let ra = range.clone();
            let alfa = &seq.range(ra);
            println!("We sent this : {:?}\n", alfa);

            println!("****************************-END_REQUEST\n");
            
            let result = seq.range(range);
            Some(result)
        }
        Some(s) if *s.name == ("Recaman".to_owned() + server.keyword).to_string() => {
            println!("****************************-BEGIN_REQUEST\n");
            println!("Got a POST {} request. This sequence is available on this server with the requested signature. Returning the desired range.\n", r);
            
            let range = request.range;
            let seq =
                sequence::recaman::RecamanSequence::new();
            let ra = range.clone();
            let alfa = &seq.range(ra);
            
            println!("We sent this : {:?}\n", alfa);

            println!("****************************-END_REQUEST\n");
            
            let result = seq.range(range);
            Some(result)
        }

        Some(s) if *s.name == ("LinearCombination".to_owned() + server.keyword).to_string() => {
            println!("****************************-BEGIN_REQUEST\n");
            println!("Got a POST {} request. This sequence is available on this server with the requested signature. Returning the desired range.\n", r);
            
            let range = request.range;
            
            let (lambda0, lambda1, lambda2) = (request.parameters[0], request.parameters[1], request.parameters[2]) ;
            let vec1 = get_sequence(&request, 0, range, error_message, my_url).await;
            
            let vec2 = get_sequence(&request, 1, range, error_message, my_url).await;
            
            match (vec1, vec2) {
                (Some(v1), Some(v2)) => {
                    
                    let result = lin_comb_from_vec(v1, v2, lambda0, lambda1, lambda2);
                    println!("We sent this : {:?}\n", result);

                    println!("****************************-END_REQUEST\n");
                    
                    return Some(result)

                },
                (_, _) => {
                    println!("We sent an empty list. Some of the deeper sequences were not available.\n");

                    println!("****************************-END_REQUEST\n");
            
                    return None
                }
            }
         }
         Some(s) if *s.name == ("Sum".to_owned() + server.keyword).to_string() => {
            println!("****************************-BEGIN_REQUEST\n");
            println!("Got a POST {} request. This sequence is available on this server with the requested signature. Returning the desired range.\n", r);
            
            
            let range = request.range;
            
            
            let vec1 = get_sequence(&request, 0, range, error_message, my_url).await;
            let vec2 = get_sequence(&request, 1, range, error_message, my_url).await;
            
            match (vec1, vec2) {
                (Some(v1), Some(v2)) => {
                    let result = lin_comb_from_vec(v1, v2, 0.0, 1.0, 1.0);
                    println!("We sent this : {:?}\n", result);

                    println!("****************************-END_REQUEST\n");
                    
                    return Some(result)

                },
                (_, _) => {
                    println!("We sent an empty list. Some of the deeper sequences were not available.\n");

                println!("****************************-END_REQUEST\n");
            
                    return None
                }
            }
         }
         Some(s) if *s.name == ("Product".to_owned() + server.keyword).to_string() => {
            println!("****************************-BEGIN_REQUEST\n");
            println!("Got a POST {} request. This sequence is available on this server with the requested signature. Returning the desired range.\n", r);
            
            
            let range = request.range;
            
            
            let vec1 = get_sequence(&request, 0, range, error_message, my_url).await;
            let vec2 = get_sequence(&request, 1, range, error_message, my_url).await;
             
            match (vec1, vec2) {
                (Some(v1), Some(v2)) => {
                    let result = pointwise_multiply(v1, v2);
                    println!("We sent this : {:?}\n", result);

                    println!("****************************-END_REQUEST\n");
                    
                    return Some(result)

                },
                (_, _) => {
                    println!("We sent an empty list. Some of the deeper sequences were not available.\n");

                println!("****************************-END_REQUEST\n");
            
                    return None
                }
            }
         }
         Some(s) if *s.name == ("Drop".to_owned() + server.keyword).to_string() => {
            println!("****************************-BEGIN_REQUEST\n");
            println!("Got a POST {} request. This sequence is available on this server with the requested signature. Returning the desired range.\n", r);
            
            
            let range = request.range;
            let shift = &request.parameters[0];
            
            if shift.is_sign_negative() {
                panic!("Cannot convert a negative float to u64")
            } else if *shift > (u64::MAX as f64) {
                panic!("Cannot convert a float larger than u64::MAX to u64");
            } else {
                let shift: u64 = *shift as u64;
                let new_from = range.from + shift;
                let new_to = range.to + shift;
                let range = Range {
                    from : new_from, 
                    to : new_to, 
                    step : range.step
                };

                let vec1 = get_sequence(&request, 0, range, error_message, my_url).await;
            
                match &vec1 {
                    Some(v) => {
                        println!("We sent this : {:?}\n", v);

                    }
                    None => println!("Didnt send anything.\n")
                }
                
                println!("****************************-END_REQUEST\n");
                    
                vec1
            }
         }
         Some(s) if *s.name == ("NthRootSequence".to_owned() + server.keyword).to_string() => {
            println!("****************************-BEGIN_REQUEST\n");
            println!("Got a POST {} request. This sequence is available on this server with the requested signature. Returning the desired range.\n", r);
            
            let range = request.range;
            let vec1 = get_sequence(&request, 0, range, error_message, my_url).await;
            
            let mut complex_res = false;

            match vec1 {
                None => return None,
                Some(input) => {
                    let res = input.iter().enumerate().map(|(n, value)| {
                        let n = range.from + (n as u64) * range.step;
                        match (value, n) {
                            (_, 0) => None, //0-ti koren....
                            (Some(val), n) => {
                                
                                if *val < 0.0 && (n) % 2 == 0 {
                                    let err = "Even root of a negative value is not a real number. I am not being paid to implement complex sequences, so you're out of luck.".to_string();
                                    **error_message = err;
                                    complex_res = true;
                                }
                                
                                Some(val.powf(1.0 / (n) as f64))
                            },
                            (None,_) => None,
                        }
                    });
                    let mut result : Vec<Option<f64>> = Vec::new();
                    for x in res {
                        result.push(x)
                    }
                    if complex_res {
                        println!("You tried to calculate even root of a negative number. Returning an error.");
                        println!("****************************-END_REQUEST\n");
                        return None
                    } else {
                        println!("We sent this : {:?}\n", result);
                        println!("****************************-END_REQUEST\n");
                 

                        return Some(result)
                    }
                    
                }
            }

        
         }

         Some(s) if *s.name == ("Story".to_owned() + server.keyword).to_string()=> {
            println!("****************************-BEGIN_REQUEST\n");
            println!("Got a POST {} request. This sequence is available on this server with the requested signature. Returning the desired range.\n", r);
            
            let range = request.range;
            
            // Extract parameters: author and genre
            let author = request.parameters[0];
            let genre = request.parameters[1];
            let sequences = &request.sequences; // Borrow the sequences
            let api_key = get_groq_api_key();
            
            // Process the input sequence (e.g., narrative elements)
            let s1 = &request.sequences[0];
            let (path1, cond1, body1) = from_seq_syntax_to_recursive_call(s1.as_ref().clone(), range);
            let result1 = Box::pin(handle_post(error_message, &path1, cond1, my_url, body1, server, number));
            let vec1 = *Pin::into_inner(Box::pin(result1.await));
            
            match vec1 {
                Some(_v1) => {
                    // Use the new function to create a StorySequence
                    let story_sequence = sequence::story::StorySequence::new(sequences.to_vec(), author, genre).await;
            
                    println!("Generated story: {:?}\n", story_sequence.story);
                    println!("Encoded story in binary: {:?}\n", story_sequence.story);
            
                    println!("****************************-END_REQUEST\n");
                    
                    return Some(story_sequence.range(range));
                },
                None => {
                    println!("We sent an empty list. The sequence was not available.\n");
            
                    println!("****************************-END_REQUEST\n");
                    
                    return None;
                }
            }
        }
        
        Some(s) if *s.name == ("Ai".to_owned() + server.keyword).to_string() => {
            println!("****************************-BEGIN_REQUEST\n");
            println!("Got a POST {} request. This sequence is available on this server with the requested signature. Returning the desired range.\n", r);
        
            
            let param4: usize = request.parameters[0].round() as usize;
        
            // Handle Ai sequence logic here
            let range = request.range;
            let seq_param = &request.sequences[0]; // Assuming this is correct, adjust if needed
            let (path1, cond1, body1) = from_seq_syntax_to_recursive_call(*seq_param.clone(), range);
            let result1 = Box::pin(handle_post(error_message, &path1, cond1, my_url, body1, server, number));
            let vec1 = *Pin::into_inner(Box::pin(result1.await));
        
            match vec1 {
                Some(input_sequence) => {
                    // Convert `input_sequence` to `Vec<Option<f64>>` if possible
                    // Assuming input_sequence is directly a Vec<Option<f64>>, adjust if needed
                    let input_sequence_vec: Vec<Option<f64>> = input_sequence;
        
                    // Create the `Ai` instance with the number of predictions
                    let ai_seq = sequence::ai::Ai::new(input_sequence_vec, param4);
        
                    // No range method in Ai, just use the predictions
                    let result = ai_seq.prediction;
        
                    println!("We sent this : {:?}\n", result);
                    println!("****************************-END_REQUEST\n");
                    Some(result)
                },
                None => {
                    println!("We sent an empty list. The sequence was not available.\n");
                    println!("****************************-END_REQUEST\n");
                    None
                }
            }
        }
        
        
        
        
        
        
        
        
        

        _ => panic!("Not implemented"),
    }
}


pub fn ip_string_to_vec(ip: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    ip.split('.')
        .map(|s| s.parse::<u8>())
        .collect()
}

pub fn vec_to_array(vec: Vec<u8>) -> Result<[u8; 4], &'static str> {
    if vec.len() < 4 {
        return Err("Vector has fewer than 4 elements");
    }

    let array: [u8; 4] = [vec[0], vec[1], vec[2], vec[3]];
    Ok(array)
}

pub fn ip_vec_to_ip(vec: Vec<u8>) -> String {
    vec.iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(".")
}


#[cfg(test)]
mod tests {

}
