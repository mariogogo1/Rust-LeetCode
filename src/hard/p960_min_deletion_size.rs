/**
960. 删列造序 III

给定由 n 个小写字母字符串组成的数组 strs ，其中每个字符串长度相等。

选取一个删除索引序列，对于 strs 中的每个字符串，删除对应每个索引处的字符。

比如，有 strs = ["abcdef","uvwxyz"] ，删除索引序列 {0, 2, 3} ，删除后为 ["bef", "vyz"] 。

假设，我们选择了一组删除索引 answer ，那么在执行删除操作之后，最终得到的数组的行中的 每个元素 都是按字典序排列的（即 (strs[0][0] <= strs[0][1] <= ... <= strs[0][strs[0].length - 1]) 和 (strs[1][0] <= strs[1][1] <= ... <= strs[1][strs[1].length - 1]) ，依此类推）。

请返回 answer.length 的最小可能值 。

https://leetcode.cn/problems/delete-columns-to-make-sorted-iii/description/
*/

pub struct Solution;
/// 類似面试题 08.13. 堆箱子
/// 使用動態規劃
/// 同最長上升子序列的動態規劃做法，時間O(N^2)，N=字串長度
/// 只是每次做比較下標i跟j的時候，要做每一直行所有字串都要比較，時間是O(M)，M列字串
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let m = strs.len();
        let n = strs[0].len();

        let mut dp = vec![0; n];

        let mut ans = 0;
        for i in 0..n {
            let mut count = 0;
            for j in 0..i {
                let mut ok = true;
                // 判斷每個字串的字典序
                for k in 0..m {
                    if strs[k].chars().nth(i).unwrap() < strs[k].chars().nth(j).unwrap() {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    count = count.max(dp[j]);
                }
            }
            dp[i] = count + 1;
            ans = ans.max(dp[i]);
        }

        n as i32 - ans
    }
}
