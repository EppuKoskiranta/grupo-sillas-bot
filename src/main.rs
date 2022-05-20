use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};

mod auth;
mod discord_bot_types;
mod lol;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    lol::get_game_ids;
    let (event, _context) = event.into_parts();

    println!("{}",event.to_string());
    let event_body = event["body"].as_str();
    let authorized_request = auth::verify_request(&event);

    if authorized_request {

        match event_body {
            Some(x) => {
                return Ok(handle_request(x));
            }
            None => {
                return Ok(make_error_response(400, "No body"))
            }
        }

    } else {
        println!("Not authorized :o");
        return Ok(make_error_response(401, "invalid request signature"));
    }
}

fn process_request(event_body: &str) -> Result<discord_bot_types::BotResponse, discord_bot_types::BotError> {
    panic!("ahh...")
}

fn handle_request(event_body: &str) -> Value {
    println!("body: {}", event_body);
    let payload: Result<Value, _> = serde_json::from_str(event_body);

    match payload {
        Ok(body) => {
            let interaction_type = body["type"].as_i64();
            let command_data = body["data"];

            if interaction_type == Some(1) {
                let ping_response: Value = json!(
                    { 
                        "statusCode": 200,
                        "headers": {
                            "Content-Type": "application/json"
                        },
                        "body": json!({
                            "type": 1
                        }).to_string()
                    });

                return ping_response;
            } else if interaction_type == Some(2) {
                println!("Trying to respond to a command interaction");

                let response = create_response(command_data);

                match response {
                    Err(err) => {
                        make_error_response(400, format!("Unrecognised command input payload, err: {}", err.to_string() ))
                    },
                    Ok(response) => {
                        let x = json!(
                            { 
                                "statusCode": 200,
                                "headers": {
                                    "Content-Type": "application/json"
                                },
                                "body": json!({
                                    "type": 4,
                                    "data": {
                                        "tts": false,
                                        "content": response
                                    }
                                }).to_string()
                            });
            
                        println!("{}", x.to_string());
                        return x;
                    }
                }


            } else {
                return make_error_response(400, "Unrecognised request type");
            }


        }
        Err(_) => {
            println!("Not a JSON payload");
            return make_error_response(400, "Not a JSON payload");
        }
        
    }
}
    
fn create_response(command_data: Value) -> Result<String, serde_json::Error> {
    let command: discord_types::Command = serde_json::from_value(command_data)?;

    panic!("oh noes");
}

fn make_error_response(error_code: u64, description: &str) -> serde_json::Value {
    json!({
        "statusCode": error_code,
        "body": description
    })
}

