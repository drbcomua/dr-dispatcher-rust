mod name;
mod name_controller;
mod reactive_storage;

#[tokio::main]
async fn main() {
    let api = name_controller::name_routes();

    warp::serve(api).run(([0, 0, 0, 0], 8080)).await;
}
