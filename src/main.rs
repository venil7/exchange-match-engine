use bytes::{Buf, Bytes};
use exchange_service::{exchange::{exchange_server::ExchangeServer, OrderRequest}, ExchangeService};
// use tonic::transport::Server;
use prost::Message;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //https://www.thorsten-hans.com/grpc-services-in-rust-with-tonic/
    // let address = "[::1]:8080".parse().unwrap();
    // let exchange_service = ExchangeService::default();

    // Server::builder()
    //     .add_service(ExchangeServer::new(exchange_service))
    //     .serve(address)
    //     .await?;


    json_test().await?;

    Ok(())
}


async fn json_test() -> Result<(), Box<dyn std::error::Error>> {

    let data = r#"
    {
        "type": 0,
        "direction": 0,
        "amount" : 1.23
    }
    "#;

    let mut order_request: OrderRequest = serde_json::from_str(data)?;

    let mut buff: Vec<u8> = Vec::new();

    order_request.encode(&mut buff)?;

    order_request = OrderRequest::decode(Bytes::from(buff))?;

    let str = serde_json::to_string_pretty(&order_request)?;

    print!("{json}", json=str);
    Ok(())
}