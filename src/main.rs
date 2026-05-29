use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{
        Arc,
        mpsc::{self, Receiver, Sender},
    },
    thread,
    time::{Duration, Instant},
};

use intragram::ThreadPool;

const ADDR: &str = "0.0.0.0:6821";
const MSG: usize = 64;
const RATE_LIMIT: Duration = Duration::from_secs(2);

enum Message {
    ClientConnected(Arc<TcpStream>),
    ClientDisconnected(SocketAddr),
    NewMessage(SocketAddr, Vec<u8>),
}

#[derive(Debug)]
struct Client {
    connection: Arc<TcpStream>,
    last_message: Instant,
}

fn handle_clients(stream: Arc<TcpStream>, sender: Sender<Message>) -> Result<(), ()> {
    let mut buff = vec![0; MSG];

    sender
        .send(Message::ClientConnected(stream.clone()))
        .map_err(|_| eprintln!("Error Printing stream"))?;

    println!("Client Connected");

    loop {
        match stream.as_ref().read(&mut buff) {
            Ok(0) => {
                if let Ok(socket_addr) = stream.peer_addr() {
                    sender
                        .send(Message::ClientDisconnected(socket_addr))
                        .map_err(|e| eprintln!("Error disconnecting Client: {e:#?}"))?;
                    break;
                }
            }
            Ok(n) => {
                let msg = buff[0..n].to_vec();
                let response = String::from_utf8(msg.clone())
                    .map_err(|e| eprintln!("Error reading Client: {e:#?}"))?;
                println!("Response is {response:?}");

                if let Ok(socket_addr) = stream.peer_addr() {
                    println!("{}: {:?}", ADDR, msg);
                    sender
                        .send(Message::NewMessage(socket_addr, msg))
                        .map_err(|e| eprintln!("Failed to send message to client: {e:#?}"))?;
                }
            }

            Err(err) => {
                eprintln!("Error with client {err:?}");
                break;
            }
        }
    }
    Ok(())
}

fn server(receiver: Receiver<Message>) -> Result<(), ()> {
    let mut clients: HashMap<SocketAddr, Client> = HashMap::new();

    loop {
        let msg = receiver
            .recv()
            .map_err(|err| eprintln!("Error with reading from recieiver {err:?}"))?;

        match msg {
            Message::ClientConnected(client_stream) => {
                let addr = match client_stream.peer_addr() {
                    Ok(address) => address,
                    Err(_) => {
                        eprintln!(
                            "ERROR: Server received ClientConnected but couldn't get address."
                        );
                        let _ = client_stream.shutdown(std::net::Shutdown::Both);
                        continue;
                    }
                };
                clients.insert(
                    addr,
                    Client {
                        connection: client_stream,
                        last_message: Instant::now(),
                    },
                );
            }

            Message::ClientDisconnected(socket_addr) => {
                println!("DISCONNECT received for: {}", socket_addr);
                clients.remove(&socket_addr);

                for (recipients_addr, client) in clients.iter() {
                    println!("Sending disconnect notification to: {}", recipients_addr);
                    if *recipients_addr != socket_addr {
                        let _ = client
                            .connection
                            .as_ref()
                            .write(format!("Address {socket_addr} Disconnected \n").as_bytes())
                            .map_err(|err| eprintln!("Error writing to recipients: {err}"));
                    }
                }
            }

            Message::NewMessage(socket_addr, client_stream) => {
                let mut should_send = true;

                if let Some(client) = clients.get_mut(&socket_addr) {
                    let time_since_last_message =
                        Instant::now().duration_since(client.last_message);

                    if time_since_last_message < RATE_LIMIT {
                        eprintln!("Client {client:#?} has to wait to send");
                        should_send = false;
                    }
                    client.last_message = Instant::now();
                } else {
                    eprintln!("ERROR Client is not in our clients list, weird");
                    should_send = false;
                }

                if should_send {
                    for (recipients_addr, client) in clients.iter() {
                        let message = String::from_utf8(client_stream.clone()).unwrap();
                        if *recipients_addr != socket_addr {
                            let _ = client
                                .connection
                                .as_ref()
                                .write(&format!("Address: {socket_addr} - {message}").as_bytes())
                                .map_err(|err| eprintln!("Error writing to stuff: {err}"));
                        }
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), ()> {
    let listener = TcpListener::bind(ADDR)
        .map_err(|err| eprintln!("Error is {err:?}"))
        .expect("Error connecting");

    println!("Listening on Server {ADDR}");

    // let mut clients: Vec<_> = vec![];

    let (sender, reciever) = mpsc::channel::<Message>();
    let pool = ThreadPool::new(10);

    thread::spawn(|| server(reciever));

    for stream in listener.incoming() {
        match stream {
            Ok(streams) => {
                println!("Connection Established");
                // clients.push(streams.try_clone().expect("Error cloning stream"));
                let sender_clone = sender.clone();
                let new_stream = Arc::new(streams);
                pool.execute(|| {
                    handle_clients(new_stream, sender_clone)
                        .map_err(|_| eprintln!("Error handling clients"))
                        .ok();
                })?;
            }
            Err(e) => {
                eprintln!("closing connection with: {} with ERROR : {e:?}", ADDR);
                break;
            }
        }
    }
    Ok(())
}
