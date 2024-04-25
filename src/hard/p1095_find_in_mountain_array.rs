/**
1095. 山脉数组中查找目标值

（这是一个 交互式问题 ）

给你一个 山脉数组 mountainArr，请你返回能够使得 mountainArr.get(index) 等于 target 最小 的下标 index 值。

如果不存在这样的下标 index，就请返回 -1。



何为山脉数组？如果数组 A 是一个山脉数组的话，那它满足如下条件：

首先，A.length >= 3

其次，在 0 < i < A.length - 1 条件下，存在 i 使得：

A[0] < A[1] < ... A[i-1] < A[i]
A[i] > A[i+1] > ... > A[A.length - 1]


你将 不能直接访问该山脉数组，必须通过 MountainArray 接口来获取数据：

MountainArray.get(k) - 会返回数组中索引为k 的元素（下标从 0 开始）
MountainArray.length() - 会返回该数组的长度


注意：

对 MountainArray.get 发起超过 100 次调用的提交将被视为错误答案。此外，任何试图规避判题系统的解决方案都将会导致比赛资格被取消。

https://leetcode.cn/problems/find-in-mountain-array/description/
*/
pub struct Solution;
// 先用二分查找找出山頂 之後可以很輕鬆地使用二分查找對遞增跟遞減數列查出target的位置
impl Solution {
    pub fn find_in_mountain_array(target: i32, mountain_arr: &MountainArray) -> i32 {
        //先找出山頂
        let mut r = mountain_arr.length() - 1;
        let mut find_r = r; //之後從遞減數列找TARGET
        let mut max_idx = 0; //山頂

        while max_idx < r {
            let m = max_idx + (r - max_idx) / 2;
            if mountain_arr.get(m) > mountain_arr.get(m + 1) {
                r = m;
            } else {
                max_idx = m + 1;
            }
        }

        let mut l = 0;
        let mut max_idx_l = max_idx;
        // 左邊遞增數列
        while l <= max_idx_l {
            let m = l + (max_idx_l - l) / 2;
            let val = mountain_arr.get(m);

            if val == target {
                return m;
            } else if val > target {
                max_idx_l = m - 1;
            } else {
                l = m + 1;
            }
        }
        // 右邊遞增減數列
        while max_idx <= find_r {
            let m = max_idx + (find_r - max_idx) / 2;
            let val = mountain_arr.get(m);

            if val == target {
                return m;
            } else if val < target {
                find_r = m - 1;
            } else {
                max_idx = m + 1;
            }
        }

        -1
    }
}
