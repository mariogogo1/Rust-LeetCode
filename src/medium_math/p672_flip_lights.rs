/**
319. 灯泡开关

初始时有 n 个灯泡处于关闭状态。第一轮，你将会打开所有灯泡。接下来的第二轮，你将会每两个灯泡关闭第二个。

第三轮，你每三个灯泡就切换第三个灯泡的开关（即，打开变关闭，关闭变打开）。第 i 轮，你每 i 个灯泡就切换第 i 个灯泡的开关。直到第 n 轮，你只需要切换最后一个灯泡的开关。

找出并返回 n 轮后有多少个亮着的灯泡。

https://leetcode.cn/problems/bulb-switcher/description/
*/

pub struct Solution;

// 當第X個燈泡經歷第"因數"輪的時候，燈泡的開關會改變
// 所以當X有奇數個因數的時候，燈泡最終會是亮的
// X 有奇數的因數 -> X 為完全平方數
impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64 + 0.5).sqrt() as i32
    }
}
