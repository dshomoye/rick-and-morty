# Rick And Morty Rust Helper

This is a rust crate for working with the rick and morty api [here](https://rickandmortyapi.com).
Primarily a learning (rust) experience for me. Contributions welcome!

---
## Usage

 - Get a single character by id:

```rust
 extern crate rick_and_morty as rm;

 async fn get_character() -> () {
   let c = rm::character::get(1).await;
   match c {
       Ok(res) => println!("{:?}", res),
       Err(e) => println!("{:?}", e),
    }
}
```

- Get all characters 

```rust
 extern crate rick_and_morty as rm;

 async fn get_characters() -> () {
   let c = rm::character::get_all().await;
   match c {
       Ok(res) => println!("{:?}", res),
       Err(e) => println!("{:?}", e),
    }
}
```
---

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
