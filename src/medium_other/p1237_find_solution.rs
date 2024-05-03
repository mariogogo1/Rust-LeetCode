/**
1237. 找出给定方程的正整数解

给你一个函数  f(x, y) 和一个目标结果 z，函数公式未知，请你计算方程 f(x,y) == z 所有可能的正整数 数对 x 和 y。满足条件的结果数对可以按任意顺序返回。

尽管函数的具体式子未知，但它是单调递增函数，也就是说：

f(x, y) < f(x + 1, y)
f(x, y) < f(x, y + 1)
函数接口定义如下：

interface CustomFunction {
public:
  // Returns some positive integer f(x, y) for two positive integers x and y based on a formula.
  int f(int x, int y);
};
你的解决方案将按如下规则进行评判：

判题程序有一个由 CustomFunction 的 9 种实现组成的列表，以及一种为特定的 z 生成所有有效数对的答案的方法。
判题程序接受两个输入：function_id（决定使用哪种实现测试你的代码）以及目标结果 z 。
判题程序将会调用你实现的 findSolution 并将你的结果与答案进行比较。
如果你的结果与答案相符，那么解决方案将被视作正确答案，即 Accepted 。

https://leetcode.cn/problems/find-positive-integer-solution-for-a-given-equation/description/
*/
pub struct Solution;
impl Solution {
    fn binary_search(
        customfunction: &CustomFunction,
        z: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if x1 > x2 || y1 > y2 {
            return;
        }
        let m_x = (x1 + x2) / 2;
        let m_y = (y1 + y2) / 2;

        let s = customfunction.f(m_x, m_y);
        if s >= z {
            if s == z {
                ans.push(vec![m_x, m_y]);
            }
            Self::binary_search(customfunction, z, x1, y1, m_x - 1, y2, ans);
            Self::binary_search(customfunction, z, m_x, y1, x2, m_y - 1, ans);
        } else {
            Self::binary_search(customfunction, z, m_x + 1, y1, x2, m_y, ans);
            Self::binary_search(customfunction, z, x1, m_y + 1, x2, y2, ans);
        }
    }
    pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();

        Self::binary_search(customfunction, z, 1, 1, 1000, 1000, &mut ans);

        ans
    }
}
