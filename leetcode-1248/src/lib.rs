pub struct Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let (mut lo, mut hi) = (0, 0);
        let mut res = 0;
        let mut odd_cnt = 0;
        loop {
            if hi == n {
                while odd_cnt > k && lo < n {
                    if nums[lo] % 2 == 1 {
                        odd_cnt -= 1;
                    }
                    if odd_cnt == k {
                        res += 1;
                        break;
                    }
                    lo += 1;
                }
                 break;
            }
            if odd_cnt < k {
                if nums[hi] % 2 == 1 {
                    odd_cnt += 1;
                    if odd_cnt == k {
                        res += 1;
                    }
                }
                hi += 1;
            } else {
                if nums[lo] % 2 == 1 {
                    odd_cnt -= 1;
                }
                lo += 1;
                if odd_cnt == k {
                    res += 1;
                }
            }
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn testcase0() {
        assert_eq!(Solution::number_of_subarrays(vec![1, 1, 2, 1, 1], 3), 2);
    }
    #[test]
    fn testcase1() {
        assert_eq!(Solution::number_of_subarrays(vec![2, 4, 6], 1), 0);
    }
    #[test]
    fn testcase2() {
        assert_eq!(
            Solution::number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2),
            7
        );
    }
}
