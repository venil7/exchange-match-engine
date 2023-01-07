use anyhow::Result;
use domain::{sell_order, Opt, Order, OrderRequest};
// use service::{OrdersOps, RedisProvider};
// use tokio::task;
use warp::{Filter, Rejection, Reply};

async fn handler(order_request: OrderRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let order = Order::from(order_request);
    Ok::<_, Rejection>(warp::reply::json(&order))
}

pub fn create_api(
    opt: &Opt,
) -> Result<impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone> {
    // let (sender, mut receiver) = bmrng::unbounded_channel::<OrdersOps, i32>();
    // let opt1 = opt.clone();
    // task::spawn(async move {
    //     let provider = RedisProvider::try_new(&opt1.ticker, &opt1.redis).await?;
    //     while let Ok((req, responder)) = receiver.recv().await {
    //         match req {
    //             _ => responder.respond(12)?,
    //         }
    //     }
    //     Ok::<_, anyhow::Error>(())
    // });

    // POST: /api/order/{ticker} -> body OrderRequest JSON
    // let create_order_sender = sender.clone();
    let create_order = warp::path("order")
        .and(warp::path(opt.ticker.clone()))
        .and(warp::post())
        .and(warp::body::content_length_limit(1024))
        .and(warp::body::json::<OrderRequest>())
        .and_then(handler);

    // GET: /api/order/{ticker}/:id -> body OrderRequest JSON
    let get_order = warp::path("order")
        .and(warp::path(opt.ticker.clone()))
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
