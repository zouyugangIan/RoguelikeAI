#[tokio::main]

async fn main() {
    let handle = tokio::spawn(async {
        println!("Hello World");
        "return value!!"
    });
    let out = handle.await.unwrap();
    println!("got value:{:?}", out);
}
