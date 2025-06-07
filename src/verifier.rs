use num_bigint::BigUint;
use std::{collections::HashMap, sync::Mutex};
use tonic::{Code, Request, Response, Status, transport::Server};

use cp_protocol::Proof;

pub mod auth {
    include!("./auth.rs");
}

use auth::auth_server::{Auth, AuthServer};

use crate::auth::{
    AuthenticationAnswerResponse, AuthenticationChallengeResponse, RegisterResponse,
};

#[derive(Debug, Default)]
pub struct AuthImpl {
    pub user_info: Mutex<HashMap<String, UserInfo>>,
    pub auth_id_to_user: Mutex<HashMap<String, String>>,
}

#[derive(Debug, Default)]
pub struct UserInfo {
    // registration
    pub identifier: String,
    pub y1: BigUint,
    pub y2: BigUint,
    // authorization
    pub r1: BigUint,
    pub r2: BigUint,
    // verification
    pub c: BigUint,
    pub s: BigUint,
    pub session_id: String,
}

#[tonic::async_trait]
impl Auth for AuthImpl {
    async fn register(
        &self,
        request: Request<auth::RegisterRequest>,
    ) -> Result<Response<auth::RegisterResponse>, Status> {
        let request = request.into_inner();

        let identifier = request.identifier;

        let user_info = UserInfo {
            identifier: identifier.clone(),
            y1: BigUint::from_bytes_be(&request.y1),
            y2: BigUint::from_bytes_be(&request.y2),
            ..Default::default()
        };

        let user_info_hashmap = &mut self.user_info.lock().unwrap();
        user_info_hashmap.insert(identifier, user_info);

        Ok(Response::new(RegisterResponse {}))
    }

    async fn create_authentication_challenge(
        &self,
        request: Request<auth::AuthenticationChallengeRequest>,
    ) -> Result<Response<auth::AuthenticationChallengeResponse>, Status> {
        let request = request.into_inner();
        let identifier = request.identifier;

        let user_info_hashmap = &mut self.user_info.lock().unwrap();

        if let Some(user_info) = user_info_hashmap.get_mut(&identifier) {
            user_info.r1 = BigUint::from_bytes_be(&request.r1);
            user_info.r2 = BigUint::from_bytes_be(&request.r2);

            let (_, _, _, q) = Proof::get_constants();
            let c = Proof::generate_random_number_below(&q);
            user_info.c = c.clone();

            let auth_id = Proof::generate_random_string_below(12);
            let auth_id_to_user = &mut self.auth_id_to_user.lock().unwrap();
            auth_id_to_user.insert(auth_id.clone(), identifier);

            return Ok(Response::new(AuthenticationChallengeResponse {
                auth_id,
                c: c.to_bytes_be(),
            }));
        } else {
            return Err(Status::new(
                Code::NotFound,
                format!("User: {} not found", identifier),
            ));
        }
    }

    async fn verify_authentication(
        &self,
        request: Request<auth::AuthenticationAnswerRequest>,
    ) -> Result<Response<auth::AuthenticationAnswerResponse>, Status> {
        let request = request.into_inner();
        let auth_id = request.auth_id;

        let auth_id_to_user_hashmap = &mut self.auth_id_to_user.lock().unwrap();

        if let Some(identifier) = auth_id_to_user_hashmap.get(&auth_id) {
            let user_info_hashmap = &mut self.user_info.lock().unwrap();

            let user_info = user_info_hashmap
                .get(identifier)
                .expect("User info not found for identifier");

            let (p, q, alpha, beta) = Proof::get_constants();

            let proof = Proof::new(p, q, alpha, beta);

            let s = BigUint::from_bytes_be(&request.s);

            let verification = proof.verify(
                &user_info.r1,
                &user_info.r2,
                &user_info.y1,
                &user_info.y2,
                &user_info.c,
                &s,
            );

            if verification {
                let session_id = Proof::generate_random_string_below(12);
                return Ok(Response::new(AuthenticationAnswerResponse {
                    session_id: session_id.clone(),
                }));
            } else {
                return Err(Status::new(Code::Unauthenticated, "Verification failed"));
            }
        } else {
            return Err(Status::new(Code::NotFound, "Auth ID not found"));
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:50051".to_string();
    println!("Verifier listening on {}", addr);

    Server::builder()
        .add_service(AuthServer::new(AuthImpl::default()))
        .serve(addr.parse().expect("Could not parse address"))
        .await
        .unwrap();
}
