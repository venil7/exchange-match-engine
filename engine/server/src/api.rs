use anyhow::Result;
use domain::Opt;
use warp::{Filter, Rejection, Reply};

use crate::handlers;
use crate::routes;

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

    let create_order = routes::post_order(&opt).and_then(handlers::order_post_handler);
    let get_order = routes::get_order(&opt).and_then(handlers::order_get_handler);

    let order_api = create_order.or(get_order);

    Ok(order_api)
}
