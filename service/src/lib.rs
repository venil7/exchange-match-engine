pub mod exchange;

use exchange::{exchange_server::Exchange, OrderRequest};
use tonic::{Request, Response, Status};


#[derive(Debug, Default)]
pub struct ExchangeService {}

#[tonic::async_trait]
impl Exchange for ExchangeService {
    async fn add_order(
        &self,
        _request: Request<OrderRequest>,
    ) -> Result<Response<OrderRequest>, Status> {
        todo!()
    }
}
