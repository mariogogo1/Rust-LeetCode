/**
355. 设计推特

设计一个简化版的推特(Twitter)，可以让用户实现发送推文，关注/取消关注其他用户，能够看见关注人（包括自己）的最近 10 条推文。

实现 Twitter 类：

Twitter() 初始化简易版推特对象
void postTweet(int userId, int tweetId) 根据给定的 tweetId 和 userId 创建一条新推文。每次调用此函数都会使用一个不同的 tweetId 。
List<Integer> getNewsFeed(int userId) 检索当前用户新闻推送中最近  10 条推文的 ID 。新闻推送中的每一项都必须是由用户关注的人或者是用户自己发布的推文。推文必须 按照时间顺序由最近到最远排序 。
void follow(int followerId, int followeeId) ID 为 followerId 的用户开始关注 ID 为 followeeId 的用户。
void unfollow(int followerId, int followeeId) ID 为 followerId 的用户不再关注 ID 为 followeeId 的用户。

https://leetcode.cn/problems/design-twitter/description/
*/
use std::collections::HashSet;

struct Twitter {
    posts: Vec<(i32, i32)>,
    followees: Vec<HashSet<i32>>,
}

impl Twitter {
    fn new() -> Self {
        Self {
            posts: Vec::new(),
            followees: vec![HashSet::new(); 500],
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.posts.push((user_id, tweet_id));
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        self.posts
            .iter()
            .rev()
            .filter(|(x, _)| *x == user_id || self.followees[user_id as usize].contains(x)) // 學習 !!
            .map(|(_, x)| *x)
            .take(10)
            .collect()
    }

    fn follow(&mut self, follower: i32, followee: i32) {
        self.followees[follower as usize].insert(followee);
    }

    fn unfollow(&mut self, follower: i32, followee: i32) {
        self.followees[follower as usize].remove(&followee);
    }
}
