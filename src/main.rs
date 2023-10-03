use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener_result = TcpListener::bind("127.0.0.1:7878");

    let listener = match listener_result {
        Ok(result) => result,
        Err(error) => panic!("Failed to bind to port 7878. Is something else using that port?"),
    };

    for stream_result in listener.incoming() {
        let stream = match stream_result {
            Ok(result) => result,
            Err(error) => panic!("Connection failed {:?}", error),
        };

        handle_connection(stream);
        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| match result {
            Ok(r) => r,
            Err(error) => panic!("Failed to decode http request: {:?}", error),
        })
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line = http_request.first().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
