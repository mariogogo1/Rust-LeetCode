/**
384. 打乱数组

给你一个整数数组 nums ，设计算法来打乱一个没有重复元素的数组。打乱后，数组的所有排列应该是 等可能 的。

实现 Solution class:

Solution(int[] nums) 使用整数数组 nums 初始化对象
int[] reset() 重设数组到它的初始状态并返回
int[] shuffle() 返回数组随机打乱后的结果

https://leetcode.cn/problems/shuffle-an-array/description/
*/
// 舉例說明： 長度是4的數組 [a,b,c,d]
// 隨機從4個數字抽取一個決定最後一個位置的數字 ，最後一個位置選到每個數字的機率 = 1/4
// 再來
// 從前3個數字抽取一個數字決定下標2位置的數字， 下標二的位置選到每個數字的機率 = 3/4 * 1/3 = 1/4
// ....以此類推
use rand::Rng;
struct Solution {
    rng: rand::rngs::ThreadRng,
    nums: Vec<i32>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let mut solution = Solution {
            rng: rand::thread_rng(),
            nums,
        };

        solution
    }

    fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }

    fn shuffle(&mut self) -> Vec<i32> {
        let mut shuffle_nums = self.nums.clone();

        for i in (0..self.nums.len()).rev() {
            let rand_idx = self.rng.gen_range(0..(i + 1));
            shuffle_nums.swap(i, rand_idx);
        }

        shuffle_nums.clone()
    }
}
