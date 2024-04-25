/**
860. 柠檬水找零

在柠檬水摊上，每一杯柠檬水的售价为 5 美元。顾客排队购买你的产品，（按账单 bills 支付的顺序）一次购买一杯。

每位顾客只买一杯柠檬水，然后向你付 5 美元、10 美元或 20 美元。你必须给每个顾客正确找零，也就是说净交易是每位顾客向你支付 5 美元。

注意，一开始你手头没有任何零钱。

给你一个整数数组 bills ，其中 bills[i] 是第 i 位顾客付的账。如果你能给每位顾客正确找零，返回 true ，否则返回 false 。

https://leetcode.cn/problems/lemonade-change/description/
*/

pub struct Solution;
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut coin_map = vec![0, 0, 0]; // 初始化硬币数量数组，分别代表 5、10、20 的数量
        for &value in bills.iter() {
            match value {
                5 => coin_map[0] += 1,
                10 => {
                    coin_map[1] += 1;
                    coin_map[0] -= 1;
                    if coin_map[0] < 0 {
                        return false;
                    }
                }
                20 => {
                    if coin_map[1] > 0 {
                        coin_map[1] -= 1;
                        coin_map[0] -= 1;
                        if coin_map[0] < 0 {
                            return false;
                        }
                    } else {
                        coin_map[0] -= 3;
                        if coin_map[0] < 0 {
                            return false;
                        }
                    }
                    coin_map[2] += 1;
                }
                _ => return false, // 对于其他面值的账单，返回 false
            }
        }
        true
    }
}
