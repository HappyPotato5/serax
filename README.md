[![Crates.io](https://img.shields.io/badge/Crates.io-latest-blue?style=for-the-badge&logo=rust&logoColor=black&labelColor=BBBBBB)](https://crates.io/crates/serax)
[![Docs.rs](https://img.shields.io/badge/Documentation-Serax-blue?style=for-the-badge&logo=Docs.rs&logoColor=black&labelColor=BBBBBB)](https://docs.rs/serax/latest/serax/)
[![Github](https://img.shields.io/badge/Github-Serax-blue?style=for-the-badge&logo=github&logoColor=black&labelColor=BBBBBB)](https://github.com/HappyPotato5/serax)
[![License](https://img.shields.io/badge/License-Serax/License-blue?style=for-the-badge&logo=readdotcv&logoColor=black&labelColor=BBBBBB)](https://github.com/HappyPotato5/serax/blob/master/LICENSE)

# Serax
Serax is a **simple to use** serialization library for Rust.

Comes with **full support** for Rust **primitive** types, **lists**, **maps**, **optionals**...

Any type can be made serializable by deriving the `Serialize` macro, same for `Deserialize`.

## Examples
```rust
use serax::{binary, Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
enum Animal {
    Dog, Cat, Parrot, Turtle, Other(String)
}

#[derive(Debug, Serialize, Deserialize)]
struct Color(u8,u8,u8);

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: u16,
    favorite_letter: char,
    favorite_animal: Animal,
    favorite_color: Color,
}

fn main() {
    let user = User {
        name: "Marcus".to_string(),
        age: 22,
        favorite_letter: 'a',
        favorite_animal: Animal::Parrot,
        favorite_color: Color(128, 255, 64),
    };

    println!("The user is: {:#?}\n", user);
    
    let bytes = binary::to_bytes(&user).expect("Serialization error");

    println!("Or in binary:  {:?}\n", bytes);

    let user_from_bytes: User = binary::from_bytes(bytes).expect("Deserialization error");

    println!("So: {:#?}", user_from_bytes);
}
```