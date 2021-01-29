fn main() {
  println!("Hello, world!");
}

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
  let mut res = vec![];
  let (mut i, mut j) = (0, 0);
  while res.len() < nums1.len() + nums2.len() {
    if j >= nums2.len() {
      res.extend_from_slice(&nums1.clone().split_off(i));
      break;
    }
    if i >= nums1.len() {
      res.extend_from_slice(&nums2.clone().split_off(j));
      break;
    }
    if nums1[i] < nums2[j] {
      res.push(nums1[i]);
      i += 1;
    } else {
      res.push(nums2[j]);
      j += 1;
    }
  }
  if res.len() % 2 == 0 {
    return (res[res.len() / 2] + res[res.len() / 2 - 1]) as f64 / 2.0;
  }
  res[res.len() / 2] as f64
}

    // let (mut a1, mut a2) = if nums1.len() < nums2.len() {
    //     (nums1, nums2)
    // } else {
    //     (nums2, nums1)
    // };
    // let (len1, len2) = (a1.len(), a2.len());
    // let (mut imin, mut imax, half) = (0, len1, (len1 + len2 + 1) / 2);
    // while imin <= imax {
    //     let i = (imin + imax) / 2;
    //     let j = half - i;
    //     if i < imax && a2[j - 1] > a1[i] {
    //         imin = i + 1;
    //     } else
    //     if i > imin && a1[i - 1] > a2[j] {
    //         imax = i - 1;
    //     } else {
            
    //         let mut max_left = 0;
    //         if i == 0 {
    //             max_left = a2[j - 1];
    //         } else
    //         if j == 0 {
    //             max_left = a1[i - 1];
    //         } else {
    //             max_left = std::cmp::max(a1[i - 1], a2[j - 1]);
    //         }
    //         if (len1 + len2) % 2 == 1 {
    //             return max_left as f64;
    //         }
            
    //         let mut min_right = 0;
    //         if i == len1 {
    //             min_right = a2[j];
    //         } else
    //         if j == len2 {
    //             min_right = a1[i];
    //         } else {
    //             min_right = std::cmp::max(a1[i], a2[j]);
    //         }
    //         return (max_left + min_right) as f64 / 2.0;
    //     }
    // }
    // return 0f64;

    // fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    //     let (mut a1, mut a2) = if nums1.len() < nums2.len() {
    //         (nums1, nums2)
    //     } else {
    //         (nums2, nums1)
    //     };
    //     let (len1, len2) = (a1.len(), a2.len());
    //     let (mut imin, mut imax, half) = (0, len1, (len1 + len2 + 1) / 2);
    //     while imin <= imax {
    //         let i = (imin + imax) / 2;
    //         let j = half - i;
    //         if i < imax && a2[j - 1] > a1[i] {
    //             imin = i + 1;
    //         } else
    //         if i > imin && a1[i - 1] > a2[j] {
    //             imax = i - 1;
    //         } else {
                
    //             let mut max_left = 0;
    //             if i == 0 {
    //                 max_left = a2[j - 1];
    //             } else
    //             if j == 0 {
    //                 max_left = a1[i - 1];
    //             } else {
    //                 max_left = std::cmp::max(a1[i - 1], a2[j - 1]);
    //             }
    //             if (len1 + len2) % 2 == 1 {
    //                 return max_left as f64;
    //             }
                
    //             let mut min_right = 0;
    //             if i == len1 {
    //                 min_right = a2[j];
    //             } else
    //             if j == len2 {
    //                 min_right = a1[i];
    //             } else {
    //                 min_right = std::cmp::min(a1[i], a2[j]);
    //             }
    //             return (max_left + min_right) as f64 / 2.0;
    //         }
    //     }
    //     return 0f64;