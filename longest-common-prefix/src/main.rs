fn main() {
    let strs = vec!["reflower".to_string(), "flow".to_string(), "flight".to_string()];
    assert_eq!(longest_common_prefix(strs), "fl".to_string());
}

pub fn __longest_common_prefix(strs: Vec<String>) -> String {
  divide(&strs, 0, strs.len() - 1)
}

fn divide(strs: &Vec<String>, l: usize, r: usize) -> String {
  if strs.len() == 0 {
    return "".to_string();
  }

  if l == r {
    return strs[l].clone();
  }
  
  let mid = (l + r) / 2;
  let mut left = divide(strs, l, mid).clone();
  let mut right = divide(strs, mid + 1, r).clone();

  return if left.len() < right.len() {
    conquer(&mut left, &mut right)
  } else {
    conquer(&mut right, &mut left)
  }
}

fn conquer(l: &mut String, r: &mut String) -> String {
  while l.len() > 0 {
    if r.starts_with(l.as_str()) {
      return l.clone();
    }
    l.pop();
  }

  "".to_string()
}

pub fn _longest_common_prefix(strs: Vec<String>) -> String {
        if let Some(first_str) = strs.first() {
            for (i, ch) in first_str.char_indices() {
                for str_ in &strs {
                    if str_.chars().nth(i).unwrap_or_default() != ch {
                        return first_str[..i].to_string();
                    }
                }
            }
            return first_str.clone();
        }
        String::from("")
    }

pub fn longest_common_prefix(strs: Vec<String>) -> String { 
  match strs.is_empty() {
    true => "".to_string(),
    _ => {
      strs.iter().skip(1).fold(strs[0].clone(), |acc, x| {
        acc
          .chars()
          .zip(x.chars())
          .take_while(|(x,y)| x == y)
          .map(|(x, _)| x)
          .collect()
      })
    }
  }
}

