/**
406. 根据身高重建队列

假设有打乱顺序的一群人站成一个队列，数组 people 表示队列中一些人的属性（不一定按顺序）。每个 people[i] = [hi, ki] 表示第 i 个人的身高为 hi ，前面 正好 有 ki 个身高大于或等于 hi 的人。

请你重新构造并返回输入数组 people 所表示的队列。返回的队列应该格式化为数组 queue ，其中 queue[j] = [hj, kj] 是队列中第 j 个人的属性（queue[0] 是排在队列前面的人）。

https://leetcode.cn/problems/queue-reconstruction-by-height/description/
*/
pub struct Solution;
impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable_by(|a, b| {
            if a[0] == b[0] {
                return b[1].cmp(&a[1]);
            };
            a[0].cmp(&b[0])
        });

        let n = people.len();
        let mut ans = vec![vec![]; n];
        for person in people.iter() {
            let mut space = person[1] + 1;
            for i in 0..n {
                if ans[i].is_empty() {
                    space -= 1;
                    if space == 0 {
                        ans[i] = person.clone();
                    }
                }
            }
        }

        ans
    }
}
