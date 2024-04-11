/**
752. 打开转盘锁

你有一个带有四个圆形拨轮的转盘锁。每个拨轮都有10个数字： '0', '1', '2', '3', '4', '5', '6', '7', '8', '9' 。每个拨轮可以自由旋转：例如把 '9' 变为 '0'，'0' 变为 '9' 。每次旋转都只能旋转一个拨轮的一位数字。

锁的初始数字为 '0000' ，一个代表四个拨轮的数字的字符串。

列表 deadends 包含了一组死亡数字，一旦拨轮的数字和列表里的任何一个元素相同，这个锁将会被永久锁定，无法再被旋转。

字符串 target 代表可以解锁的数字，你需要给出解锁需要的最小旋转次数，如果无论如何不能解锁，返回 -1 。

https://leetcode.cn/problems/open-the-lock/description/
*/
pub struct Solution;
use std::collections::HashSet;

impl Solution {
    fn count_path(x: usize) -> Vec<usize> {
        let mut nums = vec![];
        let a = [(x / 1000), (x / 100) % 10, (x / 10) % 10, x % 10];
        let mut t = 10000;
        for &v in a.iter() {
            t /= 10;
            if v == 0 {
                nums.push(x + 1 * t);
                nums.push(x + 9 * t);
            } else if v == 9 {
                nums.push(x - 1 * t);
                nums.push(x - 9 * t);
            } else {
                nums.push(x + 1 * t);
                nums.push(x - 1 * t);
            }
        }

        nums
    }

    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut visited = vec![false; 10000];
        let mut start = vec![0];
        let target_num = target.parse::<usize>().unwrap();
        let deadends_set: HashSet<usize> = deadends.iter().map(|s| s.parse().unwrap()).collect();

        for v in deadends_set {
            visited[v] = true;
        }

        let mut step = 0;

        while !start.is_empty() {
            let mut new_start = vec![];
            for &v in &start {
                if v == target_num {
                    return step;
                }
                if visited[v] {
                    continue;
                }
                visited[v] = true;
                new_start.extend(Self::count_path(v));
            }
            start = new_start;
            step += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::open_lock(vec!["8888".to_string()], "0009".to_string()),
            1
        );
    }
    #[test]
    fn test_case() {}
}
