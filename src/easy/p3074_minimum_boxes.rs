/**
3074. 重新分装苹果

给你一个长度为 n 的数组 apple 和另一个长度为 m 的数组 capacity 。

一共有 n 个包裹，其中第 i 个包裹中装着 apple[i] 个苹果。同时，还有 m 个箱子，第 i 个箱子的容量为 capacity[i] 个苹果。

请你选择一些箱子来将这 n 个包裹中的苹果重新分装到箱子中，返回你需要选择的箱子的 最小 数量。

注意，同一个包裹中的苹果可以分装到不同的箱子中。

https://leetcode.cn/problems/apple-redistribution-into-boxes/description/
*/

pub struct Solution;
impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        let mut total = 0;
        for i in 0..apple.len() {
            total += apple[i];
        }
        capacity.sort_by(|x, y| y.cmp(&x));
        for i in 0..capacity.len() {
            total -= capacity[i];
            if total <= 0 {
                return (i + 1) as i32;
            }
        }
        return capacity.len() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::minimum_boxes(vec![1, 3, 2], vec![4, 3, 1, 5, 2]),
            2
        );
        assert_eq!(Solution::minimum_boxes(vec![5, 5, 5], vec![2, 4, 2, 7]), 4);
    }
    #[test]
    fn test_case() {}
}
