use std::io::stdin;

pub mod auth {
    include!("./auth.rs");
}

use auth::auth_client::AuthClient;
use num_bigint::BigUint;

use crate::auth::{AuthenticationChallengeRequest, RegisterRequest};

use cp_protocol::Proof;

#[tokio::main]
async fn main() {
    let mut buffer = String::new();
    let mut client = AuthClient::connect("http://127.0.0.1:50051")
        .await
        .expect("Could not connect to the verifier");

    println!("Connected to the verifier");
    println!("Please provide your identifier:");

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
    buffer.clear();

    let (p, q, alpha, beta) = Proof::get_constants();
    let proof = Proof::new(p, q, alpha, beta);

    let [y1, y2] = proof.create_pair(&password);

    let request = RegisterRequest {
        identifier: identifier.clone(),
        y1: y1.to_bytes_be(),
        y2: y2.to_bytes_be(),
    };

    let _response = client
        .register(request)
        .await
        .expect("Failed to send registration request");

    let k = proof.generate_random_number();
    let [r1, r2] = proof.create_pair(&k);

    let request = AuthenticationChallengeRequest {
        identifier: identifier.clone(),
        r1: r1.to_bytes_be(),
        r2: r2.to_bytes_be(),
    };

    let response = client
        .create_authentication_challenge(request)
        .await
        .expect("Failed to send authentication challenge request")
        .into_inner();

    let auth_id = response.auth_id;
    let c = BigUint::from_bytes_be(&response.c);

    println!("Please provide the password to log in:");
    stdin()
        .read_line(&mut buffer)
        .expect("Could not get the password from stdin");
    let password = BigUint::from_bytes_be(buffer.trim().as_bytes());

    let s = proof.solve(&k, &c, &password);

    let request = auth::AuthenticationAnswerRequest {
        auth_id,
        s: s.to_bytes_be(),
    };

    let response = client
        .verify_authentication(request)
        .await
        .expect("Failed to verify authentication")
        .into_inner();

    println!("Authenticated with session_id: {}", response.session_id);
}
