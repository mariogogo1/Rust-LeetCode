# heap使用筆記    
1.常用語法  
https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#method.pop
    (1).std::collections::BinaryHeap：默認大根堆
    (2).std::cmp::Reverse 小根堆
    (3).push()：
    小根堆 heap.push(reverse(x)) 
    (4).heap.peek()：用peek()函数获取堆中的最大值，返回类型是Option<&T>
    (5).pop()：彈出堆頂，The worst case cost of pop on a heap containing n elements is O(log(n)).
    (6).heap.len()
    (7).heap.clear()
    (8).heap.is_empty()
    (9).heap.into_sorted_vec()：排序好





