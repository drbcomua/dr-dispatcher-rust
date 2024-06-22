use crate::name::Name;
use crate::reactive_storage::ReactiveStorage;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;

pub fn name_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let storage = Arc::new(RwLock::new(ReactiveStorage::new()));

    let storage_filter = warp::any().map(move || storage.clone());

    warp::path("name")
        .and(warp::post())
        .and(warp::body::json())
        .and(storage_filter.clone())
        .and_then(handle_post_names)
        .or(warp::path("name")
            .and(warp::delete())
            .and(warp::body::json())
            .and(storage_filter)
            .and_then(handle_delete_names))
}

async fn handle_post_names(
    names: Vec<Name>,
    storage: Arc<RwLock<ReactiveStorage>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = storage.write().await.dispatch(names).await;
    Ok(warp::reply::json(&result))
}

async fn handle_delete_names(
    names: Vec<Name>,
    storage: Arc<RwLock<ReactiveStorage>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    storage.write().await.remove(names).await;
    Ok(warp::reply::with_status("", warp::http::StatusCode::OK))
}
