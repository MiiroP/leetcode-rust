pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let (mut lo, mut hi) = (0, 0);
        let mut f = vec![vec![false; n]; n];

        for i in (0..n).rev() {
            for j in i..n {
                if i == j {
                    f[i][j] = true;
                } else if i == j - 1 {
                    f[i][j] = bytes[i] == bytes[j];
                } else {
                    f[i][j] = bytes[i] == bytes[j] && f[i + 1][j - 1];
                }
                if f[i][j] && j - i > hi - lo {
                    hi = j;
                    lo = i;
                }
            }
        }

        let res = &bytes[lo..(hi + 1)];
        String::from_utf8(res.to_vec()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let res1 = Solution::longest_palindrome("babad".to_string());
        assert!(res1 == "bab" || res1 == "aba");
    }
}
