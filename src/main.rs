struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        if len < 2 {
            return s;
        }
        let mut input: Vec<char> = Vec::new();
        let mut max_idx = 0;
        let mut max_len = 1;
        let mut center = 0;
        let mut p = Vec::with_capacity(input.len());
        for c in s.chars() {
            input.push('#');
            p.push(0);
            input.push(c);
            p.push(0);
        }
        input.push('#');
        p.push(0);

        let mut max_right = 0;
        for i in 0..input.len() {
            if i < max_right {
                let mirror = 2 * center - i;
                p[i] = (max_right - i).min(p[mirror]);
            }
            let mut left = i as i32 - (1 + p[i]) as i32;
            let mut right = i as i32 + (1 + p[i]) as i32;
            while left >= 0 && right < input.len() as i32 && input[left as usize] == input[right as usize] {
                p[i] += 1;
                left -= 1;
                right += 1;
            }
            if i + p[i] > max_right {
                max_right = i + p[i];
                center = i;
            }
            if p[i] > max_len {
                max_len = p[i];
                max_idx = (i - max_len) / 2;
            }
        }
        (&s[max_idx..max_idx + max_len]).to_string()
    }
}

fn main() {
    let input1 = "ABBBB".to_owned();
    let result1 = Solution::longest_palindrome(input1);
    assert_eq!("BBBB", &result1);
    let input2 = "ABBABB".to_owned();
    let result2 = Solution::longest_palindrome(input2);
    assert_eq!("BBABB", &result2);
}
