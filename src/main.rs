use std::time::Duration;

use tokio::time::timeout;
use tokio::{self};
use tonlib::address::TonAddress;
use tonlib::client::{TonClient, TonClientBuilder, TonClientInterface};
use tonlib::config::{MAINNET_CONFIG, TESTNET_CONFIG};
use log::LevelFilter;

#[tokio::main]
async fn main() {
    // Инициализация логирования
      env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Error)
        .init();
    
    // Указываем адрес, баланс которого нужно проверить
    let address_str = "EQDk2VTvn04SUKJrW7rXahzdF8_Qi6utb0wj43InCu9vdjrR";

    // Преобразование строки в TonAddress
    let address = match TonAddress::from_base64_url(address_str) {
        Ok(addr) => addr,
        Err(err) => {
            eprintln!("Invalid address: {}", err);
            return;
        },
    };

    // Создание клиента Ton
    TonClient::set_log_verbosity_level(2);
    println!("==================");
    println!("==================");


    let client = match TonClient::builder()
        .with_config(MAINNET_CONFIG)
        .with_pool_size(10)
        .with_keystore_dir(String::from("/tmp"))
        .build()
        .await {
            Ok(client) => client,
            Err(err) => {
                eprintln!("Failed to create TonClient: {}", err);
                return;
            },
        };

    // Получение состояния аккаунта
    match timeout(Duration::from_secs(10), client.get_raw_account_state(&address)).await {
        Ok(Ok(state)) => {
            println!("Account balance: {}", state.balance);
        },
        Ok(Err(err)) => {
            eprintln!("Failed to get account state: {}", err);
        },
        Err(_) => {
            eprintln!("Request timed out");
        },
    };
}
