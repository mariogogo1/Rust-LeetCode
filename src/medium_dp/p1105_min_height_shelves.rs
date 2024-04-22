/**
1105. 填充书架

给定一个数组 books ，其中 books[i] = [thicknessi, heighti] 表示第 i 本书的厚度和高度。你也会得到一个整数 shelfWidth 。

按顺序 将这些书摆放到总宽度为 shelfWidth 的书架上。

先选几本书放在书架上（它们的厚度之和小于等于书架的宽度 shelfWidth ），然后再建一层书架。重复这个过程，直到把所有的书都放在书架上。

需要注意的是，在上述过程的每个步骤中，摆放书的顺序与给定图书数组 books 顺序相同。

例如，如果这里有 5 本书，那么可能的一种摆放情况是：第一和第二本书放在第一层书架上，第三本书放在第二层书架上，第四和第五本书放在最后一层书架上。
每一层所摆放的书的最大高度就是这一层书架的层高，书架整体的高度为各层高之和。

以这种方式布置书架，返回书架整体可能的最小高度。
。
https://leetcode.cn/problems/filling-bookcase-shelves/description/
*/

pub struct Solution;
impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let n = books.len();
        let mut dp = vec![0; n + 1];

        for i in 1..=n {
            let mut width = 0;
            let mut max_height_this_layer = 0;
            let mut x = i32::MAX;
            for j in (0..i) {
                width += books[i - j - 1][0];
                max_height_this_layer = max_height_this_layer.max(books[i - j - 1][1]);
                if width > shelf_width {
                    break;
                }
                x = x.min(max_height_this_layer + dp[i - j - 1]);
            }
            dp[i] = x;
        }

        dp[n]
    }
}
