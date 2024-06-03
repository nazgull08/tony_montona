use std::sync::Arc;
use ton_client::ClientContext;
use ton_client::net::ParamsOfQueryCollection;
use ton_client::ClientConfig;
use serde_json::json;

#[tokio::main]
async fn main() {
    // Создаем конфигурацию клиента с альтернативным сервером
    let config = ClientConfig {
        network: ton_client::net::NetworkConfig {
            server_address: Some("https://venom-testnet.evercloud.dev/d9636056bed3470aa1f4cf343451c9d3/graphql".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    let context = Arc::new(ClientContext::new(config).unwrap());

    // Адрес контракта, который нужно проверить
    let address = "UQC38O5AB3-eLN8EfwCZ53by3oychSb49ycWv9J_tiP-owpZ";

    // Параметры запроса для проверки баланса
    let query = ParamsOfQueryCollection {
        collection: "accounts".to_string(),
        filter: Some(json!({"id": { "eq": address }})),
        result: "balance".to_string(),
        order: None,
        limit: None,
    };

    // Выполняем запрос
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
