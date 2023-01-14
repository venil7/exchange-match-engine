use domain::OrderRequest;
use engine::OrderBookProvider;
use uuid::Uuid;

pub async fn order_post_handler(
    order_request: OrderRequest,
    provider: Option<impl OrderBookProvider>,
) -> Result<impl warp::Reply, warp::Rejection> {
    provider
        .ok_or_else(|| anyhow::anyhow!("aa"))
        .and_then(|mut p| p.enqueue(order_request))
        .map_err(|_| warp::reject::reject())
        .map(|order| warp::reply::json(&order))
}

pub async fn order_get_handler(
    id: Uuid,
    provider: Option<impl OrderBookProvider>,
) -> Result<impl warp::Reply, warp::Rejection> {
    provider
        .ok_or_else(|| anyhow::anyhow!("aa"))
        .and_then(|mut p| p.get_order(&id))
        .map_err(|_| warp::reject::reject())
        .map(|order| warp::reply::json(&order))
}
