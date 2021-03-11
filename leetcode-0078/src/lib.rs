pub struct Solution;

impl Solution {
    fn dfs(n: usize, nums: &Vec<i32>, tmp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        res.push(tmp.clone());
        for i in n..nums.len() {
            tmp.push(nums[i]);
            Solution::dfs(i + 1, nums, tmp, res);
            tmp.pop();
        }
    }

    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::dfs(0, &nums, &mut vec![], &mut res);
        res
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        println!("{:?}", Solution::subsets(vec![1,2,3]));
    }
}
