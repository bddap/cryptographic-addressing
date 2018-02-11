extern crate sslhash;
extern crate openssl;

use openssl::ssl::{SslConnector, SslMethod};
use sslhash::AcceptorBuilder;
use std::net::{TcpStream, TcpListener};
use std::env;

fn main() {
    if let Some(a) = env::args().nth(2) {
        client(a);
    }
}

fn client(hash: String) {
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

    // Replace "localhost:1234" with what you want to connect to.
    let client = TcpStream::connect("localhost:1234").unwrap();

    // Assumes you have a String called "hash" that is the hash of the server's public key.
    // Somehow receive this from the server.
    // A simple way would be to ask the user for the hash.
    let mut client = sslhash::connect(&connector, client, hash).unwrap();
}
