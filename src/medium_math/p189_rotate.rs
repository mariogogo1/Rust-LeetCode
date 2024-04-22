/**
189. 轮转数组

给定一个整数数组 nums，将数组中的元素向右轮转 k 个位置，其中 k 是非负数。

https://leetcode.cn/problems/rotate-array/submissions/
*/

pub struct Solution;
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if k == 0 {
            return;
        }

        let n = nums.len();
        let k = k as usize;
        let mut visited = vec![false; n];
        let mut start: usize = 0;

        loop {
            if start < n && !visited[start] {
                let mut pos = start;
                let mut value = nums[pos];
                let mut nxt_value = 0;
                loop {
                    if !visited[pos] {
                        visited[pos] = true;
                        pos += k;
                        pos %= n;
                        nxt_value = nums[pos];
                        nums[pos] = value;
                        value = nxt_value;
                    } else {
                        break;
                    }
                }
                start += 1;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {}
    #[test]
    fn test_case() {}
}
