/**

458. 可怜的小猪

有 buckets 桶液体，其中 正好有一桶 含有毒药，其余装的都是水。它们从外观看起来都一样。为了弄清楚哪只水桶含有毒药，你可以喂一些猪喝，通过观察猪是否会死进行判断。不幸的是，你只有 minutesToTest 分钟时间来确定哪桶液体是有毒的。

喂猪的规则如下：

选择若干活猪进行喂养
可以允许小猪同时饮用任意数量的桶中的水，并且该过程不需要时间。
小猪喝完水后，必须有 minutesToDie 分钟的冷却时间。在这段时间里，你只能观察，而不允许继续喂猪。
过了 minutesToDie 分钟后，所有喝到毒药的猪都会死去，其他所有猪都会活下来。
重复这一过程，直到时间用完。
给你桶的数目 buckets ，minutesToDie 和 minutesToTest ，返回 在规定时间内判断哪个桶有毒所需的 最小 猪数 。

提示：

1 <= buckets <= 1000
1 <= minutesToDie <= minutesToTest <= 100

https://leetcode.cn/problems/poor-pigs/description/
*/
pub struct Solution;
/// 本題想法：
/// 將豬的個數看成維度 =n
/// 時間/毒發時間 + 1 = 一隻豬可以測試幾次 = 每一維度的長度 = y
/// buckets <= y ^ n
///
/// 案例說明：
/// 兩隻豬，100桶水，時間足以測試10次，我們可以將100桶水排成10*10的正方形平面
/// 小豬A 每次測試喝10桶水，從列1~列10，
/// 小豬B 每次測試喝10桶水，從行1~行10，
/// 當小豬A跟B死亡時就可以知道有毒水桶的座標!!
use std::f64;

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let x: f64 = (minutes_to_test / minutes_to_die + 1) as f64;
        let n: f64 = (buckets as f64 - 0.05).log(x); // 精度問題 0.05調整
        n.ceil() as i32
    }
}
