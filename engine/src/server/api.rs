use crate::domain::{sell_order, Opt, Order, OrderRequest};
use anyhow::Result;
use warp::{Filter, Rejection, Reply};

pub fn create_api(
    opt: &Opt,
) -> Result<impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone> {
    let ticker = opt.ticker.clone();

    // POST: /api/order/{ticker} -> body OrderRequest JSON
    let create_order = warp::path("order")
        .and(warp::path(ticker.clone()))
        .and(warp::post())
        .and(warp::body::content_length_limit(1024))
        .and(warp::body::json::<OrderRequest>())
        .and_then(|order_request| async move {
            let order = Order::from(order_request);
            Ok::<_, Rejection>(warp::reply::json(&order))
        });

    // GET: /api/order/{ticker}/:id -> body OrderRequest JSON
    let get_order = warp::path("order")
        .and(warp::path(ticker.clone()))
        .and(warp::path::param::<uuid::Uuid>())
        .and(warp::get())
        .and_then(|id| async move {
            let order = Order {
                id,
                ..sell_order(1, 3)
            };
            Ok::<_, Rejection>(warp::reply::json(&order))
        });

    let order_api = warp::path("api").and(create_order.or(get_order));

    Ok(order_api)
}
