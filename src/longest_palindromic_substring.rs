// Given a string s, return the longest palindromic substring in s.
// Example 1:
// Input: s = "babad"
// Output: "bab"
// Explanation: "aba" is also a valid answer.
// Example 2:
// Input: s = "cbbd"
// Output: "bb"
// Constraints:
// 1 <= s.length <= 1000
// s consist of only digits and English letters.

// time: 60ms
// memory: 2.2MB
pub fn longest_palindrome(s: String) -> String {
    let len = s.len();
    match len {
        0 => return s,
        1 => return s,
        _ => {
            let mut current_length = len;
            while current_length > 0 {
                let mut start = 0;
                while start + current_length <= len {
                    let end = start + current_length;
                    let sub = &s[start..end];
                    if palindrome_check(sub) {
                        return sub.to_string();
                    }
                    start += 1;
                }
                current_length -= 1;
            }

        }
    }
    String::new()
}

fn palindrome_check(s: &str) -> bool {
    let len = s.len();
    match len {
        0 => return true,
        1 => return true,
        _ => {
            let bytes = s.as_bytes();
            let first = bytes[0];
            let last = bytes[len - 1];
            if first == last {
                return palindrome_check(&s[1..len - 1])
            }
        }
    }
    false
}
