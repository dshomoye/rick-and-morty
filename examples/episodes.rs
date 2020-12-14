extern crate rick_morty_api_rust as rm;

// example of getting all episodes
async fn get_episodes() -> () {
    let c = rm::episode::get_all().await;
    match c {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e),
    }
    ()
}

#[tokio::main]
async fn main() {
    get_episodes().await;
}
