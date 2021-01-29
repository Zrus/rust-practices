fn main() {
  let nums = vec![2, 7, 11, 13];
  let target = 9;
  println!("{:?}", two_sum(nums, target));
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut res: Vec<i32> = Vec::new();
  let mut flag = 0;
  let mut count = 1;
  while flag < nums.len() {
      if target - nums[count] == nums[flag] {
          res.push(flag as i32);
          res.push(count as i32);
          return res;
      }
      count += 1;
      if count == nums.len() {
          flag += 1;
          count = flag + 1;
      }
  }
  res
}
