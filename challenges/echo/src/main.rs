use serde::{Deserialize, Serialize};
use std::io::stdin;
use std::string::String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: i32,
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
    pub msg_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<String>,
}

fn main() {
    let mut global_id = 0;
    let mut global_node_id = "0".to_string();
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
