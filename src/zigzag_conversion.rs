// The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this:
// (you may want to display this pattern in a fixed font for better legibility)
// P   A   H   N
// A P L S I I G
// Y   I   R
// (PAYPAL IS HIRING)
// And then read line by line: "PAHNAPLSIIGYIR"
// Write the code that will take a string and make this conversion given a number of rows:
// string convert(string s, int numRows);
// Example 1:
// Input: s = "PAYPALISHIRING", numRows = 3
// Output: "PAHNAPLSIIGYIR"
// Example 2:
// Input: s = "PAYPALISHIRING", numRows = 4
// Output: "PINALSIGYAHRPI"
// Explanation:
// P     I    N
// A   L S  I G
// Y A   H R
// P     I
// Input: s = "A", numRows = 1
// Output: "A"
// Constraints:
// 1 <= s.length <= 1000
// s consists of English letters (lower-case and upper-case), ',' and '.'.
// 1 <= numRows <= 1000

// time cost: 3ms
// memory cost: 2.2MB
pub fn convert(s: String, num_rows: i32) -> String {
    let len = s.len();
    let offset = num_rows * 2 - 2;
    if num_rows == 1 || num_rows as usize >= len {
        return s;
    }
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut result_vec = Vec::new();
    while i < len {
        result_vec.push(bytes[i]);
        i += offset as usize;
    }
    i = 0;
    let mut minus = 1;
    while minus <= num_rows - 1 {
        while i < len {
            let mut next = i + offset as usize;
            let first = i + minus as usize;
            let second = next - minus as usize;
            if first < len {
                result_vec.push(bytes[first]);
            }
            if second > first && second < len {
                result_vec.push(bytes[second]);
            }
            i = next;
        }
        minus += 1;
        i = 0;
    }
    String::from_utf8(result_vec).unwrap()
}
