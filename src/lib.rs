use core::panic;
use std::{collections::HashMap, fs::{File, read_to_string}, io::{Result, Write}, path::Path};
use serde_json::{Value as JSONValue};

pub type Value = serde_json::Value;
pub type JSON = HashMap<String, JSONValue>;

pub struct Storage {
  pub path: String,
  pub content: JSON
}

fn write_json_to_file(filepath: String, json: JSON) {
  let path = Path::new(&filepath);

  let data = match serde_json::to_string(&json) {
    Ok(data) => data,
    Err(err) => panic!("Couldn't serialize JSON: {}", err)
  };

  let mut file = match File::create(&filepath) {
    Ok(file) => file,
    Err(err) => panic!("Couldn't read file: ({}) {}", path.display(), err)
  };

  match file.write_all(data.as_bytes()) {
    Err(err) => panic!("Error writing file: ({}) {}", path.display(), err),
    Ok(_) => ()
  }
}

impl Storage {
  pub fn new(filename: &str) -> Storage {
    let path = Path::new(filename);

    if !path.exists() {
      File::create(path)
        .expect("An error is occurred while trying to open the configuration file.");

      write_json_to_file(filename.to_string(), JSON::new());
    }

    Storage {
      path: String::from(filename),
      content: JSON::new()
    }
  }

  pub fn update_file(&self) -> Result<()> {
    let path = Path::new(&self.path);

    let data = serde_json::to_string(&self.content)?;
    let mut file = File::create(&path)?;
    file.write_all(data.as_bytes())?;

    Ok(())
  }

  pub fn pull(&mut self) -> Result<()> {
    let string_content = read_to_string(Path::new(&self.path))?;
    let data : JSON = serde_json::from_str(&string_content)?;
    self.content = data;

    Ok(())
  }

  pub fn get(&self, key: String) -> JSONValue {
    let data = &self.content;

    return data[&key].clone();
  }

  pub fn put(&mut self, key: String, value: JSONValue) -> Result<()> {
    &self.content.insert(key, value);
    self.update_file()?;

    Ok(())
  }
}
