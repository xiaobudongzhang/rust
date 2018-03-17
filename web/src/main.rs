use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let listener = TcpListener::bind("192.168.80.128:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("conection ested!");

        handle_connection(stream);
    }
   
}


fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;512];
    
    stream.read(&mut buffer).unwrap();


    let mut file = File::open("hello.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("Request:{}", String::from_utf8_lossy(&buffer[..]));

    
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
