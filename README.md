# Simple Storage
> Simple, fast and easy to use key-value store.

## Disclaimer 
This project is not intended to be used for a production environment.


## Usage

```rust

  use simple_storage::{Storage, Value};

  fn main() {
    let mut storage = Storage::new("db.json");
    storage.pull();

    // add key
    storage
      .put("username", Value::String("rawnly"));

    // retrive key
    let username = storage
        .get("username")
        .to_string()
        .unwrap();
  }

```
