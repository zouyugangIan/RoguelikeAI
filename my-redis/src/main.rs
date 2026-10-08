use mini_redis::{Result, client};
use tokio::time::Duration;
//
// #[tokio::main]
//
// async fn main() -> Result<()> {
//     let mut client = client::connect("127.0.0.1:6379").await?;
//
//     client.set("hello", "world".into()).await?;
//
//     let result = client.get("hello").await?;
//
//     println!("got value from the server; result={:?}", result);
//
//     Ok(())
// }

async fn sleep_then_print(timer: i32) {
    println!("Start timer {}", timer);

    // std::thread::sleep(Duration::from_secs(4));
    tokio::time::sleep(Duration::from_secs(1)).await;

    println!("Timer {} done", timer);
}

#[tokio::main]
async fn main() {
    tokio::join!(
        sleep_then_print(1),
        sleep_then_print(2),
        sleep_then_print(3),
    );
}
// async fn parallel_sum(nums: Vec<i32>) -> i32 {
//     let (send, recv) = tokio::sync::oneshot::channel();
//
//     rayon::spawn(move || {
//         // Perform an expensive computation.
//         let mut sum = 0;
//         for num in nums {
//             sum += num;
//         }
//
//         // Send the result back to Tokio.
//         let _ = send.send(sum);
//     });
//
//     // Wait for the rayon task.
//     recv.await.expect("Panic in rayon::spawn")
// }
//
// #[tokio::main]
// async fn main() {
//     let nums = vec![1; 1024 * 1024];
//     println!("{}", parallel_sum(nums).await);
// }
