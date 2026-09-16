# Serax
Serax is a **simple to use** serialization library for Rust.

Comes with **full support** for Rust **primitive** types, **lists**, **maps**, **optionals**...

Any type can be made serializable by deriving the `Serialize` macro, same for `Deserialize`.

**<div class="warning">Support for Unsized types is still work in progress.</div>**

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