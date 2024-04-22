/**
1109. 航班预订统计

这里有 n 个航班，它们分别从 1 到 n 进行编号。

有一份航班预订表 bookings ，表中第 i 条预订记录 bookings[i] = [firsti, lasti, seatsi] 意味着在从 firsti 到 lasti （包含 firsti 和 lasti ）的 每个航班 上预订了 seatsi 个座位。

请你返回一个长度为 n 的数组 answer，里面的元素是每个航班预定的座位总数。


https://leetcode.cn/problems/corporate-flight-bookings/description/
*/

pub struct Solution;
impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut difference = vec![0; n as usize];
        for i in 0..bookings.len() {
            difference[bookings[i][0] as usize - 1] += bookings[i][2];
            if bookings[i][1] < n {
                difference[bookings[i][1] as usize] -= bookings[i][2];
            }
        }

        for i in 1..n as usize {
            difference[i] += difference[i - 1];
        }
        difference
    }
}
