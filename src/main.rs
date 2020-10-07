#[async_std::main]
async fn main() -> surf::Result<()> {
    let client = surf::client();
    let string = client
        .head("https://httpbin.org/head")
        .recv_string()
        .await?;
    println!("{}", string);
    Ok(())
}
