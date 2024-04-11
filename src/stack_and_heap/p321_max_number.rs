/**
321. 拼接最大数

给你两个整数数组 nums1 和 nums2，它们的长度分别为 m 和 n。数组 nums1 和 nums2 分别代表两个数各位上的数字。同时你也会得到一个整数 k。

请你利用这两个数组中的数字中创建一个长度为 k <= m + n 的最大数，在这个必须保留来自同一数组的数字的相对顺序。

返回代表答案的长度为 k 的数组。

提示：

m == nums1.length
n == nums2.length
1 <= m, n <= 500
0 <= nums1[i], nums2[i] <= 9
1 <= k <= m + n

https://leetcode.cn/problems/create-maximum-number/description/
*/

pub struct Solution;
//321. 拼接最大数
/// 測試數字範圍不大，直接做照題意做
/// 這題思考容易，但是寫起來的細節很多。
/// 需要參考402 用單調棧排出每個arr，找出刪除x個位數後，排出不改變順序的最大數字。
impl Solution {
    fn find_max_arr(arr: &Vec<i32>, mut k: usize) -> Vec<i32> {
        let mut ans = Vec::new();
        // println!("{:?},{}", arr, k);
        for &i in arr.iter() {
            while let Some(&top) = ans.last() {
                if top < i && k > 0 {
                    k -= 1;
                    ans.pop();
                } else {
                    break;
                }
            }
            ans.push(i);
        }
        while k > 0 {
            k -= 1;
            ans.pop();
        }
        // println!("{:?},{}", ans, k);
        return ans;
    }

    fn combine_arrs(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut ptr1: usize = 0;
        let mut ptr2: usize = 0;
        while ptr1 < arr1.len() && ptr2 < arr2.len() {
            //println!("{},{}", ptr1, ptr2);
            let mut x = 0;
            let mut first_big = true;
            while ptr1 + x < arr1.len() && ptr2 + x < arr2.len() {
                if arr1[ptr1 + x] == arr2[ptr2 + x] {
                    x += 1;
                } else {
                    break;
                }
            }
            if ptr1 + x == arr1.len() {
                ans.push(arr2[ptr2]);
                ptr2 += 1;
            } else if ptr2 + x == arr2.len() {
                ans.push(arr1[ptr1]);
                ptr1 += 1;
            } else if arr1[ptr1 + x] < arr2[ptr2 + x] {
                ans.push(arr2[ptr2]);
                ptr2 += 1;
            } else {
                ans.push(arr1[ptr1]);
                ptr1 += 1;
            }
        }

        if ptr1 == arr1.len() {
            while ptr2 < arr2.len() {
                ans.push(arr2[ptr2]);
                ptr2 += 1;
            }
        } else if ptr2 == arr2.len() {
            while ptr1 < arr1.len() {
                ans.push(arr1[ptr1]);
                ptr1 += 1;
            }
        }

        return ans;
    }
    fn compare_max_arr<'a>(arr1: &'a Vec<i32>, arr2: &'a Vec<i32>) -> &'a Vec<i32> {
        let n = arr1.len();
        for i in 0..n {
            if arr1[i] < arr2[i] {
                return arr2;
            } else if arr1[i] > arr2[i] {
                return arr1;
            }
        }
        return arr1;
    }

    pub fn max_number(arr1: Vec<i32>, arr2: Vec<i32>, k: i32) -> Vec<i32> {
        let k_usize = k as usize;
        let n = arr1.len();
        let m = arr2.len();
        let mut ans = vec![0; k_usize];
        let mut temp_ans;
        for i in 0..=k_usize {
            if i <= n && (k_usize - i) <= m {
                //println!("{},{}", n - i, m + i - k_usize);
                temp_ans = Self::combine_arrs(
                    Self::find_max_arr(&arr1, n - i),
                    Self::find_max_arr(&arr2, m + i - k_usize),
                );
                // println!("{:?}", temp_ans);
                ans = Self::compare_max_arr(&ans, &temp_ans).to_vec();
            }
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            Solution::max_number(vec![6, 7], vec![6, 0, 4], 5),
            vec![6, 7, 6, 0, 4]
        );

        assert_eq!(
            Solution::max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5),
            vec![9, 8, 6, 5, 3]
        );
        assert_eq!(
            Solution::max_number(vec![3, 9], vec![8, 9], 3),
            vec![9, 8, 9]
        );
    }
    #[test]
    fn test_case() {
        assert_eq!(
            Solution::max_number(
                vec![5, 0, 2, 1, 0, 1, 0, 3, 9, 1, 2, 8, 0, 9, 8, 1, 4, 7, 3],
                vec![7, 6, 7, 1, 0, 1, 0, 5, 6, 0, 5, 0],
                31
            ),
            vec![
                7, 6, 7, 5, 1, 0, 2, 1, 0, 1, 0, 5, 6, 0, 5, 0, 1, 0, 3, 9, 1, 2, 8, 0, 9, 8, 1, 4,
                7, 3, 0
            ]
        );
    }
}
