use anyhow::Result;
use domain::{sell_order, Opt, Order, OrderRequest};
// use service::{OrdersOps, RedisProvider};
// use tokio::task;
use warp::{Filter, Rejection, Reply};

use crate::routes;

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

    let create_order = routes::post_order(&opt).and_then(handler);
    let get_order = routes::get_order(&opt).and_then(|id| async move {
        let order = Order {
            id,
            ..sell_order(1, 3)
        };
        Ok::<_, Rejection>(warp::reply::json(&order))
    });

    let order_api = create_order.or(get_order);

    Ok(order_api)
}
