use warp::{path, Filter};

#[tokio::main]
async fn main() {
    start_server().await;
}

async fn start_server() {
    let create = path!("create").map(|| "create");
    let read = path!("read" / u32).map(|id| format!("read: {}", id));
    let update = path!("update" / u32).map(|id| format!("update: {}", id));
    let delete = path!("delete" / u32).map(|id| format!("delete: {}", id));

    let routes = create.or(read).or(update).or(delete);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await
}
