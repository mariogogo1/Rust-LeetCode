/**
646. 最长数对链

给你一个由 n 个数对组成的数对数组 pairs ，其中 pairs[i] = [lefti, righti] 且 lefti < righti 。

现在，我们定义一种 跟随 关系，当且仅当 b < c 时，数对 p2 = [c, d] 才可以跟在 p1 = [a, b] 后面。我们用这种形式来构造 数对链 。

找出并返回能够形成的 最长数对链的长度 。

你不需要用到所有的数对，你可以以任何顺序选择其中的一些数对来构造。

https://leetcode.cn/problems/maximum-length-of-pair-chain/description/
*/
/// 435. 无重叠区间
/// 646. 最长数对链
/// 以上兩題概念一模一樣
impl Solution {
    pub fn find_longest_chain(mut intervals: Vec<Vec<i32>>) -> i32 {
        let n = intervals.len();
        intervals.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

        let mut first = i32::MIN;
        let mut ans = 0;
        for interval in intervals {
            if first < interval[0] {
                first = interval[1];
                ans += 1;
            }
        }
        ans
    }
}
