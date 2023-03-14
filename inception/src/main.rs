use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
};

fn main() {
    //create tcp listner on port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        //println!("Hello, world Connection established");
    }
    
}


//handle a connection stream
fn handle_connection(mut stream: TcpStream) {

    //Reading from tcpstream and printing data
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
        
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("inception.html").unwrap();
    let length = contents.len();
    let content_type = "text/html; charset=utf-8";

    let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();    
    println!("Request: {:#?}", http_request);
}