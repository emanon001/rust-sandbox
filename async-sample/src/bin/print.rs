use futures;
use tokio;

async fn handle(i: usize) -> usize {
    println!("{}", i);
    i
}

#[tokio::main]
async fn main() {
    let mut handlers = Vec::new();
    for i in 1..10 {
        let handler = tokio::spawn(handle(i));
        handlers.push(handler);
    }

    let res = futures::future::join_all(handlers).await;
    let res = res.into_iter().map(|r| r.unwrap()).collect::<Vec<_>>();
    println!("{:?}", res);
}
