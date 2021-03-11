pub struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::running_sum(vec![1,2,3,4]), vec![1,3,6,10]);
        assert_eq!(Solution::running_sum(Vec::new()), Vec::new());
    }
}
