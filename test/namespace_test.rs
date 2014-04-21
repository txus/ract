extern crate ract;

use ract::namespace::Namespace;
use ract::value::RInt;

#[test]
fn get_set_test() {
  let mut ns = Namespace::new(~"user");

  ns.set(~"foo", RInt(42));

  match ns.get(~"foo") {
    Some(&RInt(value)) => assert!(value == 42),
    Some(_) => assert!(false),
    None => assert!(false)
  }
}
