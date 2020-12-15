#![allow(dead_code)]

extern crate rick_and_morty as rm;


// example of getting all characters
async fn get_characters() -> () {
    let c = rm::character::get_all().await;
    match c {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e),
    }
}

// get multiple characters by id
async fn get_multiple() -> () {
    let ids = vec![1,2,3];
    let cs = rm::character::get_multiple(ids).await;
    match cs {
        Ok(chars) => println!("{:?}", chars),
        Err(e) => println!("{:?}", e),
    }
}

async fn get_page() -> () {
    let cp = rm::character::get_page(3).await;
    match cp {
        Ok(p) => println!("{:?}", p),
        Err(e) => println!("{:?}", e),
    }
}

async fn get_single() -> Result<(), rm::Error> {
    let char = rm::character::get(1).await?;
    let origin = char.origin().await?;
    let epi = char.episodes().await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // get_characters().await;
    // get_multiple().await;
    // get_page().await;
    get_single().await;
}
