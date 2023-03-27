//fs : reading file 
//net : working with tcp listeners and streams
//io :  reading and writing to streams
use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
};

fn main() {

    //create tcp listner on port 7878
    //let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    //for each incoming connection attempt
    for stream in listener.incoming() {
        //TODO : handle error grancefully without using unwrap
        let stream = stream.unwrap();

        //call connection handler
        handle_connection(stream);
        
    }
    
}


//handle a connection stream
fn handle_connection(mut stream: TcpStream) {

    //Reading from tcpstream and printing data
    //Prints details of the entire message sent from the client
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
   
    println!("Request: {:#?}", http_request);    

    //Send response back to the client
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("inception.html").unwrap();
    let length = contents.len();
    let content_type = "text/html; charset=utf-8";
    let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();    

    //TODO:send an error responsne for all other paths passed by the client
    //TODO: monitor the resource utilization of the server
   
}