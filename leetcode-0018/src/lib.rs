use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut map_two_sums: HashMap<i32, Vec<(usize, usize)>> = HashMap::new();
        let mut sums: HashSet<i32> = HashSet::new();
        let mut res: Vec<Vec<i32>> = Vec::new();
        let n = nums.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let tmp = nums[i] + nums[j];
                let v = map_two_sums.entry(tmp).or_insert(Vec::new());
                v.push((i, j));
                sums.insert(tmp);
            }
        }
        for s in sums {
            let complement = target - s;
            if let Some(v) = map_two_sums.get(&complement) {

            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
