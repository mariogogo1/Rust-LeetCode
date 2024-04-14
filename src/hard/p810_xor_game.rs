/**
810. 黑板异或游戏

黑板上写着一个非负整数数组 nums[i] 。

Alice 和 Bob 轮流从黑板上擦掉一个数字，Alice 先手。如果擦除一个数字后，剩余的所有数字按位异或运算得出的结果等于 0 的话，当前玩家游戏失败。 另外，如果只剩一个数字，按位异或运算得到它本身；如果无数字剩余，按位异或运算结果为 0。

并且，轮到某个玩家时，如果当前黑板上所有数字按位异或运算结果等于 0 ，这个玩家获胜。

假设两个玩家每步都使用最优解，当且仅当 Alice 获胜时返回 true。

https://leetcode.cn/problems/chalkboard-xor-game/description/
*/
pub struct Solution;

/// 思考一個問題 什麼情況下 B一定會輸
/// 當黑板上的任意一個數nums[i]被擦掉後，異或結果都是0
/// 所以
/// 我們有 num[1]^...^num[n] != 0
/// 令 num[1]^...^num[n] = num[i]^other[i] 對每一個i都成立
/// 當擦掉 num[i] 只剩下other[i]，other[i] = 0  (因為當黑板上的任意一個數nums[i]被擦掉後，異或結果都是0)
/// 我們把 other[1]^..^other[n] = 0 且 other[1]^..^other[n] = (num[1]^...^num[n])^...^(num[1]^...^num[n])   一共有(n-1)組(num[1]^...^num[n])
/// 所以知道 (n-1) 為偶數的情況下 A必勝 <-> B必輸 <-> B行動時 n為奇數 <-> A行動時 n為偶數 (第一個結論)
/// 同理 當A行動時 n如果是奇數就會輸 除非 "當前所有異或總和" 已經=0 (第二個結論)

impl Solution {
    pub fn xor_game(nums: Vec<i32>) -> bool {
        if nums.len() % 2 == 0 {
            return true;
        }
        let mut xor = 0;
        for num in nums {
            xor ^= num;
        }
        xor == 0
    }
}
