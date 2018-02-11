extern crate sslhash;

use sslhash::AcceptorBuilder;
use std::net::{TcpStream, TcpListener};

// Listen on tls for lookups.

fn main() {
    // Create a builder.
    // Default values:
    // - RSA bits: 3072
    // - Cache directory: The same directory as the executable
    let (acceptor, hash) = AcceptorBuilder::default()
        .set_cache_dir(None)
        .build()
        .unwrap();

    println!("{}", hash);
    
    // Replace "localhost:1234" with what you want to bind to.
    // On UNIX, use 0.0.0.0 as IP to make it public.
    let tcp = TcpListener::bind("localhost:1234").unwrap();
    let (client, _) = tcp.accept().unwrap();
    let mut client = acceptor.accept(client).unwrap();

    // client is a SslStream<TcpStream> now ready to be used.
    // Somehow transfer the hash to the client.
    // A simple way would be to tell the user to give this to all clients.
}
