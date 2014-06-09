use std::collections::HashMap;

use value::Value;

pub struct Namespace {
  name: String,
  vars: HashMap<String, Value>
}

impl Namespace {
  pub fn new(name: String) -> Namespace {
    Namespace {
      name: name,
      vars: HashMap::new()
    }
  }

  pub fn get<'a>(&'a self, key: String) -> Option<&'a Value> {
    self.vars.find(&key)
  }

  pub fn set(&mut self, key: String, value: Value) {
    self.vars.insert(key, value);
  }
}
