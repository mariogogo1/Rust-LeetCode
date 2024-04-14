/**
1306. 跳跃游戏 III

这里有一个非负整数数组 arr，你最开始位于该数组的起始下标 start 处。当你位于下标 i 处时，你可以跳到 i + arr[i] 或者 i - arr[i]。

请你判断自己是否能够跳到对应元素值为 0 的 任一 下标处。

注意，不管是什么情况下，你都无法跳到数组之外。

https://leetcode.cn/problems/jump-game-iii/description/
*/

// BFS
impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let n = arr.len();
        let mut visited = vec![false; n];
        let mut arr_vec = vec![start];

        while !arr_vec.is_empty() {
            let go_vec = arr_vec.clone();
            arr_vec.clear();
            for node in go_vec {
                let idx = node as usize;
                visited[idx] = true;
                if arr[idx] == 0 {
                    return true;
                }
                if node >= arr[idx] && !visited[idx - arr[idx] as usize] {
                    arr_vec.push(node - arr[idx]);
                }
                if idx + (arr[idx] as usize) < n && !visited[idx + arr[idx] as usize] {
                    arr_vec.push(node + arr[idx]);
                }
            }
        }

        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {}
    #[test]
    fn test_case() {}
}
