pub mod n_104_deep_of_tree;
pub mod n_1470;
pub mod n_1800;
pub mod n_94;
pub mod n_26;


//遍历二维数组
#[cfg(test)]
mod tests {
    #[test]
    fn it_works1() {
        let vec1 = vec![11, 12, 13, 14, 31];
        let vec2 = vec![22, 23, 24, 15, 32];
        let vec3 = vec![21, 26, 25, 16, 33];
        let vec4 = vec![20, 19, 18, 17, 34];
        let vec5 = vec![41, 42, 43, 44, 45];
        let vec6 = vec![61, 62, 63, 64, 65];
        let arr = vec![vec1, vec2, vec3, vec4, vec5, vec6];
        traversal_array2(arr);

        for i in (0..5).rev() {
            println!("{i}")
        }
    }

    fn traversal_array2(nums: Vec<Vec<i32>>) {
        let mut ans = Vec::<i32>::new();
        if nums.len() == 0 {
            return;
        }
        let w = nums[0].len();
        let h = nums.len();

        let mut left = 0;
        let mut top = 0;
        let mut right = w - 1;
        let mut bottom = h - 1;

        while top < bottom && left < right {
            for i in left..right {
                ans.push(nums[top][i]);
            }
            for i in top..bottom {
                ans.push(nums[i][right]);
            }
            for i in (left + 1..=right).rev() {
                ans.push(nums[bottom][i]);
            }
            for i in (top + 1..=bottom).rev() {
                ans.push(nums[i][left]);
            }
            left += 1;
            top += 1;
            right -= 1;
            bottom -= 1;
        }

        if top == bottom {
            for i in left..right {
                ans.push(nums[top][i]);
            }
        } else if left == right {
            for i in top..bottom {
                ans.push(nums[i][right]);
            }
        }

        println!("{:?}", ans)
    }
}
