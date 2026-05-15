use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread::sleep,
    time::Duration,
};

use intragram::ThreadPool;

const ADDR: &str = "127.0.0.1:6821";
const MSG: u32 = 32;

fn handle_clients(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader
        .lines()
        .next()
        .expect("Error with request")
        .expect("Error with request");

    let (status_line, file_name) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream
        .write_all(response.as_bytes())
        .expect("Error sending response");

    // println!("Request: {request:#?}")
}

fn main() {
    let listener = TcpListener::bind(ADDR).expect("Listener failed to start");
    listener
        .set_nonblocking(true)
        .expect("Error setting serer to non-blocking");

    let pool = ThreadPool::new(2);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection Established");
                pool.execute(|| {
                    handle_clients(stream);
                });
            }
            Err(_e) => {}
        }
    }
}
