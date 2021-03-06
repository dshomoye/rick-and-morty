#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! # rick_and_morty
//! 
//! The `rick_and_morty` crate provides a wrapper around [rickandmortyapi.com](https://rickandmortyapi.com)
//!
//!  It exports all the basic entities available from the api:
//! - [`character`](character)
//! - [`episode`](episode)
//! - [`location`](location)
//! 
//! Each module provides: `get`, `get_all`, `get_multiple` for equvailent REST endpoints and matching structs.
//! The structs also have helper methods for getting related entities. 
//! For example [`Character.origin()`](character/struct.Character.html#method.origin) method returns a `Location` object.
//! 
//! All provided functions/methods are asynchronous:
//! 
//! ## Examples:
//! 
//! - Getting all characters:
//! 
//! ```rust
//! use rick_and_morty as rm;
//! 
//! async fn get_characters() -> () {
//!   let c = rm::character::get_all().await;
//!   match c {
//!       Ok(res) => println!("{:?}", res),
//!       Err(e) => println!("{:?}", e),
//!   }
//! }
//! ```
//! 
//! - Getting a single location:
//! 
//! ```rust
//! use rick_and_morty as rm;
//! 
//! async fn get_location() -> () {
//!     let c = rm::location::get(1).await;
//!     match c {
//!       Ok(res) => println!("{:?}", res),
//!       Err(e) => println!("{:?}", e),
//!    }
//! }
//! ```
//! 

mod entities;
mod entity;

pub use self::entities::{character::character, episode::episode, location::location};
pub use self::entity::entity::{Error, Object, PageResponse, Info};
