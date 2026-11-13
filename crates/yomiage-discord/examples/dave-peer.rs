//! JSON-lines peer for .github/scripts/check-dave-interop.py. Test data only.
use std::io::{self, BufRead, Write};
use std::num::NonZeroU16;

use davey::{DaveSession, MediaType, ProposalsOperationType};
use serde_json::{Value, json};

#[path = "../tests/support/dave_gateway.rs"]
mod dave_gateway;

fn decode(request: &Value, field: &str) -> Vec<u8> {
    hex::decode(request[field].as_str().unwrap()).unwrap()
}

fn main() {
    let user = std::env::args().nth(1).unwrap().parse().unwrap();
    let mut session =
        DaveSession::new(NonZeroU16::new(1).unwrap(), user, 1_234_567_890, None).unwrap();
    let gateway = dave_gateway::Gateway::new(1_234_567_890);
    for line in io::stdin().lock().lines() {
        let request: Value = serde_json::from_str(&line.unwrap()).unwrap();
        let response = match request["command"].as_str().unwrap() {
            "gateway" => json!({"data": hex::encode(gateway.external_sender())}),
            "external" => {
                session.set_external_sender(&decode(&request, "data")).unwrap();
                json!({})
            }
            "keypackage" => json!({"data": hex::encode(session.create_key_package().unwrap())}),
            "propose" => json!({"data": hex::encode(gateway.propose_add(
                request["epoch"].as_u64().unwrap(), &decode(&request, "data")))}),
            "proposals" => {
                let users: Vec<u64> = serde_json::from_value(request["users"].clone()).unwrap();
                let change = session
                    .process_proposals(
                        ProposalsOperationType::APPEND,
                        &decode(&request, "data"),
                        Some(&users),
                    )
                    .unwrap()
                    .unwrap();
                json!({"commit": hex::encode(change.commit), "welcome": hex::encode(change.welcome.unwrap())})
            }
            "commit" => {
                session.process_commit(&decode(&request, "data")).unwrap();
                json!({})
            }
            "welcome" => {
                session.process_welcome(&decode(&request, "data")).unwrap();
                json!({})
            }
            "auth" => {
                json!({"data": hex::encode(session.get_epoch_authenticator().unwrap().as_slice())})
            }
            "encrypt" => {
                json!({"data": hex::encode(session.encrypt_opus(&decode(&request, "data")).unwrap())})
            }
            "decrypt" => {
                match session.decrypt(
                    request["user"].as_u64().unwrap(),
                    MediaType::AUDIO,
                    &decode(&request, "data"),
                ) {
                    Ok(data) => json!({"data": hex::encode(data)}),
                    Err(_) => json!({"error": "rejected"}),
                }
            }
            _ => panic!("unknown test command"),
        };
        println!("{response}");
        io::stdout().flush().unwrap();
    }
}
