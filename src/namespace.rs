extern crate collections;

use value::Value;

pub struct Namespace {
  name: ~str,
  vars: collections::HashMap<~str, Value>
}

impl Namespace {
  pub fn new(name: ~str) -> Namespace {
    Namespace {
      name: name,
      vars: collections::HashMap::new()
    }
  }

  pub fn get<'a>(&'a self, key: ~str) -> Option<&'a Value> {
    self.vars.find(&key)
  }

  pub fn set(&mut self, key: ~str, value: Value) {
    self.vars.insert(key, value);
  }
}
