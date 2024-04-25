/**
4. 寻找两个正序数组的中位数

给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。

算法的时间复杂度应该为 O(log (m+n)) 。

https://leetcode.cn/problems/median-of-two-sorted-arrays/description/
*/
pub struct Solution;
impl Solution {
    fn dfs(pos: usize, nums1: &Vec<i32>, nums2: &Vec<i32>, p1: &mut usize, p2: &mut usize) -> i32 {
        if nums1.len() == *p1 {
            *p2 += pos;
            return nums2[*p2];
        }
        if nums2.len() == *p2 {
            *p1 += pos;
            return nums1[*p1];
        }

        if pos == 0 {
            return nums1[*p1].min(nums2[*p2]);
        }
        let cancel = ((pos + 1) / 2)
            .min(nums1.len() - *p1)
            .min(nums2.len() - *p2);

        if nums1[*p1 + cancel - 1] > nums2[*p2 + cancel - 1] {
            *p2 += cancel
        } else {
            *p1 += cancel
        }

        return Self::dfs(pos - cancel, nums1, nums2, p1, p2);
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n = nums1.len();
        let m = nums2.len();
        let mut p1: usize = 0;
        let mut p2: usize = 0;

        if (n + m) % 2 == 0 {
            return (Self::dfs((n + m) / 2 - 1, &nums1, &nums2, &mut p1, &mut p2) as f64
                + Self::dfs(1, &nums1, &nums2, &mut p1, &mut p2) as f64)
                / 2.0;
        }
        return Self::dfs((n + m) / 2, &nums1, &nums2, &mut p1, &mut p2) as f64;
    }
}
