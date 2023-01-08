use domain::{Opt, OrderRequest};
use warp::{filters::BoxedFilter, path, Filter};

fn order_api_prefix(opt: &Opt) -> BoxedFilter<()> {
    path!("api" / "order" / ..)
        .and(warp::path(opt.ticker.clone()))
        .boxed()
}

// POST: /api/order/{ticker} -> body OrderRequest JSON
pub fn post_order(opt: &Opt) -> BoxedFilter<(OrderRequest,)> {
    warp::post()
        .and(order_api_prefix(opt))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024))
        .and(warp::body::json::<OrderRequest>())
        .boxed()
}

// GET: /api/order/{ticker}/:id -> body OrderRequest JSON
pub fn get_order(opt: &Opt) -> BoxedFilter<(uuid::Uuid,)> {
    warp::get()
        .and(order_api_prefix(opt))
        .and(warp::path::param::<uuid::Uuid>())
        .and(warp::path::end())
        .boxed()
}
