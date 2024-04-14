/**
41. 缺失的第一个正数

给你一个未排序的整数数组 nums ，请你找出其中没有出现的最小的正整数。

请你实现时间复杂度为 O(n) 并且只使用常数级别额外空间的解决方案。
提示：

https://leetcode.cn/problems/first-missing-positive/description/
*/
pub struct Solution;
// 如果做手動排序 整理nums 把數字(i+1)放到下標i的位置
// 第一個對應位置沒有出現該數字的就是缺失
// 可是排序需要O(N*LOG N)的時間
// 我們可以使用把該下標位置翻成負號當作出現過了
// 原本是負號的數字根本不影響答案，把它們一開始都變成(n+1)
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        for i in 0..n as usize {
            if nums[i] <= 0 {
                nums[i] = n + 1;
            }
        }

        for i in 0..n as usize {
            let mut idx = nums[i];
            if idx < 0 {
                idx *= -1;
            }
            if idx <= n && idx > 0 {
                if nums[idx as usize - 1] > 0 {
                    nums[idx as usize - 1] *= -1;
                }
            }
        }

        for i in 0..n as usize {
            if nums[i] > 0 {
                return i as i32 + 1;
            }
        }
        n + 1
    }
}
