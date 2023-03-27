
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    env,
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
    println!("Request line is {request_line}");

    
    let get_regex = Regex::new(r"GET (.*) HTTP/1.1").unwrap();
    let put_regex = Regex::new(r"PUT (.*) HTTP/1.1").unwrap();
    if get_regex.is_match(&request_line){
        get_handler(stream, &request_line);
    }

    if put_regex.is_match(&request_line){

    }

}

fn get_handler(mut stream: TcpStream,request_line:&String){

    let (status_line, filename) = {


        let fileregex = Regex::new(r"/(.*) ").unwrap();
        let temp = fileregex.find(&request_line).map(|mat| mat.as_str());
        
        match temp {
            Some(str) => ("HTTP/1.1 200 OK", str),
            None =>  ("HTTP/1.1 404 NOT FOUND", "404.html")
        }
        

    };

    // Combining filename and "." so that rust knows file location
    let temp = ".".to_owned() + filename;
    // Removing last space from temp, it messes with filenames
    let name = &&temp[0..temp.len() - 1];

    let content = match filename{
        "/ " => fs::read_to_string("./html/hello.html").unwrap(),
        "404.html" => fs::read_to_string("./html/404.html").unwrap(),
        _ => fs::read_to_string(name).unwrap()
    };

    let length = content.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}


fn put_handler(mut stream:TcpStream,request_line:&String){
    
}