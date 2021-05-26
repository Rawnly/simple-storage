# Simple Storage
> Simple, fast and easy to use key-value store.

## Installation
```toml
# Cargo.toml
simple-storage = "0.0.2"
```

## Usage
```rust
  use std::io::Result;
  use simple_storage::{Storage, Value};

  fn main() -> Result<()> {
    let mut storage = Storage::new("/tmp/db.json");
    storage.pull();

    // add key
    storage.put("username", Value::String("rawnly"))?;

    // retrive key
    let username = match storage.get("username") {
      Some(username) => username,
      Err(key_not_found_error) => "rawnly" // handle the error here
    }
        
    Ok(())
  }
```

