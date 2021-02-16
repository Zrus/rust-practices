fn main() {
  assert_eq!(is_valid("()[]{}".to_string()), true);
}

pub fn is_valid(s: String) -> bool {
  let mut stack = Vec::new();
  for c in s.chars() {
    match c {
      ')' => {
        if stack.pop() != Some('(') {
          return false;
        }
      }
      ']' => {
        if stack.pop() != Some('[') {
          return false;
        }
      }
      '}' => {
        if stack.pop() != Some('{') {
          return false;
        }
      }
      _ => stack.push(c),
    }
  }
  stack.is_empty()
}
