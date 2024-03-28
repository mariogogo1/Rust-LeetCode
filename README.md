# Rust-LeetCode

**與編譯器鬥法，其樂無窮。**  

# 目的  
Rust學習紀錄並分享整理。  
Rust算是年輕語言，當前Rust的刷題練習在網路上還算是少的(2024/03)，分享並記錄自己的學習過程。  
部分難題會寫上解題思路  

# 題目目錄  
| No.  | Title                        | File           | Algorithm          | Remark                 |
| ---- | ---------------------------- | -------------- | ------------------ | ---------------------- |
| 0001 | 两数之和                     | easy           | Hashmap            |                        |
| 0003 | 无重复字符的最长子串         | medium_string  | DP/Hashmap         |                        |
| 0004 | **寻找两个正序数组的中位数** | hard           | Binary search      | unimplement            |
| 0005 | **最长回文子串**             | medium_string  | Manacher           | unimplement            |
| 0009 | 回文数                       | easy           |                    |                        |
| 0011 | 盛最多水的容器               | medium_other   | Two pointers       |                        |
| 0020 | 有效的括号                   | stack_and_head | Stack              |                        |
| 0032 | 最长有效括号                 | stack_and_head | Stack              |                        |
| 0042 | 接雨水                       | hard           | Two pointers       |                        |
| 0050 | Pow(x, n)                    | medium_math    | Divide and conquer |                        |
| 0051 | N 皇后                       | hard           | Traceback          |                        |
| 0052 | N 皇后 II                    | hard           | Traceback          |                        |
| 0069 | x 的平方根                   | easy           | Binary search      |                        |
| 0084 | 柱状图中最大的矩形           | stack_and_head | Stack              |                        |
| 0085 | 最大矩形                     | stack_and_head | Stack              |                        |
| 0155 | 最小栈                       | construct      | Stack              |                        |
| 0225 | 用队列实现栈                 | construct      | Stack              |                        |
| 0227 | 计算器                       | medium_other   |                    | update 同面试题 16.26. |
| 0232 | 用栈实现队列                 | construct      | Stack              |                        |
| 0238 | 除自身以外数组的乘积         | medium_other   | Prefix sum         |                        |
| 0321 | 拼接最大数                   | stack_and_head | Stack              | update                 |
| 0372 | 超级次方                     | medium_math    | Divide and conquer |                        |
| 0402 | 移掉 K 位数字                | stack_and_head | Stack              |                        |
| 0403 | 青蛙过河                     | hard           | DP                 | update                 |
| 0456 | 132 模式                     | stack_and_head | Stack              | update                 |
| 0458 | 可怜的小猪                   | hard           |                    | update                 |
| 0496 | 下一个更大元素 I             | stack_and_head | Stack              |                        |
| 0498 | 对角线遍历                   | medium_other   |                    |                        |
| 0503 | 下一个更大元素 II            | stack_and_head | Stack              |                        |
| 0504 | 七进制数                     | easy           |                    |                        |
| 0507 | 完美数                       | easy           |                    |                        |
| 0556 | 下一个更大元素 III           | stack_and_head | Stack              |                        |
| 0647 | **回文子串**                 | medium_string  | Manacher           | unimplement            |
| 0650 | 两个键的键盘                 | medium_dp      | DP                 | update                 |
| 0670 | 最大交换                     | stack_and_head |                    |                        |
| 0704 | 二分查找                     | easy           | Binary search      |                        |
| 0768 | 最多能完成排序的块 II        | stack_and_head | Stack              |                        |
| 0769 | 最多能完成排序的块           | medium_other   | Greedy             |                        |
| 0793 | 阶乘函数后 K 个零            | hard           | Binary search      | update                 |
| 0907 | 子数组的最小值之和           | stack_and_head | Stack              |                        |
| 0972 | 相等的有理数                 | hard           |                    | update                 |
| 1052 | 爱生气的书店老板             | medium_other   | Sliding window     |                        |
| 1223 | 掷骰子模拟                   | hard           | DP                 | update                 |
| 1363 | 形成三的最大倍数             | hard           |                    | update                 |
| 1402 | 做菜顺序                     | hard           | Prefix sum         | update                 |
| 1424 | 对角线遍历 II                | medium_other   |                    |                        |
| 1463 | 摘樱桃 II                    | hard           | DP                 | update                 |
| 1492 | n 的第 k 个因子              | medium_math    |                    |                        |
| 1553 | 吃掉 N 个橘子的最少天数      | hard           | DP                 | update                 |
| 1739 | 放置盒子                     | hard           | Binary search      | update                 |
| 1856 | 子数组最小乘积的最大值       | stack_and_head | Stack              |                        |
| 1969 | 数组元素的最小非零乘积       | medium_math    | Divide and conquer |                        |
| 2104 | 子数组范围和                 | stack_and_head | Stack              |                        |
| 2281 | 巫师的总力量和               | stack_and_head | Stack              |                        |
| 2334 | 元素值大于变化阈值的子数组   | stack_and_head | Stack              |                        |
| 2454 | 下一个更大元素 IV            | stack_and_head | Stack              |                        |
| 2498 | 青蛙过河 II                  | medium_other   | Greedy             | update                 |
| 2561 | 重排水果                     | hard           | Greedy  /Hashmap   | update                 |
| 2671 | 频率跟踪器                   | construct      |                    |                        |
| 2681 | 英雄的力量                   | hard           |                    | update                 |
| 2709 | **最大公约数遍历**           | hard           | Union Find         | unimplement            |
| 2865 | 美丽塔 I                     | stack_and_head | Stack/DP           |                        |
| 2866 | 美丽塔 II                    | stack_and_head | Stack/DP           |                        |
| 2899 | 上一个遍历的整数             | easy           |                    |                        |
| 2924 | 找到冠军 II                  | medium_graph   |                    |                        |
| 2929 | 给小朋友们分糖果 II          | medium_math    | Combination        |                        |



# 學習資料  
並查集算法：https://zhuanlan.zhihu.com/p/93647900  
並查集算法：https://www.cnblogs.com/onlyblues/p/14668087.html  

# 待完成事項  
RUST 閉包  
RUST重寫已完成的GOLANG題目 (400)  
紀錄 DEBUG 環境  
MD 編輯  