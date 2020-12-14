extern crate rick_and_morty as rm;

// example of getting all characters
async fn get_characters() -> () {
    let c = rm::character::get_all().await;
    match c {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e),
    }
    ()
}

#[tokio::main]
async fn main() {
    get_characters().await;
}
