use std::fmt;

pub enum Value {
  RInt(int),
  RStr(String),
  RList(Vec<Value>),
  RNil
}

impl fmt::Show for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      RNil           => write!(f, "nil"),
      RInt(i)        => write!(f, "{:i}", i),
      RStr(ref s)    => write!(f, "\"{:s}\"", s.as_slice()),
      RList(ref arr) => {
        let mut string: String = "".to_string();
        for &ref elem in arr.iter() {
          string.push_str(format!("{}", elem).as_slice());
          string.push_char(' ');
        }
        string.pop_char();
        write!(f, "({:s})", string)
      }
    }
  }
}
