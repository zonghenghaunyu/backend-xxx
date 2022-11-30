
#[cfg(test)]
mod tests {

    #[test]
    fn it_works1() {
        let vec1 = vec![10, 20, 30, 5, 10, 50];
        let a = max_ascending_sum(vec1);
        println!("{}",a)
    }

    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {

        let mut lastNum = nums.get(0).unwrap();

        let mut sum = 0;
        let mut sumMax : i32 = 0;

        for x in nums.iter() {
            if(lastNum >= x){
                sum = 0;
            }
            sum += x;
            if(sumMax <= sum){
                sumMax = sum;
            }
            lastNum = x;
        }

        sumMax
    }
}
