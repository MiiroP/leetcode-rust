use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut hi, mut lo) = (height.len() - 1, 0 as usize);
        let mut res = 0;
        while hi > lo {
            if height[hi] > height[lo] {
                res = max(res, height[lo] * (hi - lo) as i32);
                lo += 1;
            } else {
                res = max(res, height[hi] * (hi - lo) as i32);
                hi -= 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
        assert_eq!(Solution::max_area(vec![4,3,2,1,4]), 16);
    }
}
