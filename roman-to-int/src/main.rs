fn main() {
    println!("{}", roman_to_int("MCMLXXXVI".to_string()));
}

fn roman_to_int (s: String) -> i32 {
  let symbols: std::collections::HashMap<&str, i32> = [
    ("I", 1), 
    ("V", 5),
    ("X", 10),
    ("L", 50),
    ("C", 100),
    ("D", 500),
    ("M", 1000)
  ].iter().cloned().collect();
  
  let mut res = 0;
  let mut prev = 0;
  for c in s.chars() {
    if let Some(&cur) = symbols.get(&c.to_string().as_str()) {
      res += cur;
      if cur > prev {
        res -= 2 * prev;
      }
      prev = cur;
    }
  }
  res
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.chars()
        .rev()
        .map(|c| match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        })
        .scan(i32::min_value(), |state, x| {
            let result = if x < *state { -x } else { x };
            *state = x;
            Some(result)
        })
        .sum()
    }
}