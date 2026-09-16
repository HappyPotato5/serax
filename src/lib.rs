
//! Serax is a **simple to use** serialization library for Rust.
//! 
//! Comes with **full support** for Rust **primitive** types, **lists**, **maps**, **optionals**...
//! 
//! Any type can be made serializable by deriving the [`Serialize`] macro, same for [`Deserialize`].
//! 
//! **<div class="warning">Support for Unsized types is still work in progress.</div>**
//! 
//! # Examples
//! ```
//! use serax::{binary, Serialize, Deserialize};
//! 
//! #[derive(Debug, Serialize, Deserialize)]
//! enum Animal {
//!     Dog, Cat, Parrot, Turtle, Other(String)
//! }
//! 
//! #[derive(Debug, Serialize, Deserialize)]
//! struct Color(u8,u8,u8);
//! 
//! #[derive(Debug, Serialize, Deserialize)]
//! struct User {
//!     name: String,
//!     age: u16,
//!     favorite_letter: char,
//!     favorite_animal: Animal,
//!     favorite_color: Color,
//! }
//! 
//! fn main() {
//!     let user = User {
//!         name: "Marcus".to_string(),
//!         age: 22,
//!         favorite_letter: 'a',
//!         favorite_animal: Animal::Parrot,
//!         favorite_color: Color(128, 255, 64),
//!     };
//! 
//!     println!("The user is: {:#?}\n", user);
//!     
//!     let bytes = binary::to_bytes(&user).expect("Serialization error");
//! 
//!     println!("Or in binary:  {:?}\n", bytes);
//! 
//!     let user_from_bytes: User = binary::from_bytes(bytes).expect("Deserialization error");
//! 
//!     println!("So: {:#?}", user_from_bytes);
//! }
//! ```

/// The submodule that defines traits related to **serialization**.
pub mod serialize;

/// The submodule that defines traits related to **deserialization**.
pub mod deserialize;

/// The submodule that handles **binary** serialization.
/// 
/// Use [`binary::to_bytes()`] to convert a [`Serializable`](Serialize) object into its binary serial (`Vec<u8>`).
/// 
/// Use [`binary::from_bytes()`] to reconstruct a [`Deserializable`](Deserialize) object from its binary serial (`Vec<u8>`).
pub mod binary;

#[doc(inline)]
pub use serialize::{ Serialize, SerializeUnsized };
#[doc(inline)]
pub use deserialize::{ Deserialize, DeserializeUnsized };


/// A derive macro that automatically implements [`Serialize`] for any `Sized` type.
#[doc(inline)]
pub use serax_macros::Serialize;

/// A derive macro that automatically implements [`Deserialize`] for any `Sized` type.
#[doc(inline)]
pub use serax_macros::Deserialize;