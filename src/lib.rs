extern crate sslhash;
extern crate openssl;

use sslhash::AcceptorBuilder;
use openssl::ssl::{SslStream, SslAcceptor, HandshakeError, SslMethod, SslConnector};
use std::io;
use std::string::String;
use std::net::{TcpStream, TcpListener, SocketAddr};

struct CryptoListener {
    listener: TcpListener,
    acceptor: SslAcceptor,
    hash: String
}

impl CryptoListener {
    fn new() -> CryptoListener {
        let (acceptor, hash) = AcceptorBuilder::default()
            .set_cache_dir(None)
            .build()
            .unwrap();
        let tcp = TcpListener::bind("0.0.0.0:1234").unwrap();
        CryptoListener {
            listener: tcp,
            acceptor: acceptor,
            hash: hash
        }
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
    use connect;
    use std::thread;
    
    #[test]
    fn ssl() {
        // Securely Send Lettuce
        let listener = CryptoListener::new();
        let hash = listener.hash.clone();
        let client = thread::spawn(move || {
            connect(hash).write_all(b"Lettuce").unwrap();
        });
        let (mut c, _) = listener.accept().unwrap();
        let mut recieved = Vec::new();
        c.read_to_end(&mut recieved).unwrap();
        let lettuce: Vec<u8> = b"Lettuce".iter().map(|&a| a).collect();
        assert!(recieved == lettuce);
        client.join().unwrap();
    }
}
