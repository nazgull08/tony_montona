use std::sync::Arc;
use ton_client::ClientContext;
use ton_client::net::ParamsOfQueryCollection;
use ton_client::ClientConfig;
use serde_json::json;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        network: ton_client::net::NetworkConfig {
            server_address: Some("https://venom-testnet.evercloud.dev/d9636056bed3470aa1f4cf343451c9d3/graphql".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    let context = Arc::new(ClientContext::new(config).unwrap());

    let address = "qwe";

    let query = ParamsOfQueryCollection {
        collection: "accounts".to_string(),
        filter: Some(json!({"id": { "eq": address }})),
        result: "balance".to_string(),
        order: None,
        limit: None,
    };

    match ton_client::net::query_collection(context.clone(), query).await {
        Ok(result) => {
            if let Some(account) = result.result.first() {
                let balance = account["balance"].as_str().unwrap();
                println!("Balance: {}", balance);
            } else {
                println!("Account not found");
            }
        }
        Err(e) => {
            println!("Error occurred: {:?}", e);
        }
    }
}
