/**
LCP 25. 古董键盘

小扣在秋日市集购买了一个古董键盘。由于古董键盘年久失修，键盘上只有 26 个字母 a~z 可以按下，且每个字母最多仅能被按 k 次。

小扣随机按了 n 次按键，请返回小扣总共有可能按出多少种内容。由于数字较大，最终答案需要对 1000000007 (1e9 + 7) 取模。

提示：

1 <= k <= 5
1 <= n <= 26*k

https://leetcode.cn/problems/Uh984O/description/
*/
pub struct Solution;

// 骨董鍵盤
///
use std::collections::HashMap;

impl Solution {
    const VAL_MOD: i64 = 1_000_000_007;
    /// 用插入的想法寫
    /// 假設當前字串都是由abc三種字母組成，長度是4(4+1=5個空格)
    /// 那麼用d字母可以選擇插入0個~K個進入字串的5個空格之中
    /// dp[i][j] 表示長度i，字母使用數量j 的字串排列總數
    /// dp[i][j] = dp[i][j-1] + C(i,1)*dp[i-1][j-1] +... +C(i,k)*dp[i-k][j-1]
    /// dp[0][j] = 1

    pub fn keyboard(k: i32, n: i32) -> i64 {
        let n = n as usize;
        let k = k as usize;
        let mut dp: Vec<Vec<i64>> = vec![vec![0; 27]; n + 1];
        let mut combine: Vec<Vec<i64>> = vec![vec![0; k + 1]; n + 1];
        for i in 0..=n {
            combine[i][0] = 1;
            for j in 1..=k.min(i) {
                combine[i][j] = combine[i - 1][j - 1] + combine[i - 1][j];
                combine[i][j] %= Self::VAL_MOD;
            }
        }

        for j in 0..=26 {
            dp[0][j] = 1;
        }
        for i in 1..=n {
            for j in 1..=26 {
                for delta in 0..=k.min(i) {
                    dp[i][j] += combine[i][delta] * dp[i - delta][j - 1] % Self::VAL_MOD;
                    dp[i][j] %= Self::VAL_MOD;
                }
            }
        }
        //println!("{:?}", dp);
        //println!("{:?}", combine);

        return dp[n][26];
    }

    /// dp(n)[(a,b,c,d,e,f)] 紀錄為 有a個按鍵用了0次，b個按鍵各用了1次....，f個按鍵各用了5次 的排列數
    /// n = a*0 + b*1 + c*2 + d*3 + e*4 + f*5
    /// 26 = a+b+c+d+e+f
    ///
    /// 總排列數：F(n) = Sigma(dp(n))
    ///                  a~f
    ///
    /// ** 狀態壓縮：(a,b,c,d,e,f) --> 若開好幾維陣列將很難維護
    /// 考慮變成哈希表 <K,V> = <(a,b,c,d,e,f)按鍵使用狀態,dp(n) = 組合數>
    /// 由於26 = a+b+c+d+e+f 可以把 再用五個加號分隔 總共是31個記號，可以用i32做紀錄
    /// a~f的個數用0表示 ， +號 用1表示
    /// 舉例：
    ///  10111000001000000000010000000000 --->表示為 a=10, b=10, c=5, d=0, e=0, f=1,
    ///
    /// 但這方法最後有點慢
    fn nexts_n(bit: usize, permutations: i64, mut k: i32) -> Vec<(usize, i64)> {
        let mut ans_permutations: Vec<(usize, i64)> = Vec::new();
        let mut calculator: usize = bit;
        let mut total_digits: i64 = 0;
        let mut digits: i64 = 0;
        while k > 0 {
            // println!("nexts_n,{},{},{}", bit, permutations, k);
            if calculator & 1 == 0 {
                digits += 1;
            } else {
                if digits > 0 {
                    let x = bit ^ (0b_11 << (total_digits - 1));
                    //     println!("new_bit,{},{}", x, digits * permutations);
                    ans_permutations.push((x, (digits * permutations) % Self::VAL_MOD));
                    digits = 0;
                }
                k -= 1;
            }
            total_digits += 1;
            calculator = calculator >> 1;
        }

        return ans_permutations;
    }

    pub fn keyboard_bit_slow(k: i32, n: i32) -> i64 {
        let n_usize = n as usize;
        let mut dp_index_search: Vec<Vec<usize>> = vec![Vec::new(); n_usize + 1]; // 快速查詢 n = x 的時候有哪些usize
        let mut dp_hashmap: HashMap<usize, i64> = HashMap::new(); // usize 對應的
        let state: usize = 0b_11111100000000000000000000000000;
        dp_hashmap.insert(state, 1);
        dp_index_search[0].push(state);
        for i in 0..n_usize {
            for j in 0..dp_index_search[i].len() {
                let s = Self::nexts_n(
                    dp_index_search[i][j],
                    *dp_hashmap.get(&dp_index_search[i][j]).unwrap_or(&0),
                    k,
                );
                for (new_usize, new_permutations) in s.iter() {
                    if !dp_hashmap.contains_key(new_usize) {
                        dp_index_search[i + 1].push(*new_usize);
                    }
                    *dp_hashmap.entry(*new_usize).or_insert(0) += new_permutations;
                }
            }
        }
        // 加總計算dp(n)的結果
        let mut ans: i64 = 0;
        for &i in dp_index_search[n_usize].iter() {
            ans += dp_hashmap.get(&i).unwrap_or(&0);
            ans %= Self::VAL_MOD;
        }

        return ans as i64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(Solution::keyboard(1, 1), 26);
        assert_eq!(Solution::keyboard(1, 2), 650);
        assert_eq!(Solution::keyboard(3, 3), 17_576);
        assert_eq!(Solution::keyboard(2, 4), 454_350);
    }
    #[test]
    fn test_case() {
        assert_eq!(Solution::keyboard(5, 130), 735_365_374);
    }
}
