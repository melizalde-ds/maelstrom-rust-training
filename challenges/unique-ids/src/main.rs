use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::stdin;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: u64,
    pub src: String,
    pub dest: String,
    pub body: MessageBody,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageBody {
    #[serde(rename = "type")]
    pub message_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
}


fn main() {
    let mut hasher = DefaultHasher::new();
    let mut global_id = 0u64;
    let mut global_node_id = "0".to_string();
    let mut seed = 0u64;
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("Failed to read line");
        let trimmed = buffer.trim();
        if trimmed.is_empty() {
            break;
        }
        let message: Message = serde_json::from_str(trimmed).expect("Failed to parse JSON");
        eprintln!("Received: {}", trimmed);
        match message.body.message_type.as_str() {
            "init" => {
                if global_node_id == "0" {
                    global_node_id = message.body.node_id.clone().expect("No node id");
                }
                global_node_id.hash(&mut hasher);
                let salt = hasher.finish() as u32;
                let mut rng = StdRng::seed_from_u64(salt.into());
                seed = rng.random();
                let message = Message {
                    id: global_id,
                    src: global_node_id.clone(),
                    dest: message.src,
                    body: MessageBody {
                        message_type: "init_ok".to_string(),
                        node_id: Some("node_id".to_string()),
                        node_ids: None,
                        msg_id: None,
                        in_reply_to: message.body.msg_id,
                        echo: None,
                        id: None,
                    },
                };
                global_id += 1;
                println!("{}", serde_json::to_string(&message).unwrap());
                eprintln!("Sending: {}", serde_json::to_string(&message).unwrap());
            }
            "echo" => {
                let message = Message {
                    id: global_id,
                    src: global_node_id.clone(),
                    dest: message.src,
                    body: MessageBody {
                        message_type: "echo_ok".to_string(),
                        node_id: None,
                        node_ids: None,
                        msg_id: Some(message.body.msg_id.unwrap()),
                        in_reply_to: message.body.msg_id,
                        echo: message.body.echo.clone(),
                        id: None,
                    },
                };
                global_id += 1;
                println!("{}", serde_json::to_string(&message).unwrap());
                eprintln!("Sending: {}", serde_json::to_string(&message).unwrap());
            }
            "generate" => {
                let message = Message {
                    id: global_id,
                    src: global_node_id.clone(),
                    dest: message.src,
                    body: MessageBody {
                        message_type: "generate_ok".to_string(),
                        node_id: None,
                        node_ids: None,
                        msg_id: Some(message.body.msg_id.unwrap()),
                        in_reply_to: message.body.msg_id,
                        echo: None,
                        id: Some(global_id + seed),
                    },
                };
                global_id += 1;
                println!("{}", serde_json::to_string(&message).unwrap());
                eprintln!("Sending: {}", serde_json::to_string(&message).unwrap());
            }
            _ => {
                panic!("Unknown message type: {}", message.body.message_type);
            }
        }
    }
}