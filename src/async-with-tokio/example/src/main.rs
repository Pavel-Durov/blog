

#[tokio::main]
async fn main() {
    tokio::spawn(tasks()).await.unwrap();    
}

async fn tasks(){
    let blocking = tokio::task::spawn_blocking(blocking_task);
    let mut async_h = Vec::new();
    for id in 0..10 {
        async_h.push(tokio::spawn(async_task(id)));
    }
    for h in async_h {
        println!("async: {}", h.await.unwrap());
    }
    println!("blocking: {}", blocking.await.unwrap());
}
fn blocking_task() -> String {
    std::thread::sleep(std::time::Duration::from_secs(5));
    "Done working...".to_string()
}

async fn async_task(id: i32) -> String {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    format!("Async Call. ID: {}!", id)
}