use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs::File;
use std::thread;
use std::time::Duration;
extern crate web;
use web::ThreadPool;

fn main() {
    let listener = TcpListener::bind("192.168.80.128:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("conection ested!");

        pool.execute(||{
            handle_connection(stream);
        });        

    }
   
}


fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;512];
    
    stream.read(&mut buffer).unwrap();


    let get = b"GET / HTTP/1.1\r\n";
    let sleep =b"GET /sleep HTTP/1.1\r\n";

    let (status_line,filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n","hello.html")
    }else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n","hello.html")
    }else{
       
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n","404.html")
    };
    
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //println!("Request:{}", String::from_utf8_lossy(&buffer[..]));

    
    let response = format!("{}{}",status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
}