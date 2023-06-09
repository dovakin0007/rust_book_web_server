use std::net::{TcpListener, TcpStream};
use std::io::{prelude::* , BufReader};
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        handle_connection(stream);

    }

    pub fn handle_connection(mut stream: TcpStream){
        let buf_reader = BufReader::new(&mut stream);
        // let http_response = buf_reader.lines().map(|result| result.unwrap())
        // .take_while(|line| !line.is_empty()).collect();
        let http_response = buf_reader.lines().next().unwrap().unwrap();
    
        println!("{:?}", http_response);
        
        let (status, file) = if http_response == "GET / HTTP/1.1"  {
            ("HTTP/1.1 200 OK", "index.html")
        }else{
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
        
        let contents = fs::read_to_string(&file).unwrap();
        let contents_len = contents.len();

        let response = format!("{status}\r\nContent-Length: {contents_len}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
     

    }
}