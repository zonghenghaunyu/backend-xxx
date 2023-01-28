#[cfg(test)]
mod tttt {

    #[test]
    fn works() {
        let mut nums = vec![1, 2, 2, 2, 3, 3, 4, 5, 6, 7, 8];
        let ans = remove_duplicates(&mut nums);
        println!("{}:{:?}", ans, nums)
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 {
            return 1;
        }

        let mut pri = 0;
        let mut tail = 1;

        loop {
            if nums[pri] == nums[tail] {
                tail += 1;
            } else {
                pri += 1;
                nums[pri] = nums[tail];
                tail += 1;
            }
            if tail == len {
                break;
            }
        }
        (pri + 1) as i32
    }
}
