/**
75. 颜色分类

给定一个包含红色、白色和蓝色、共 n 个元素的数组 nums ，原地对它们进行排序，使得相同颜色的元素相邻，并按照红色、白色、蓝色顺序排列。

我们使用整数 0、 1 和 2 分别表示红色、白色和蓝色。

必须在不使用库内置的 sort 函数的情况下解决这个问题。

https://leetcode.cn/problems/sort-colors/description/
*/
pub struct Solution;

/// 雙指針紀錄0跟1現在的位置，1的指針在遇到數字是0跟1的時候都要往前一步
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut ptr_0 = 0 as usize;
        let mut ptr_1 = 0 as usize;
        for i in 0..nums.len() {
            let x = nums[i];
            nums[i] = 2;
            if x <= 1 {
                nums[ptr_1] = 1;
                ptr_1 += 1;
            }
            if x == 0 {
                nums[ptr_0] = 0;
                ptr_0 += 1;
            }
        }
    }
}
