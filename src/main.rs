extern crate ract;

use ract::value::{RInt,RList,RStr};

fn main() {
  let my_list = RList(vec![RInt(42), RStr("foo".to_string())]);
  println!("{}", my_list);
}
