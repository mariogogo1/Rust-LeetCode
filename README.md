# Rust-LeetCode

**與編譯器鬥法，其樂無窮。**  
Rust學習紀錄並分享整理。  
傳言說Rust的學習曲線很陡峭，分享這些資料希望可以減少大家學習的辛苦。    
Rust算是年輕語言，當前Rust的刷題練習在網路上還算是少的(2024/03)，分享並記錄自己的學習過程。  
部分難題會寫上解題思路  
Notes會記錄常用的語法  

# 題目目錄  
| No.    | Title                           | File               | Algorithm            | Remark             |
| ------ | ------------------------------- | ------------------ | -------------------- | ------------------ |
| 0001   | 两数之和                        | easy               | Hashmap              |                    |
| 0003   | 无重复字符的最长子串            | medium_string      | DP/Hashmap           |                    |
| 0004   | **寻找两个正序数组的中位数**    | hard               | Binary search        | unimplement        |
| 0005   | **最长回文子串**                | medium_string      | Manacher             | unimplement        |
| 0006   | Z 字形变换                      | medium_other       |                      | UPDATE             |
| 0009   | 回文数                          | easy               |                      |                    |
| 0011   | 盛最多水的容器                  | medium_other       | Two pointers         |                    |
| 0015   | 三数之和                        | medium_other       | Two pointers         | UPDATE             |
| 0016   | 最接近的三数之和                | medium_other       | Two pointers         | UPDATE             |
| 0017   | 电话号码的字母组合              | medium_dfs_or_bfs  | DFS/Hashmap          |                    |
| 0018   | 四数之和                        | medium_other       | Two pointers         | UPDATE             |
| 0020   | 有效的括号                      | stack_and_heap     | Stack                |                    |
| 0021   | 合并两个有序链表                | listnode           | Recursive            |                    |
| 0022   | 括号生成                        | medium_dfs_or_bfs  |                      |                    |
| 0024   | 两两交换链表中的节点            | listnode           | Recursive            | UPDATE             |
| 0031   | 下一个排列                      | combine_permute    |                      |                    |
| 0032   | 最长有效括号                    | stack_and_heap     | Stack                |                    |
| 0034   | 查找元素的第一个和最后一个位置  | medium_other       | Binary search        |                    |
| 0035   | 搜索插入位置                    | easy               | Binary search        |                    |
| 0036   | 有效的数独                      | medium_other       |                      | UPDATE             |
| 0039   | 组合总和                        | combine_permute    | DFS                  |                    |
| 0040   | 组合总和 II                     | combine_permute    | DFS                  |                    |
| 0041   | 缺失的第一个正数                | hard               |                      |                    |
| 0042   | 接雨水                          | hard               | Two pointers         |                    |
| 0045   | 跳跃游戏 II                     | series\jump_game   | Greed                |                    |
| 0046   | 全排列                          | combine_permute    | DFS                  |                    |
| 0047   | 全排列 II                       | combine_permute    | DFS                  |                    |
| 0050   | Pow(x, n)                       | medium_math        | Divide and conquer   |                    |
| 0051   | N 皇后                          | hard               | Traceback            |                    |
| 0052   | N 皇后 II                       | hard               | Traceback            |                    |
| 0053   | 最大子数组和                    | subarray           | Kadane               |                    |
| 0055   | 跳跃游戏                        | series\jump_game   | Greed                |                    |
| 0060   | 排列序列                        | combine_permute    |                      |                    |
| 0061   | 旋转链表                        | listnode           |                      | UPDATE             |
| 0062   | 不同路径                        | medium_dp          | DP                   | UPDATE             |
| 0063   | 不同路径 II                     | medium_dp          | DP                   | UPDATE             |
| 0064   | 最小路径和                      | medium_dp          | DP                   | UPDATE             |
| 0069   | x 的平方根                      | easy               | Binary search        |                    |
| 0075   | 颜色分类                        | medium_other       | Two pointers         |                    |
| 0077   | 组合                            | combine_permute    | DFS                  |                    |
| 0082   | 删除排序链表中的重复元素 II     | listnode           |                      | UPDATE             |
| 0083   | 删除排序链表中的重复元素        | listnode           |                      | UPDATE             |
| 0084   | 柱状图中最大的矩形              | stack_and_heap     | Stack                |                    |
| 0085   | 最大矩形                        | stack_and_heap     | Stack                |                    |
| 0094   | 二叉树的中序遍历                | treenode           | DFS                  | UPDATE             |
| 0095   | 不同的二叉搜索树 II             | treenode           | DFS                  | UPDATE             |
| 0096   | 不同的二叉搜索树                | treenode           | DP                   | UPDATE             |
| 0100   | 相同的树                        | treenode           | Recursive            | UPDATE             |
| 0101   | 对称二叉树                      | treenode           | DFS                  | UPDATE             |
| 0102   | 二叉树的层序遍历                | treenode           | BFS                  | UPDATE             |
| 0103   | 二叉树的锯齿形层序遍历          | treenode           | BFS                  | UPDATE             |
| 0104   | 二叉树的最大深度                | treenode           | Recursive            | UPDATE             |
| 0105   | 从前序与中序遍历序列构造二叉树  | treenode           | Recursive            | UPDATE             |
| 0106   | 从中序与后序遍历序列构造二叉树  | treenode           | Recursive            | UPDATE             |
| 0107   | 二叉树的层序遍历 II             | treenode           | BFS                  | UPDATE             |
| 0112   | 路径总和                        | treenode           | DFS                  | UPDATE             |
| 0113   | 路径总和 II                     | treenode           | DFS                  | UPDATE             |
| 0115   | 不同的子序列                    | hard               | DP                   | UPDATE             |
| 0121   | 买卖股票的最佳时机              | series\stock       | DP                   |                    |
| 0122   | 买卖股票的最佳时机 II           | series\stock       | DP                   |                    |
| 0123   | 买卖股票的最佳时机 III          | series\stock       | DP                   |                    |
| 0144   | 二叉树的前序遍历                | treenode           | DFS                  | UPDATE             |
| 0145   | 二叉树的后序遍历                | treenode           | DFS                  | UPDATE             |
| 0152   | 乘积最大子数组                  | subarray           |                      |                    |
| 0155   | 最小栈                          | construct          | Stack                |                    |
| 0167   | 两数之和 II - 输入有序数组      | medium_other       | Two pointers         | UPDATE             |
| 0169   | 多数元素                        | easy               | Boyer-Moore 投票算法 |                    |
| 0188   | 买卖股票的最佳时机 IV           | series\stock       | DP                   |                    |
| 0198   | 打家劫舍                        | series\rob         | DP                   |                    |
| 0202   | 快乐数                          | easy               | Hashmap              |                    |
| 0204   | 计数质数                        | medium_math        |                      |                    |
| 0206   | 反转链表                        | listnode           | Recursive            | UPDATE             |
| 0209   | 长度最小的子数组                | subarray           |                      |                    |
| 0215   | 数组中的第K个最大元素           | stack_and_heap     | Heap /計數           |                    |
| 0216   | 组合总和 III                    | combine_permute    | DFS                  |                    |
| 0217   | 存在重复元素                    | easy               |                      |                    |
| 0219   | 存在重复元素 II                 | easy               |                      |                    |
| 0220   | 存在重复元素 III                | hard               | Bucket Sort          |                    |
| 0221   | 最大正方形                      | medium_dp          | DP                   |                    |
| 0225   | 用队列实现栈                    | construct          | Stack                |                    |
| 0226   | 翻转二叉树                      | treenode           | Recursive            | UPDATE             |
| 0227   | 计算器                          | medium_other       |                      | 同面试题 16.26.    |
| 0229   | 多数元素 II                     | medium_other       | Hashmap              |                    |
| 0232   | 用栈实现队列                    | construct          | Stack                |                    |
| 0235   | 二叉搜索树的最近公共祖先        | treenode           | Recursive            | UPDATE             |
| 0236   | 二叉树的最近公共祖先            | treenode           | Recursive            | UPDATE             |
| 0238   | 除自身以外数组的乘积            | medium_other       | Prefix sum           |                    |
| 0258   | 各位相加                        | easy               |                      |                    |
| 0263   | 丑数                            | easy               |                      |                    |
| 0264   | 丑数 II                         | medium_math        | Multiple Pointers    |                    |
| 0279   | 完全平方数                      | medium_dp          | DP                   |                    |
| 0287   | 寻找重复数                      | medium_other       | Hashmap / Tips       |                    |
| 0295   | 数据流的中位数                  | construct          | Heap                 | 面试题 17.20.      |
| 0300   | 最长递增子序列                  | medium_dp          | Binary search        |                    |
| 0309   | 买卖股票的最佳时机含冷冻期      | series\stock       | DP                   |                    |
| 0313   | 超级丑数                        | medium_other       | Multiple Pointers    |                    |
| 0321   | 拼接最大数                      | stack_and_heap     | Stack                |                    |
| 0337   | 打家劫舍 III                    | series\rob         | DP /DFS              | UPDATE             |
| 0347   | 前 K 个高频元素                 | stack_and_heap     | Heap/Hashmap         |                    |
| 0354   | 俄罗斯套娃信封问题              | hard               | Binary search        |                    |
| 0367   | 有效的完全平方数                | easy               | Binary search        |                    |
| 0371   | 两整数之和                      | medium_math        | Bitwise Operations   | UPDATE             |
| 0372   | 超级次方                        | medium_math        | Divide and conquer   |                    |
| 0375   | 猜数字大小 II                   | medium_dp          | DP                   |                    |
| 0377   | 组合总和 IV                     | combine_permute    | DP                   |                    |
| 0382   | 链表随机节点                    | random_sampling    |                      |                    |
| 0398   | 随机数索引                      | random_sampling    |                      |                    |
| 0402   | 移掉 K 位数字                   | stack_and_heap     | Stack                |                    |
| 0403   | 青蛙过河                        | hard               | DP                   |                    |
| 0404   | 左叶子之和                      | treenode           | DFS                  | UPDATE             |
| 0413   | 等差数列划分                    | subarray           |                      |                    |
| 0435   | 无重叠区间                      | medium_dp          | DP                   |                    |
| 0437   | 路径总和 III                    | treenode           | DFS                  | UPDATE             |
| 0446   | 等差数列划分 II - 子序列        | hard               | DP/Hashmap           |                    |
| 0452   | 用最少数量的箭引爆气球          | medium_dp          | DP                   |                    |
| 0454   | 四数相加 II                     | medium_other       | Hashmap              |                    |
| 0456   | 132 模式                        | stack_and_heap     | Stack                |                    |
| 0458   | 可怜的小猪                      | hard               |                      |                    |
| 0474   | 一和零                          | medium_dp          | DP                   |                    |
| 0478   | 在圆内随机生成点                | random_sampling    |                      |                    |
| 0479   | 最大回文数乘积                  | hard               |                      | UPDATE             |
| 0486   | 预测赢家                        | series\stone_game  | DP                   |                    |
| 0491   | 非递减子序列                    | medium_dfs_or_bfs  | DFS                  |                    |
| 0496   | 下一个更大元素 I                | stack_and_heap     | Stack                |                    |
| 0497   | 非重叠矩形中的随机点            | random_sampling    |                      |                    |
| 0498   | 对角线遍历                      | medium_other       |                      |                    |
| 0502   | IPO                             | stack_and_heap     | Heap                 |                    |
| 0503   | 下一个更大元素 II               | stack_and_heap     | Stack                |                    |
| 0504   | 七进制数                        | easy               |                      |                    |
| 0507   | 完美数                          | easy               |                      |                    |
| 0519   | 随机翻转矩阵                    | random_sampling    |                      |                    |
| 0523   | 连续的子数组和                  | subarray           |                      |                    |
| 0528   | 按权重随机选择                  | random_sampling    |                      |                    |
| 0554   | 砖墙                            | medium_other       | Hashmap              |                    |
| 0556   | 下一个更大元素 III              | stack_and_heap     | Stack                |                    |
| 0560   | 和为 K 的子数组                 | subarray           |                      |                    |
| 0633   | 平方数之和                      | medium_math        | Binary search        |                    |
| 0646   | 最长数对链                      | medium_dp          | DP                   |                    |
| 0647   | **回文子串**                    | medium_string      | Manacher             | unimplement        |
| 0650   | 两个键的键盘                    | medium_dp          | DP                   |                    |
| 0653   | 两数之和 IV                     | treenode           | DFS                  | UPDATE             |
| 0658   | 找到 K 个最接近的元素           | medium_other       | Slide window         |                    |
| 0670   | 最大交换                        | stack_and_heap     |                      |                    |
| 0692   | 前K个高频单词                   | medium_string      |                      |                    |
| 0701   | 二叉搜索树中的插入操作          | treenode           | Recursive            | UPDATE             |
| 0704   | 二分查找                        | easy               | Binary search        |                    |
| 0705   | 设计哈希集合                    | construct          |                      |                    |
| 0710   | 黑名单中的随机数                | random_sampling    |                      |                    |
| 0713   | 乘积小于 K 的子数组             | subarray           |                      |                    |
| 0714   | 买卖股票的最佳时机含手续费      | series\stock       | DP                   |                    |
| 0752   | 打开转盘锁                      | medium_dfs_or_bfs  | BFS                  |                    |
| 0768   | 最多能完成排序的块 II           | stack_and_heap     | Stack                |                    |
| 0769   | 最多能完成排序的块              | medium_other       | Greedy               |                    |
| 0793   | 阶乘函数后 K 个零               | hard               | Binary search        |                    |
| 0799   | 香槟塔                          | medium_dp          | DP                   |                    |
| 0810   | 黑板异或游戏                    | hard               |                      |                    |
| 0836   | 矩形重叠                        | easy               |                      |                    |
| 0857   | 雇佣 K 名工人的最低成本         | stack_and_heap     | Heap /sort           |                    |
| 0875   | 爱吃香蕉的珂珂                  | medium_other       | Binary search        | UPDATE             |
| 0877   | 石子游戏                        | series\stone_game  | 純推理               |                    |
| 0887   | 鸡蛋掉落                        | hard               | DP                   | 1884.鸡蛋掉落-两枚 |
| 0889   | 根据前序和后序遍历构造二叉树    | treenode           | Recursive            | UPDATE             |
| 0891   | 子序列宽度之和                  | hard               |                      |                    |
| 0901   | 股票价格跨度                    | construct          |                      |                    |
| 0907   | 子数组的最小值之和              | stack_and_heap     | Stack                |                    |
| 0918   | 环形子数组的最大和              | subarray           | Kadane               |                    |
| 0951   | 翻转等价二叉树                  | treenode           | Recursive            | UPDATE             |
| 0960   | 删列造序 III                    | hard               | DP                   |                    |
| 0972   | 相等的有理数                    | hard               |                      |                    |
| 0973   | 最接近原点的 K 个点             | stack_and_heap     | Heap                 |                    |
| 0974   | 和可被 K 整除的子数组           | subarray           |                      |                    |
| 0977   | 有序数组的平方                  | easy               |                      |                    |
| 0989   | 数组形式的整数加法              | easy               |                      | UPDATE             |
| 1052   | 爱生气的书店老板                | medium_other       | Sliding window       |                    |
| 1105   | 填充书架                        | medium_dp          | DP                   | UPDATE             |
| 1109   | 航班预订统计                    | medium_other       | Difference           | UPDATE             |
| 1223   | 掷骰子模拟                      | hard               | DP                   |                    |
| 1224   | 最大相等频率                    | hard               | Hashmap              |                    |
| 1276   | 不浪费原料的汉堡制作方案        | medium_math        |                      |                    |
| 1269   | 停在原地的方案数                | hard               | DP                   | UPDATE             |
| 1306   | 跳跃游戏 III                    | series\jump_game   | BFS                  |                    |
| 1340   | 跳跃游戏 V                      | series\jump_game   | DFS / DP             |                    |
| 1345   | 跳跃游戏 IV                     | series\jump_game   | BFS                  |                    |
| 1363   | 形成三的最大倍数                | hard               |                      |                    |
| 1383   | 最大的团队表现值                | stack_and_heap     | Heap/Sort            |                    |
| 1388   | 3n 块披萨                       | stack_and_heap     | Heap/DP              |                    |
| 1402   | 做菜顺序                        | hard               | Prefix sum           |                    |
| 1406   | 石子游戏 III                    | series\stone_game  | Prefix sum           |                    |
| 1411   | 给 N x 3 网格图涂色的方案数     | hard               | DP                   | UPDATE             |
| 1424   | 对角线遍历 II                   | medium_other       |                      |                    |
| 1463   | 摘樱桃 II                       | hard               | DP                   |                    |
| 1492   | n 的第 k 个因子                 | medium_math        |                      |                    |
| 1510   | 石子游戏 IV                     | series\stone_game  | DP                   |                    |
| 1553   | 吃掉 N 个橘子的最少天数         | hard               | DP                   |                    |
| 1561   | 你可以获得的最大硬币数目        | medium_other       |                      |                    |
| 1599   | 经营摩天轮的最大利润            | medium_other       |                      |                    |
| 1686   | 石子游戏 VI                     | series\stone_game  | Sort                 |                    |
| 1675   | 数组的最小偏移量                | stack_and_heap     | Heap                 | **困難**           |
| 1690   | 石子游戏 VII                    | series\stone_game  | DP                   |                    |
| 1696   | 跳跃游戏 VI                     | series\jump_game   | DP/Heap/Slide Window |                    |
| 1691   | 堆叠长方体的最大高度            | hard               | DP                   |                    |
| 1735   | 生成乘积数组的方案数            | hard               | DP                   | UPDATE             |
| 1739   | 放置盒子                        | hard               | Binary search        |                    |
| 1776   | 车队 II                         | stack_and_heap     | Stack                |                    |
| 1793   | 好子数组的最大分数              | stack_and_heap     | Stack                |                    |
| 1856   | 子数组最小乘积的最大值          | stack_and_heap     | Stack                |                    |
| 1866   | 恰有 K 根木棍可以看到的排列数目 | hard               | DP                   |                    |
| 1871   | 跳跃游戏 VII                    | series\jump_game   | DP/Prefix sum        |                    |
| 1872   | 石子游戏 VIII                   | series\stone_game  | DP/Prefix sum        | **困難**           |
| 1927   | 求和游戏                        | series\stone_game  |                      |                    |
| 1931   | 用三种不同颜色为网格涂色        | hard               | DP                   | UPDATE             |
| 1969   | 数组元素的最小非零乘积          | medium_math        | Divide and conquer   |                    |
| 1997   | 访问完所有房间的第一天          | medium_dp          | DP                   |                    |
| 2007   | 从双倍数组中还原原数组          | medium_other       | Hashmap              | UPDATE             |
| 2009   | 使数组连续的最少操作数          | hard               | Two pointers         | UPDATE             |
| 2025   | 分割数组的最多方案数            | hard               |                      | UPDATE             |
| 2029   | 石子游戏 IX                     | series\stone_game  |                      |                    |
| 2088   | 统计农场中肥沃金字塔的数目      | hard               | DP                   |                    |
| 2104   | 子数组范围和                    | stack_and_heap     | Stack                |                    |
| 2280   | 表示一个折线图的最少线段数      | medium_math        |                      |                    |
| 2281   | 巫师的总力量和                  | stack_and_heap     | Stack                |                    |
| 2321   | 拼接数组的最大分数              | subarray           | Kadane               |                    |
| 2334   | 元素值大于变化阈值的子数组      | stack_and_heap     | Stack                |                    |
| 2344   | 使数组可以被整除的最少删除次数  | stack_and_heap     | Heap                 |                    |
| 2354   | 优质数对的数目                  | hard               |                      | UPDATE             |
| 2386   | 找出数组的第 K 大和             | stack_and_heap     | Heap                 |                    |
| 2454   | 下一个更大元素 IV               | stack_and_heap     | Stack                |                    |
| 2498   | 青蛙过河 II                     | medium_other       | Greedy               |                    |
| 2529   | 正整数和负整数的最大计数        | easy               | Binary search        |                    |
| 2551   | 将珠子放入背包中                | stack_and_heap     | Heap                 |                    |
| 2561   | 重排水果                        | hard               | Greedy  /Hashmap     |                    |
| 2585   | 获得分数的方法数                | hard               | DP                   | UPDATE             |
| 2671   | 频率跟踪器                      | construct          |                      |                    |
| 2681   | 英雄的力量                      | hard               |                      |                    |
| 2709   | **最大公约数遍历**              | hard               | Union Find           | unimplement        |
| 2751   | 机器人碰撞                      | hard               | Stack/Sort           |                    |
| 2836   | 在传球游戏中最大化函数值        | hard               | DP/Bit manipulation  |                    |
| 2897   | 对数组执行操作使平方和最大      | hard               | DP/Bit manipulation  |                    |
| 2865   | 美丽塔 I                        | stack_and_heap     | Stack/DP             |                    |
| 2866   | 美丽塔 II                       | stack_and_heap     | Stack/DP             |                    |
| 2899   | 上一个遍历的整数                | easy               |                      |                    |
| 2924   | 找到冠军 II                     | medium_graph       |                      |                    |
| 2929   | 给小朋友们分糖果 II             | medium_math        | Combination          |                    |
| 3021   | Alice 和 Bob 玩鲜花游戏         | medium_math        |                      |                    |
| 3025   | 人员站位的方案数 I              | hard               |                      |                    |
| 3027   | 人员站位的方案数 II             | hard               |                      |                    |
| 3034   | 匹配模式数组的子数组数目 I      | hard               | KMP                  |                    |
| 3036   | 匹配模式数组的子数组数目 II     | hard               | KMP                  |                    |
| 3074   | 重新分装苹果                    | easy               |                      |                    |
| 3075   | 幸福值最大化的选择方案          | estack_and_heapasy | Heap/Sort            |                    |
| 3082   | 求出所有子序列的能量和          | hard               | DP                   |                    |
| 3099   | 哈沙德数                        | easy               |                      |                    |
| 3100   | 换水问题 II                     | medium_other       |                      |                    |
| 3101   | 交替子数组计数                  | medium_other       |                      |                    |
| LCP25  | 古董键盘                        | hard               | DP                   |                    |
| 05_04  | 下一个数                        | medium_other       | Gosper's Hack        |                    |
| 08_06  | 汉诺塔问题                      | medium_dfs_or_bfs  | DFS                  |                    |
| 08_13  | 堆箱子                          | hard               | DP                   |                    |
| 100256 | 替换字符可以得到的最晚时间      | easy               |                      |                    |
| 100265 | 素数的最大距离                  | medium_other       | Hashmap              |                    |
| 100291 | 统计特殊字母的数量 II           | medium_string      |                      | UPDATE             |
| 100294 | 统计特殊字母的数量 I            | easy               |                      | UPDATE             |


# 學習資料  
並查集算法：https://zhuanlan.zhihu.com/p/93647900  
並查集算法：https://www.cnblogs.com/onlyblues/p/14668087.html  
水塘抽樣：https://zhuanlan.zhihu.com/p/29178293  
鏈表：https://rust-unofficial.github.io/too-many-lists/  
 

# 待完成事項  
RUST重寫已完成的GOLANG題目 (250/400)  
GRAPH
Union-find Data Structure
Dijkstra 最短路