use std::net::TcpListener;

const ADDR: &str = "127.0.0.1:6821";
const MSG: u32 = 32;

fn main() {
    let listener = TcpListener::bind(ADDR).expect("Listener failed to start");
    listener
        .set_nonblocking(true)
        .expect("Error setting serer to non-blocking");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection Established");
                println!("{:?}", stream)
            }
            Err(_e) => {}
        }
    }
}
