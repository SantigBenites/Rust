
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    env
};

use regex::Regex;


fn main(){


    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let path = env::current_dir();
    if path.is_ok(){
        println!("The current directory is {}", path.unwrap().display());
    }

    for stream in listener.incoming() {
        let stream = stream.unwrap();   
        handle_connection(stream);
    }


}

fn handle_connection(mut stream: TcpStream) {

    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    
    let re = Regex::new(r"GET / HTTP/1.1").unwrap();

    let (status_line, filename) = if re.is_match(&request_line) {

        let fileregex = Regex::new(r"/(.*) ").unwrap();
        let filename : String= fileregex.find_iter(&request_line).map(|mat| mat.as_str()).collect();
        let finalname:&'static str = str::new(filename.as_str());
        ("HTTP/1.1 200 OK", finalname)

    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}