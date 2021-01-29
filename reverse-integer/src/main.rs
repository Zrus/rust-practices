fn main() {
  // println!("Hello, world!");
  println!("{}", reverse(1534236469));
}

fn reverse(x: i32) -> i32 {
  let mut res: String = x.to_string();
  if x < 0 {
    res = (x * -1).to_string();
  }
  let res = res.chars().rev().collect::<String>().parse::<i32>();
  let res = match res {
    Ok(res) => res,
    Err(_) => 0
  };
  if x < 0 {
    return res * -1;
  }
  res
}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        x.signum() * x
			.abs()
			.to_string()
			.chars()
			.rev()
			.collect::<String>()
			.parse::<i32>()
			.unwrap_or(0)
    }
}