# Rust-LeetCode

**與編譯器鬥法，其樂無窮。**  

# 目的  
Rust學習紀錄並分享整理。  
Rust算是年輕語言，當前Rust的刷題練習在網路上還算是少的(2024/03)，分享並記錄自己的學習過程。  
部分難題會寫上解題思路  

# 題目目錄  
| No.    | Title                           | File              | Algorithm            | Remark             |
| ------ | ------------------------------- | ----------------- | -------------------- | ------------------ |
| 0001   | 两数之和                        | easy              | Hashmap              |                    |
| 0003   | 无重复字符的最长子串            | medium_string     | DP/Hashmap           |                    |
| 0004   | **寻找两个正序数组的中位数**    | hard              | Binary search        | unimplement        |
| 0005   | **最长回文子串**                | medium_string     | Manacher             | unimplement        |
| 0009   | 回文数                          | easy              |                      |                    |
| 0011   | 盛最多水的容器                  | medium_other      | Two pointers         |                    |
| 0020   | 有效的括号                      | stack_and_head    | Stack                |                    |
| 0021   | 合并两个有序链表                | listnode          | Recursive            |                    |
| 0022   | 括号生成                        | medium_dfs_or_bfs |                      | UPDATE             |
| 0032   | 最长有效括号                    | stack_and_head    | Stack                |                    |
| 0042   | 接雨水                          | hard              | Two pointers         |                    |
| 0050   | Pow(x, n)                       | medium_math       | Divide and conquer   |                    |
| 0051   | N 皇后                          | hard              | Traceback            |                    |
| 0052   | N 皇后 II                       | hard              | Traceback            |                    |
| 0053   | 最大子数组和                    | subarray          | Kadane               |                    |
| 0069   | x 的平方根                      | easy              | Binary search        |                    |
| 0084   | 柱状图中最大的矩形              | stack_and_head    | Stack                |                    |
| 0085   | 最大矩形                        | stack_and_head    | Stack                |                    |
| 0121   | 买卖股票的最佳时机              | series\stock      | DP                   | UPDATE             |
| 0122   | 买卖股票的最佳时机 II           | series\stock      | DP                   | UPDATE             |
| 0123   | 买卖股票的最佳时机 III          | series\stock      | DP                   | UPDATE             |
| 0152   | 乘积最大子数组                  | subarray          |                      |                    |
| 0155   | 最小栈                          | construct         | Stack                |                    |
| 0169   | 多数元素                        | easy              | Boyer-Moore 投票算法 | UPDATE             |
| 0188   | 买卖股票的最佳时机 IV           | series\stock      | DP                   | UPDATE             |
| 0202   | 快乐数                          | easy              | Hashmap              | UPDATE             |
| 0204   | 计数质数                        | medium_math       |                      | UPDATE             |
| 0209   | 长度最小的子数组                | subarray          |                      |                    |
| 0217   | 存在重复元素                    | easy              |                      | UPDATE             |
| 0219   | 存在重复元素 II                 | easy              |                      | UPDATE             |
| 0220   | 存在重复元素 III                | hard              | Bucket Sort          | UPDATE             |
| 0225   | 用队列实现栈                    | construct         | Stack                |                    |
| 0227   | 计算器                          | medium_other      |                      | 同面试题 16.26.    |
| 0229   | 多数元素 II                     | medium_other      | Hashmap              | UPDATE             |
| 0232   | 用栈实现队列                    | construct         | Stack                |                    |
| 0238   | 除自身以外数组的乘积            | medium_other      | Prefix sum           |                    |
| 0258   | 各位相加                        | easy              |                      | UPDATE             |
| 0263   | 丑数                            | easy              |                      | UPDATE             |
| 0264   | 丑数 II                         | medium_math       | Multiple Pointers    | UPDATE             |
| 0279   | 完全平方数                      | medium_dp         | DP                   | UPDATE             |
| 0309   | 买卖股票的最佳时机含冷冻期      | series\stock      | DP                   | UPDATE             |
| 0321   | 拼接最大数                      | stack_and_head    | Stack                |                    |
| 0367   | 有效的完全平方数                | easy              | Binary search        | UPDATE             |
| 0372   | 超级次方                        | medium_math       | Divide and conquer   |                    |
| 0382   | 链表随机节点                    | random_sampling   |                      |                    |
| 0398   | 随机数索引                      | random_sampling   |                      |                    |
| 0402   | 移掉 K 位数字                   | stack_and_head    | Stack                |                    |
| 0403   | 青蛙过河                        | hard              | DP                   |                    |
| 0413   | 等差数列划分                    | subarray          |                      | UPDATE             |
| 0446   | 等差数列划分 II - 子序列        | hard              | DP/Hashmap           | UPDATE             |
| 0454   | 四数相加 II                     | medium_other      | Hashmap              | UPDATE             |
| 0456   | 132 模式                        | stack_and_head    | Stack                |                    |
| 0458   | 可怜的小猪                      | hard              |                      |                    |
| 0478   | 在圆内随机生成点                | random_sampling   |                      |                    |
| 0496   | 下一个更大元素 I                | stack_and_head    | Stack                |                    |
| 0497   | 非重叠矩形中的随机点            | random_sampling   |                      |                    |
| 0498   | 对角线遍历                      | medium_other      |                      |                    |
| 0503   | 下一个更大元素 II               | stack_and_head    | Stack                |                    |
| 0504   | 七进制数                        | easy              |                      |                    |
| 0507   | 完美数                          | easy              |                      |                    |
| 0519   | 随机翻转矩阵                    | random_sampling   |                      |                    |
| 0523   | 连续的子数组和                  | subarray          |                      |                    |
| 0528   | 按权重随机选择                  | random_sampling   |                      |                    |
| 0554   | 砖墙                            | medium_other      | Hashmap              | UPDATE             |
| 0556   | 下一个更大元素 III              | stack_and_head    | Stack                |                    |
| 0560   | 和为 K 的子数组                 | subarray          |                      |                    |
| 0633   | 平方数之和                      | medium_math       | Binary search        | UPDATE             |
| 0647   | **回文子串**                    | medium_string     | Manacher             | unimplement        |
| 0650   | 两个键的键盘                    | medium_dp         | DP                   |                    |
| 0670   | 最大交换                        | stack_and_head    |                      |                    |
| 0704   | 二分查找                        | easy              | Binary search        |                    |
| 0710   | 黑名单中的随机数                | random_sampling   |                      |                    |
| 0713   | 乘积小于 K 的子数组             | subarray          |                      |                    |
| 0714   | 买卖股票的最佳时机含手续费      | series\stock      | DP                   | UPDATE             |
| 0768   | 最多能完成排序的块 II           | stack_and_head    | Stack                |                    |
| 0769   | 最多能完成排序的块              | medium_other      | Greedy               |                    |
| 0793   | 阶乘函数后 K 个零               | hard              | Binary search        |                    |
| 0836   | 矩形重叠                        | easy              |                      | UPDATE             |
| 0887   | 鸡蛋掉落                        | hard              | DP                   | 1884.鸡蛋掉落-两枚 |
| 0901   | 股票价格跨度                    | construct         |                      | UPDATE             |
| 0907   | 子数组的最小值之和              | stack_and_head    | Stack                |                    |
| 0918   | 环形子数组的最大和              | subarray          | Kadane               |                    |
| 0972   | 相等的有理数                    | hard              |                      |                    |
| 0974   | 和可被 K 整除的子数组           | subarray          |                      |                    |
| 0977   | 有序数组的平方                  | easy              |                      | UPDATE             |
| 1052   | 爱生气的书店老板                | medium_other      | Sliding window       |                    |
| 1223   | 掷骰子模拟                      | hard              | DP                   |                    |
| 1224   | 最大相等频率                    | hard              | Hashmap              | UPDATE             |
| 1363   | 形成三的最大倍数                | hard              |                      |                    |
| 1402   | 做菜顺序                        | hard              | Prefix sum           |                    |
| 1424   | 对角线遍历 II                   | medium_other      |                      |                    |
| 1463   | 摘樱桃 II                       | hard              | DP                   |                    |
| 1492   | n 的第 k 个因子                 | medium_math       |                      |                    |
| 1553   | 吃掉 N 个橘子的最少天数         | hard              | DP                   |                    |
| 1739   | 放置盒子                        | hard              | Binary search        |                    |
| 1776   | 车队 II                         | stack_and_head    | Stack                |                    |
| 1793   | 好子数组的最大分数              | stack_and_head    | Stack                |                    |
| 1856   | 子数组最小乘积的最大值          | stack_and_head    | Stack                |                    |
| 1866   | 恰有 K 根木棍可以看到的排列数目 | hard              | DP                   |                    |
| 1969   | 数组元素的最小非零乘积          | medium_math       | Divide and conquer   |                    |
| 1997   | 访问完所有房间的第一天          | medium_dp         | DP                   |                    |
| 2088   | 统计农场中肥沃金字塔的数目      | hard              | DP                   |                    |
| 2104   | 子数组范围和                    | stack_and_head    | Stack                |                    |
| 2280   | 表示一个折线图的最少线段数      | medium_math       |                      | UPDATE             |
| 2281   | 巫师的总力量和                  | stack_and_head    | Stack                |                    |
| 2321   | 拼接数组的最大分数              | subarray          | Kadane               |                    |
| 2334   | 元素值大于变化阈值的子数组      | stack_and_head    | Stack                |                    |
| 2454   | 下一个更大元素 IV               | stack_and_head    | Stack                |                    |
| 2498   | 青蛙过河 II                     | medium_other      | Greedy               |                    |
| 2561   | 重排水果                        | hard              | Greedy  /Hashmap     |                    |
| 2671   | 频率跟踪器                      | construct         |                      |                    |
| 2681   | 英雄的力量                      | hard              |                      |                    |
| 2709   | **最大公约数遍历**              | hard              | Union Find           | unimplement        |
| 2836   | 在传球游戏中最大化函数值        | hard              | DP/Bit manipulation  |                    |
| 2897   | 对数组执行操作使平方和最大      | hard              | DP/Bit manipulation  |                    |
| 2865   | 美丽塔 I                        | stack_and_head    | Stack/DP             |                    |
| 2866   | 美丽塔 II                       | stack_and_head    | Stack/DP             |                    |
| 2899   | 上一个遍历的整数                | easy              |                      |                    |
| 2924   | 找到冠军 II                     | medium_graph      |                      |                    |
| 2929   | 给小朋友们分糖果 II             | medium_math       | Combination          |                    |
| 3082   | 求出所有子序列的能量和          | hard              | DP                   |                    |
| LCP 25 | 古董键盘                        | hard              | DP                   | UPDATE             |



# 學習資料  
並查集算法：https://zhuanlan.zhihu.com/p/93647900  
並查集算法：https://www.cnblogs.com/onlyblues/p/14668087.html 
水塘抽樣：https://zhuanlan.zhihu.com/p/29178293  
 

# 待完成事項  
RUST 閉包  
RUST重寫已完成的GOLANG題目 (400)  
紀錄 DEBUG 環境   
堆HEAP  
线段树  
数位 DP  
LISTNODE  
TREENODE/归并排序  
