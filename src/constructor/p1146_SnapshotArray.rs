/**
1146. 快照数组

实现支持下列接口的「快照数组」- SnapshotArray：

SnapshotArray(int length) - 初始化一个与指定长度相等的 类数组 的数据结构。初始时，每个元素都等于 0。
void set(index, val) - 会将指定索引 index 处的元素设置为 val。
int snap() - 获取该数组的快照，并返回快照的编号 snap_id（快照号是调用 snap() 的总次数减去 1）。
int get(index, snap_id) - 根据指定的 snap_id 选择快照，并返回该快照指定索引 index 的值。

https://leetcode.cn/problems/snapshot-array/description/

*/

struct SnapshotArray {
    snap_idx: i32,
    data_idx_values: Vec<Vec<(i32, i32)>>,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        SnapshotArray {
            snap_idx: 0,
            data_idx_values: vec![vec![]; length as usize],
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        let index = index as usize;
        if let Some(&val) = self.data_idx_values[index].last() {
            if val.0 == self.snap_idx {
                self.data_idx_values[index].pop();
            }
        }
        self.data_idx_values[index].push((self.snap_idx, val));
    }

    fn snap(&mut self) -> i32 {
        self.snap_idx += 1;
        return self.snap_idx - 1;
    }

    fn get(&mut self, index: i32, snap_id: i32) -> i32 {
        let find_idx = self.binary_search(index, snap_id);
        let index = index as usize;
        if find_idx == 0 {
            return 0;
        }
        return self.data_idx_values[index][find_idx - 1].1;
    }

    fn binary_search(&self, index: i32, snap_id: i32) -> usize {
        let index = index as usize;
        let mut l: usize = 0;
        let mut r: usize = self.data_idx_values[index].len();
        while l < r {
            let m = l + ((r - l) / 2);
            let (x, y) = self.data_idx_values[index][m];
            if x >= snap_id + 1 {
                r = m;
            } else if x == snap_id {
                return m + 1;
            } else {
                l = m + 1;
            }
        }
        l
    }
}
