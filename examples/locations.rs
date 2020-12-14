extern crate rick_morty_api_rust as rm;

// example of getting all locations
async fn get_locations() -> () {
    let c = rm::location::get_all().await;
    match c {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e),
    }
    ()
}

#[tokio::main]
async fn main() {
    get_locations().await;
}
