// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.
// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
// Example 2:
// Input: nums = [3,2,4], target = 6
// Output: [1,2]
// Example 3:
// Input: nums = [3,3], target = 6
// Output: [0,1]
// Constraints:
// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// Only one valid answer exists.
// Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?

struct Container {
    num: i32,
    index: usize
}

// time cost: 0ms
// memory cost: 2.4MB
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut index = 0;
    let length = nums.len();
    if length == 2 {
        return vec![0, 1];
    }

    // O(n)
    let mut container_vec = Vec::new();
    while index < length {
        let num = nums[index];
        container_vec.push(Container { num, index });
        index += 1;
    }

    // O(nlogn)
    container_vec.sort_by(|a, b| a.num.partial_cmp(&b.num).unwrap());

    // O(n)
    for container in &container_vec {
        let sub = target - container.num;
        let search_result = container_vec.binary_search_by(|a| a.num.cmp(&sub));
        match search_result {
            Ok(result) => {
                let result_container = &container_vec[result];
                let result_index = result_container.index;
                if result_index == container.index {
                    continue;
                }
                return vec![result_index as i32, container.index as i32];
            },
            Err(_) => continue
        }
    }
    return vec![];
}
