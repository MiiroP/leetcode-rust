use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for i in 0..nums.len() {
            let complement = target - nums[i];
            if let Some(j) = map.get(&complement) {
                return vec![i as i32, *j as i32];
            }
            map.insert(nums[i], i);
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
