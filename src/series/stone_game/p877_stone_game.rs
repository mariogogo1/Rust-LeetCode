/**
877. 石子游戏

Alice 和 Bob 用几堆石子在做游戏。一共有偶数堆石子，排成一行；每堆都有 正 整数颗石子，数目为 piles[i] 。

游戏以谁手中的石子最多来决出胜负。石子的 总数 是 奇数 ，所以没有平局。

Alice 和 Bob 轮流进行，Alice 先开始 。 每回合，玩家从行的 开始 或 结束 处取走整堆石头。 这种情况一直持续到没有更多的石子堆为止，此时手中 石子最多 的玩家 获胜 。

假设 Alice 和 Bob 都发挥出最佳水平，当 Alice 赢得比赛时返回 true ，当 Bob 赢得比赛时返回 false 。

https://leetcode.cn/problems/stone-game/description/
*/

/// 此題同 486提做法
/// 加了兩個規則 1.不會平手 2.偶數堆
/// 數學做法
/// 把所有數字分組依照下標的奇偶數分為A組跟B組
/// 數字就是 ABABABABABAB 這樣的分組
/// 一開始ALICE 就先算好 A組總和或是B組總和多，假設A組多
/// 一開始ALICE就拿走A組的元素 剩下的就是BABABABABAB ，BOB一定會選到B組的數字
/// 再下一步"A"BABABABAB或BABABABAB"A" 又會有A組的露在其中一端， ALICE可以再拿走A組的
/// 反覆如此 ALICE可以蒐集完A組全套
/// ALICE 必勝  
impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        true
    }
}
