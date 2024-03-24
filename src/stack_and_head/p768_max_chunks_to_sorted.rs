/**
768. 最多能完成排序的块 II

给你一个整数数组 arr 。

将 arr 分割成若干 块 ，并将这些块分别进行排序。之后再连接起来，使得连接的结果和按升序排序后的原数组相同。

返回能将数组分成的最多块数？

Hint:

https://leetcode.cn/problems/max-chunks-to-make-sorted-ii/description/
*/
pub struct Solution;
impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        for i in 0..arr.len() {
            let mut max_value = arr[i];
            while let Some(&top) = stack.last() {
                if top > arr[i] {
                    max_value = max_value.max(top);
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(max_value)
        }

        stack.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![5, 4, 3, 2, 1]), 1);
        assert_eq!(Solution::max_chunks_to_sorted(vec![2, 1, 3, 4, 4]), 4);
    }
    #[test]
    fn test_case() {}
}
