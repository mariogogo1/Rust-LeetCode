/**
60. 排列序列

给出集合 [1,2,3,...,n]，其所有元素共有 n! 种排列。

按大小顺序列出所有排列情况，并一一标记，当 n = 3 时, 所有排列如下：

"123"
"132"
"213"
"231"
"312"
"321"
给定 n 和 k，返回第 k 个排列。

https://leetcode.cn/problems/permutation-sequence/description/
*/
pub struct Solution;
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut visited = vec![false; n as usize];
        let mut ans = String::new();
        let mut divisor = 1;

        for i in (1..n).rev() {
            divisor *= i;
        }

        let mut left = k - 1;
        let mut temp = n;
        while left > 0 {
            let mut time = left / divisor + 1;
            left %= divisor;

            let mut idx = 0;
            while time > 0 {
                if !visited[idx] {
                    time -= 1;
                }
                idx += 1;
            }
            visited[idx - 1] = true;
            ans.push_str(&(idx).to_string());

            temp -= 1;
            divisor /= temp;
        }

        for (key, &value) in visited.iter().enumerate() {
            if !value {
                ans.push_str(&(key + 1).to_string());
            }
        }

        ans
    }
}
