fn main() {
  // println!("Hello, world!");
  println!("{}", is_palindrome(212));
}

fn is_palindrome(x: i32) -> bool {
  x == x.abs().to_string().chars().rev().collect::<String>().parse::<i32>().unwrap_or(0)
}