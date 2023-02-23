#[cfg(test)]
mod test {
    // 给定两个以 升序排列 的整数数组 nums1 和 nums2 , 以及一个整数 k 。

    // 定义一对值 (u,v)，其中第一个元素来自 nums1，第二个元素来自 nums2 。

    // 请找到和最小的 k 个数对 (u1,v1),  (u2,v2)  ...  (uk,vk) 。

    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut last = vec![nums1[0], nums2[0]];
        ans.push(last);

        let mut idx_1 = 0;
        let mut idx_2 = 0;
        for i in 0..k - 1 {
            let mut cell = Vec::new();

            let small = nums1[idx_1].min(nums2[idx_2]);

            if idx_2 + 1 > nums2.len() - 1 {
                cell.push(nums1[idx_1]);
                cell.push(nums2[idx_2]);
                idx_1 += 1;
                continue;
            }

            if idx_1 + 1 > nums1.len() - 1 {
                cell.push(nums1[idx_1]);
                cell.push(nums2[idx_2]);
                idx_2 += 1;
                continue;
            }
            if small == nums1[idx_1] {
                cell.push(small);
                cell.push(nums2[idx_2 + 1]);
                idx_2 += 1;
                if idx_2 + 1 > nums2.len() - 1 {
                    cell.push(nums2[idx_2]);
                }
            } else {
                cell.push(nums1[idx_1 + 1]);
                idx_1 += 1;
                cell.push(nums2[idx_2]);
            }
            ans.push(cell);
        }

        ans
    }

    #[test]
    pub fn is_Work() {
        println!("aaaa")
    }
}
