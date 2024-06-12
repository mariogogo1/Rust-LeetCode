/**
307. 区域和检索 - 数组可修改

给你一个数组 nums ，请你完成两类查询。

其中一类查询要求 更新 数组 nums 下标对应的值
另一类查询要求返回数组 nums 中索引 left 和索引 right 之间（ 包含 ）的nums元素的 和 ，其中 left <= right
实现 NumArray 类：

NumArray(int[] nums) 用整数数组 nums 初始化对象
void update(int index, int val) 将 nums[index] 的值 更新 为 val
int sumRange(int left, int right) 返回数组 nums 中索引 left 和索引 right 之间（ 包含 ）的nums元素的 和 （即，nums[left] + nums[left + 1], ..., nums[right]）

https://leetcode.cn/problems/range-sum-query-mutable/description/
*/
struct NumArray {
    nums: Vec<i32>,
    seg_tree_array: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut res: NumArray = NumArray {
            nums: vec![0; n],
            seg_tree_array: vec![0; n + 1], // nums[0] 會加總在 seg_tree_array[1]
        };
        for i in 0..n {
            res.update(i as i32, nums[i]);
        }

        res
    }

    fn update(&mut self, index: i32, val: i32) {
        let idx = index as usize;
        let delta = val - self.nums[idx];
        self.nums[idx] = val;
        let mut x = index as usize + 1;

        while x < self.seg_tree_array.len() {
            self.seg_tree_array[x] += delta;
            x += (x as i32 & -(x as i32)) as usize;
        }
    }

    fn sum_segment(&self, index: i32) -> i32 {
        let mut ans = 0;
        let mut x = index as usize;
        while x > 0 {
            ans += self.seg_tree_array[x];
            x &= x - 1;
        }
        ans
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sum_segment(right + 1) - self.sum_segment(left)
    }
}
