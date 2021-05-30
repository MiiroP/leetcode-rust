pub struct Solution;

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut res = String::new();
        for b in address.chars() {
            if b == '.' {
                res.push_str("[.]");
            } else {
                res.push(b);
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
        assert_eq!(
            Solution::defang_i_paddr(String::from("1.1.1.1")),
            String::from("1[.]1[.]1[.]1")
        );
    }
}
