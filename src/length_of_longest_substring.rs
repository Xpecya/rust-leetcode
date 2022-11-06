// Given a string s, find the length of the longest substring without repeating characters.
// Example 1:
// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.
// Example 2:
// Output: 1
// Explanation: The answer is "b", with the length of 1.
// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
// Constraints:
// 0 <= s.length <= 5 * 104
// s consists of English letters, digits, symbols and spaces.

// time: 6ms
// memory: 2.2MB
pub fn length_of_longest_substring(s: String) -> i32 {
    let bytes = s.as_bytes();
    let bytes_len = bytes.len();
    if bytes_len <= 1 {
        return bytes_len as i32;
    }
    let mut i = bytes_len - 1;
    let mut last: Vec<u8> = vec![];
    let mut max = 0;
    loop {
        let byte = bytes[i];
        if last.contains(&byte) {
            let mut split = last.split(|num| num - byte == 0);
            let split_next = split.next();
            let split_unwrap = split_next.unwrap();
            last = Vec::from(split_unwrap);
        }
        last.insert(0, byte);
        let last_len = last.len();
        if max < last_len {
            max = last_len;
        }
        if last_len + i < max || i == 0  {
            break;
        }
        i -= 1;
    }
    max as i32
}
