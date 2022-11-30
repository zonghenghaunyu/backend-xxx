
#[cfg(test)]
mod tests {

    #[test]
    fn it_works1() {
        let vec1 = vec![10, 20, 30, 5, 10, 50];
        let a = max_ascending_sum(vec1);
        println!("{}",a)
    }

    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {

        let mut last_num = nums.get(0).unwrap();

        let mut sum = 0;
        let mut sum_max: i32 = 0;

        for x in nums.iter() {
            if last_num >= x {
                sum = 0;
            }
            sum += x;
            if sum_max <= sum {
                sum_max = sum;
            }
            last_num = x;
        }

        sum_max
    }
}
