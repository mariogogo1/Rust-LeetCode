/**
630. 课程表 III

这里有 n 门不同的在线课程，按从 1 到 n 编号。给你一个数组 courses ，其中 courses[i] = [durationi, lastDayi] 表示第 i 门课将会 持续 上 durationi 天课，并且必须在不晚于 lastDayi 的时候完成。

你的学期从第 1 天开始。且不能同时修读两门及两门以上的课程。

返回你最多可以修读的课程数目。

https://leetcode.cn/problems/course-schedule-iii/description/
*/
use std::collections::BinaryHeap;
impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let mut time_heap: BinaryHeap<Vec<i32>> = BinaryHeap::new();
        let mut total_time = 0;

        for i in 0..courses.len() {
            if total_time + courses[i][0] <= courses[i][1] {
                total_time += courses[i][0];
                time_heap.push(courses[i].clone());
            } else {
                if let Some(the_course) = time_heap.peek() {
                    if the_course[0] > courses[i][0] {
                        total_time -= the_course[0];
                        time_heap.pop();
                        total_time += courses[i][0];
                        time_heap.push(courses[i].clone());
                    }
                }
            }
        }

        time_heap.len() as i32
    }
}
