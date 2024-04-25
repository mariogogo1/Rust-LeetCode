/**
829. 连续整数求和

给定一个正整数 n，返回 连续正整数满足所有数字之和为 n 的组数 。

https://leetcode.cn/problems/consecutive-numbers-sum/description/
*/
pub struct Solution;

/// 這題用到國中的連續整數求和的性質
///         
///
///  如果一個數N可以被連續X個整數相加 那麼 [N - X*(X-1)/2 ] 可以被 X給整除
///
///           O
///       O   O
///   O   O   O
///   O   O   O
///   O   O   O
///   O   O   O  .....  
///   O   O   O
///   O   O   O
///   O   O   O
///   T  T+1  T+2      T+(X-1)  <=  一共X個連續整數
///  
///  把多餘的 +1 +2 +3 ... +(X-1) 扣掉
///  = [N - X*(X-1)/2 ]
///  = T * X  --> 一定可以被X整除

impl Solution {
    pub fn consecutive_numbers_sum(mut n: i32) -> i32 {
        let mut ans = 0;
        for i in 0..n {
            n -= i;
            if n < i {
                break;
            }
            if n % (i + 1) == 0 {
                ans += 1;
            }
        }
        ans
    }
}
