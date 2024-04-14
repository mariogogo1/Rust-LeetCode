/**
1927. 求和游戏

Alice 和 Bob 玩一个游戏，两人轮流行动，Alice 先手 。

给你一个 偶数长度 的字符串 num ，每一个字符为数字字符或者 '?' 。每一次操作中，如果 num 中至少有一个 '?' ，那么玩家可以执行以下操作：

选择一个下标 i 满足 num[i] == '?' 。
将 num[i] 用 '0' 到 '9' 之间的一个数字字符替代。
当 num 中没有 '?' 时，游戏结束。

Bob 获胜的条件是 num 中前一半数字的和 等于 后一半数字的和。Alice 获胜的条件是前一半的和与后一半的和 不相等 。

比方说，游戏结束时 num = "243801" ，那么 Bob 获胜，因为 2+4+3 = 8+0+1 。如果游戏结束时 num = "243803" ，那么 Alice 获胜，因为 2+4+3 != 8+0+3 。
在 Alice 和 Bob 都采取 最优 策略的前提下，如果 Alice 获胜，请返回 true ，如果 Bob 获胜，请返回 false 。

https://leetcode.cn/problems/sum-game/description/
*/

/// ?的總數是奇數個 A必勝，因為A是後手結束，一定存在一種選擇使得兩半總和不相同
/// ?的總數是偶數個 先討論簡單的
/// ??x | abc  a+b+c = y , 因為B是後手 所以當y-x = 9 的時候，B必然可以控制兩個?的總和是9 這時候B必勝，A必輸 。
/// y-x != 9 的時候，則B必輸，A必勝
/// 此時做一個觀察 每兩個同邊的?，B的後手都可以控制使得兩個問號的和=9 所以當同邊的問號數量是4個的時候，y-x=18 則B必勝，A必輸
/// 接者討論以下兩個情況是否等價
/// ??x | abc  A先手勝 以及  ??? | ?bc A先手勝
/// 以上兩個分別等價於以下，只要以下兩個情況等價 就說明上面兩個等價
/// ?zx | abc  B先手輸->(1) 以及  ??? | ?bc B先手輸->(2) ,狀況(2)到狀況(1)，A是後手，若在B必輸的情況下，A最好的選擇是選擇B對面的問號並且取一樣的值
/// 例如  ??3 | 3bc B選擇3，A也選擇3，如此A可以保證B維持在必輸的局面
///
/// ??x | abc  A先手輸 以及  ??? | ?bc A先手輸，這蠻容易的，B為後手，每次都會確保選擇B對面的?以保證維持A必輸的局面。
/// 實作：計算 前半?_count  後半?_count
///           前半總和      後半總和
///  前半?_count + 後半?_count %2 == 1 A勝
///   前半?_count + 後半?_count %2 == 0  則 (前半?_count-後半?_count)/2*9 != 後半總和-前半總和 ，A勝

impl Solution {
    pub fn sum_game(num: String) -> bool {
        let mut front_count: i32 = 0;
        let mut back_count: i32 = 0;
        let mut front_sum: i32 = 0;
        let mut back_sum: i32 = 0;
        let half = num.len() / 2;

        for (i, ch) in num.chars().enumerate() {
            match ch {
                '0'..='9' => {
                    if i < half {
                        front_sum += ch.to_digit(10).unwrap() as i32;
                    } else {
                        back_sum += ch.to_digit(10).unwrap() as i32;
                    }
                }
                '?' => {
                    if i < half {
                        front_count += 1;
                    } else {
                        back_count += 1;
                    }
                }
                _ => todo!(),
            }
        }

        // 根据题目条件判断 A 是否能赢得游戏
        if (front_count + back_count) % 2 == 1 {
            // 奇数个问号，A 必胜
            return true;
        } else {
            // 偶数个问号
            if (front_count - back_count) / 2 * 9 != back_sum - front_sum {
                // 条件不满足，A 胜利
                return true;
            }
        }
        false
    }
}
