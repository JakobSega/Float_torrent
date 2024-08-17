use std::net::SocketAddr;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Error;
use hyper::{Method, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use std::pin::Pin;
use std::string::String;
use std::vec::Vec;

use std::env;

use serde_json;

use auxiliary_functions::*;

const NUMBER: u8 = 1; // Change this to 1 or 2 as needed. 1 is the Imposter server and 2 is the Elves server.

static MY: Server = select_server(NUMBER);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Print the arguments
    println!("Arguments: {:?}", args);

    // Default values
    let mut my_ip = "127.0.0.1".to_string();  // Default IP
    let mut reg_ip = "127.0.0.1".to_string(); // Default register IP
    let mut my_port = 8080; // Default port

    // Determine if we are using dynamic port assignment
let use_dynamic_port = if args.len() == 4 && args[1] == "dynamic" {
    // Dynamic port selection
    println!("Delam dinamicno.");
    reg_ip = args[2].clone();
    my_ip = args[3].clone();
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    my_port = addr.port();
    true
} else if args.len() == 4 {
    // Custom configuration with specified port
    reg_ip = args[1].clone();
    my_ip = args[2].clone();
    let my_port = match string_to_u16(&args[3]) {
        Ok(p) => {
            if p < 0 || p > 65535 {
                eprintln!("Invalid port: {}", args[3]);
                return Ok(());
            }
            p
        }
        Err(_) => {
            eprintln!("Failed to parse port: {}", args[3]);
            return Ok(());
        }
    };
    false
} else if args.len() == 1 {
    // Default configuration
    let my_project = get_project(&MY);
    my_ip = my_project.ip.clone();
    reg_ip = "127.0.0.1".to_string();
    my_port = my_project.port.clone();
    false
} else {
    eprintln!("Invalid arguments passed.");
    eprintln!("Usage: cargo run -- [dynamic] IP_REGISTRAR IP_GENERATOR [PORT]");
    return Ok(());
};

    // Convert IP string to vector
    let ip_vec = ip_string_to_vec(&my_ip).unwrap_or_else(|_| {
        eprintln!("Failed to parse IP address: {}", my_ip);
        std::process::exit(1);
    });
    let (f1, f2, f3, f4) = (ip_vec[0], ip_vec[1], ip_vec[2], ip_vec[3]);

    let ip_array = vec_to_array(ip_vec).unwrap();
    println!("{:?}", ip_array);

    // Bind to the address
    let addr: SocketAddr = if use_dynamic_port {
        (ip_array, 0).into()
    } else {
        (ip_array, my_port).into()
    };

    // Bind to the address
    let listener = TcpListener::bind(addr).await?;
    let actual_addr = listener.local_addr()?;
    let actual_port = actual_addr.port();
    println!("Listening on http://{}:{}", actual_addr.ip(), actual_port);

    let my_project = get_project_new(my_ip.clone(), actual_port as u16, &MY);
    let reg_link = format!("http://{}:7878", reg_ip);

    // Register with the register server
    let b = send_get(format!("{}/project", reg_link)).await?;
    println!("HERE {}", b);

    let _my_url = format!("http://{}:{}", my_project.ip, my_project.port);
    let b = send_post(
        format!("{}/project", reg_link),
        serde_json::to_string(&my_project).unwrap(),
    ).await?;
    println!("HERE {}", b);

    let b = send_get(format!("{}/project", reg_link)).await?;
    println!("HERE {}", b);

    let create_404 = || {
        let mut not_found = Response::new(empty());
        *not_found.status_mut() = StatusCode::NOT_FOUND;
        Ok(not_found)
    };

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::spawn(async move {
            // Use the captured `f1`, `f2`, `f3`, `f4` values inside async block
            let f1 = f1;
            let f2 = f2;
            let f3 = f3;
            let f4 = f4;

            let service = service_fn(move |req| {
                async move {
                    println!("Heard something!");
                    let method = req.method().clone();
                    let path = req.uri().path().to_owned();

                    // Helps us return a sequence when the sequence is not on our server...
                    let condition = false;

                    match (method, path.as_str()) {
                        (Method::GET, "/ping") => {
                            println!("****************************-BEGIN_REQUEST\n");
                            println!("Got a GET /ping request. Sending info about my project.\n");
                            println!("****************************-END_REQUEST\n");
                            Ok::<_, Error>(Response::new(full(
                                serde_json::to_string(&get_project(&MY)).unwrap(),
                            )))
                        },
                        (Method::GET, "/sequence") => {
                            println!("****************************-BEGIN_REQUEST\n");
                            println!("Got a GET /sequence request. Sending a list of my sequences.\n");
                            println!("****************************-END_REQUEST\n");
                            let sequences = sequences(&MY, NUMBER);
                            Ok(Response::new(full(
                                serde_json::to_string(&sequences).unwrap(),
                            )))
                        }
                        (Method::POST, r) => {
                            let body = collect_body(req).await.unwrap_or_else(|e| {
                                eprintln!("Failed to collect body: {}", e);
                                "".to_string()
                            });
                            let ip_vec_ = vec![f1, f2, f3, f4];
                            let my_ip_ = ip_vec_to_ip(ip_vec_);
                            let my_project_ = get_project_new(my_ip_, actual_port as u16, &MY);
                            let my_url_ = format!("http://{}:{}", my_project_.ip, my_project_.port);
                            let v = my_url_.as_str();
                            let mut error_message = Box::new("Error".to_string());
                            let x = handle_post(&mut error_message, r, condition, v, body, &MY, NUMBER).await;
                            let y = Pin::into_inner(Box::pin(x));

                            match *y {
                                None => {
                                    let create_404 = || {
                                        let mut not_found = Response::new(full((**error_message).to_string()));
                                        *not_found.status_mut() = StatusCode::NOT_FOUND;
                                        Ok(not_found)
                                    };
                                    create_404()
                                },
                                Some(v) => {
                                    Ok(Response::new(full(
                                        serde_json::to_string(&v).unwrap(),
                                    )))
                                }
                            }
                        }
                        _ => create_404(),
                    }
                }
            });

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
