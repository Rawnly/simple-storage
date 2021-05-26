use simple_storage::{
  Storage,
  Value
};

#[test]
#[should_panic]
fn test_get_empty_key() {
  let storage = Storage::new("/tmp/simple-storage.json");

  storage
    .get(String::from("not-existing-key"))
    .expect("Key not found.");
}

#[test]
fn test_put_key() {
  let mut storage = Storage::new("/tmp/simple-storage.json");

  storage
    .put(String::from("abracadabra"), Value::Bool(true))
    .expect("An error is occurred.")
}


#[test]
fn test_get_key() {
  let mut storage = Storage::new("/tmp/simple-storage.json");

  storage
    .put(String::from("abracadabra"), Value::Bool(true))
    .expect("An error is occurred.");

  let res = storage
    .get(String::from("abracadabra"))
    .expect("An error is occurred.")
    .as_bool()
    .expect("It is not a boolean.");

  assert_eq!(res, true);
}
