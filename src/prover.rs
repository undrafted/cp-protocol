use std::io::stdin;

pub mod auth {
    include!("./auth.rs");
}

use auth::auth_client::AuthClient;
use num_bigint::BigUint;

use crate::auth::RegisterRequest;

use cp_protocol::Proof;

#[tokio::main]
async fn main() {
    let mut buffer = String::new();
    let mut client = AuthClient::connect("http://127.0.0.1:50051")
        .await
        .expect("Could not connect to the verifier");

    println!("Connected to the verifier");

    stdin()
        .read_line(&mut buffer)
        .expect("Could not get the identifier from stdin");
    let identifier = buffer.trim().to_string();

    buffer.clear();

    println!("Please provide the password:");
    stdin()
        .read_line(&mut buffer)
        .expect("Could not get the password from stdin");
    let password = BigUint::from_bytes_be(buffer.trim().as_bytes());

    let (p, q, alpha, beta) = Proof::get_constants();
    let proof = Proof::new(p, q, alpha, beta);

    let [y1, y2] = proof.create_pair(&password);

    let request = RegisterRequest {
        identifier,
        y1: y1.to_bytes_be(),
        y2: y2.to_bytes_be(),
    };

    let _response = client
        .register(request)
        .await
        .expect("Failed to send registration request");

    println!("Registration request sent successfully");
}
