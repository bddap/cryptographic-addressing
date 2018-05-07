extern crate sslhash;
extern crate openssl;

use sslhash::{AcceptorBuilder, sha256};
use openssl::ssl::{SslStream, SslAcceptor, SslMethod, SslConnector};
use std::io;
use std::io::{Write, Read};
use std::string::String;
use std::net::{TcpStream, TcpListener};
use std::net::{SocketAddr};

use std::collections::HashMap;

const PUBLISH: u8 = 'p' as u8;
const LOOKUP: u8 = 'l' as u8;

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
        if recieved.starts_with(&[PUBLISH]) {
            self.publish(stream, address);
        } else if  recieved.starts_with(&[LOOKUP]) {

        }
        // match action {
        //     Some(Ok(PUBLISH)) => {},
        //     Some(Ok(LOOKUP)) => {},
        //     _ => {}
        // }
            
        // call either publish, or lookup
    }

    fn publish(&self, stream: SslStream<TcpStream>, address: SocketAddr) {
        // get public key from stream
        let pkhash = hash_of(stream);
        println!("{:?}", pkhash);
        // get address from stream
        // add both to self.addrs
        assert!(false); // not done
    }
}

fn hash_of(stream: SslStream<TcpStream>) -> String {
    println!("attempting to get peer cert");
    if let Some(cert) = stream.ssl().peer_certificate() {
        println!("got peer cert");
        if let Ok(pkey) = cert.public_key() {
            println!("got pk");
            if let Ok(pem) = pkey.public_key_to_pem() {
                println!("got pem");
                let hash = sha256(&pem);
                return hash;
            }
        } 
    }
    return "todo".to_string();
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
        socket.write_all(b"p").unwrap();
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
