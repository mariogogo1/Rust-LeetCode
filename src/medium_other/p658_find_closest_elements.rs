/**
658. 找到 K 个最接近的元素

给定一个 排序好 的数组 arr ，两个整数 k 和 x ，从数组中找到最靠近 x（两数之差最小）的 k 个数。返回的结果必须要是按升序排好的。

整数 a 比整数 b 更接近 x 需要满足：

|a - x| < |b - x| 或者
|a - x| == |b - x| 且 a < b

https://leetcode.cn/problems/find-k-closest-elements/description/
*/
pub struct Solution;
impl Solution {
    fn abs(a: i32, b: i32) -> i32 {
        if a > b {
            return a - b;
        }
        return b - a;
    }

    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        if arr.len() == k {
            return arr;
        }
        let mut ptr: usize = 0;
        while ptr + k < arr.len() {
            if Self::abs(arr[ptr + k], x) > Self::abs(arr[ptr], x) {
                break;
            } else if Self::abs(arr[ptr + k], x) == Self::abs(arr[ptr], x) {
                if arr[ptr + k] > arr[ptr] {
                    break;
                }
            }

            ptr += 1;
        }

        arr[ptr..ptr + k].to_vec()
    }

    pub fn find_closest_elements_basic(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut pairs: Vec<(i32, usize)> = Vec::new();
        for i in 0..arr.len() {
            if arr[i] >= x {
                pairs.push((arr[i] - x, i));
            } else {
                pairs.push((x - arr[i], i));
            }
        }

        pairs.sort_unstable_by(|a, b| {
            if a.0 == b.0 {
                return a.1.cmp(&b.1);
            }
            return a.0.cmp(&b.0);
        });

        let mut ans = Vec::new();
        for i in 0..k as usize {
            ans.push(arr[pairs[i].1]);
        }
        ans.sort_unstable();
        ans
    }
}
