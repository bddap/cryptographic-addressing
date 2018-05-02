extern crate sslhash;
extern crate openssl;

use sslhash::AcceptorBuilder;
use openssl::ssl::{SslStream, SslAcceptor, HandshakeError, SslMethod, SslConnector};
use std::io;
use std::io::{Write, Read};
use std::string::String;
use std::net::{TcpStream, TcpListener};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use std::collections::HashMap;

struct Matchmaker {
    listener: CryptoListener,
    addrs: HashMap<String, std::net::SocketAddr>
}

impl Matchmaker {
    fn new(tcp: TcpListener) -> Matchmaker {
        Matchmaker {
            listener: CryptoListener::new(tcp),
            addrs: HashMap::new()
        }
    }

    fn serve_one(&self) {
        match self.listener.accept() {
            Ok((stream, address)) => {
                self.recieve(stream, address);
            },
            _ => {}
        }
    }

    fn recieve(&self, mut stream: SslStream<TcpStream>, address: SocketAddr) {
        // read and parse as json
        let mut recieved = Vec::new();
        stream.read_to_end(&mut recieved).unwrap();
        
        // call either publish, or lookup
    }

    fn publish(&self, stream: SslStream<TcpStream>) {}
}

struct CryptoListener {
    listener: TcpListener,
    acceptor: SslAcceptor,
    hash: String
}

impl CryptoListener {
    fn new(tcp: TcpListener) -> CryptoListener {
        let (acceptor, hash) = AcceptorBuilder::default()
            .set_cache_dir(None)
            .build()
            .unwrap();
        CryptoListener {
            listener: tcp,
            acceptor: acceptor,
            hash: hash
        }
    }

    fn publish(&self, matchmaker_addr: SocketAddr, matchmaker_pub_hash: String) {
        let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
        let tcp = TcpStream::connect(matchmaker_addr).unwrap();
        let mut socket = sslhash::connect(&connector, tcp, matchmaker_pub_hash).unwrap();
        socket.write_all(b"Lettuce").unwrap();
    }
    
    fn accept(&self) -> io::Result<(SslStream<TcpStream>, std::net::SocketAddr)> {
        let (a, b) = self.listener.accept()?;
        if let Ok(d) = self.acceptor.accept(a) {
            Ok((d, b))
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "oh no!"))
        }
    }
}

pub fn connect(hash: String) -> SslStream<TcpStream> {
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
    let tcp = TcpStream::connect("localhost:1234").unwrap();
    sslhash::connect(&connector, tcp, hash).unwrap()
}

#[cfg(test)]
mod tests {
    use std::io::{Write, Read};
    use CryptoListener;
    use Matchmaker;
    use connect;
    use std::thread;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
    
    // #[test]
    // fn ssl() {
    //     // Securely Send Lettuce
    //     let listener = CryptoListener::new();
    //     let hash = listener.hash.clone();
    //     let client = thread::spawn(move || {
    //         connect(hash).write_all(b"Lettuce").unwrap();
    //     });
    //     let (mut c, _) = listener.accept().unwrap();
    //     let mut recieved = Vec::new();
    //     c.read_to_end(&mut recieved).unwrap();
    //     let lettuce: Vec<u8> = b"Lettuce".iter().map(|&a| a).collect();
    //     assert!(recieved == lettuce);
    //     client.join().unwrap();
    // }

    #[test]
    fn matchmaker_insert() {
        let matchmaker_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 2512);
        let matchmaker_tcp = TcpListener::bind(matchmaker_addr).unwrap();
        let matchmaker = Matchmaker::new(matchmaker_tcp); // spawn matchmaker; get address
        let matchmaker_pub_hash = matchmaker.listener.hash.clone();
        let matchmaker_thread = thread::spawn(move || {
            matchmaker.serve_one()
        });
        let listener_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
        let listener_tcp = TcpListener::bind(listener_addr).unwrap();
        let listener = CryptoListener::new(listener_tcp); // create keypair
        listener.publish(matchmaker_addr, matchmaker_pub_hash); // publish keypair to matchmaker
        // lookup keypair on matchmaker
        matchmaker_thread.join().unwrap();
    }
}

// Matchmaker:
// listens for publish. Stores publish.
// allows for address lookups

// Listener:
// Has keypair
// Publishes to Matchmaker

// Connector
// Looks up Listener address from Matchmaker
// Etablishes connection with Listener
