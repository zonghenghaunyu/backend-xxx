

#[cfg(test)]
mod tests {

    #[test]
    fn do_test() {
        let s = vec![10, 5,-2,20, 20];
        let a = max_product(s);
        println!("{}",a)
    }
    #[test]
    fn do_test2() {
        let s = vec![-2,0,-1];
        let a = max_product_with_dp(s);
        println!("{}",a)
    }
    #[test]
    fn do_test3() {
        let s = vec![-2,0,-1];
        let a = max_product_with_dp1(s);
        println!("{}",a)
    }

    fn max_product(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let len = nums.len();
        for i in 0..len {
            let mut cur_max = nums[i];
            let mut cache = nums[i];
            for j in i+1..len {
                cache = cache * nums[j];
                if cache > cur_max {
                    cur_max = cache;
                }
            }
            if cur_max > max {
                max = cur_max;
            }
        }
        max
    }

    //err
    fn max_product_with_dp(nums: Vec<i32>) -> i32 {
        let len = nums.len();

        let mut max = 1;
        let mut rmax = nums[0];
        let mut min = 1;
        for i in 0..len {
            if nums[i] < 0 {
                let a = max;
                max = min;
                min = a;
            }
            max = nums[i].max(max * nums[i]);
            min = nums[i].min(min * nums[i]);
            rmax = rmax.max(max)
        }
        rmax
    }
    // [2,3,-2,4]
    fn max_product_with_dp1(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut imax = nums[0];
        let mut imin = nums[0];
        let mut max = nums[0];
        for i in 1..len {
            if nums[i] < 0 {
                let tpm = imax;
                imax = imin;
                imin = tpm;
            }
            // imax = imax.max(imax * nums[i]);
            // imin = imin.min(imin * nums[i]);
            imax = nums[i].max(imax * nums[i]);
            imin = nums[i].min(imin * nums[i]);
            max = max.max(imax);
            println!("{},{}",i,imax);
            println!("{},{}",i,imin)
        }
        max
    }



}