extern crate ract;

use ract::namespace::Namespace;
use ract::value::RInt;

#[test]
fn get_set_test() {
  let mut ns = Namespace::new("user".to_string());

  ns.set("foo".to_string(), RInt(42));

  match ns.get("foo".to_string()) {
    Some(&RInt(value)) => assert!(value == 42),
    Some(_) => assert!(false),
    None => assert!(false)
  }
}
