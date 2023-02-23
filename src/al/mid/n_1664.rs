#[cfg(test)]
mod test {

    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let len = nums.len();

        if len == 1 {
            return 1;
        }
        let mut odd = true;
        let mut ans = 0;
        for idx in 0..len {
            let mut odd_sum = 0;
            let mut even_sum = 0;
            for inner in 0..len {
                if idx == inner {
                    continue;
                }
                //odd
                if odd {
                    odd_sum += nums[inner];
                    odd = false;
                    continue;
                }
                //even
                if !odd {
                    even_sum += nums[inner];
                    odd = true;
                }
            }
            if odd_sum == even_sum && odd_sum !=0{
                ans += 1;
            }
        }
        ans
    }

    pub fn ways_to_make_fair1(nums: Vec<i32>) -> i32 {
        //3 ,4 ,5 ,3 ,2 ,6 ,3 ,6
        //13,19,10,15,5 ,12,3 ,6                    
        //3 ,4 ,8 ,7 ,10,13,13,19
        // if idx = odd
        // odd = head[idx - 1] + tail[idx + 2]
        // even = head[idx - 2] + tail[idx + 1]
        // if idx = even
        // odd = head[idx - 2] + tail[idx + 1]
        // even = head[idx - 1] + tail[idx + 2]
        //3,4,5, ,2,6,3,6{
        //1,2,3,4
        let mut ans = 0;
        let len = nums.len();

        if len == 1{
            return 1;
        }
        if len == 2{
            return 0;
        }
        if len == 3{
            //0
            if nums[1] == nums[2]{
                ans += 1;
            }
            //1
            if nums[0] == nums[2]{
                ans += 1;
            }
            //2
            if nums[0] == nums[1]{
                ans += 1;
            }
            return ans;

        }
        if len == 4{
            //0
            if nums[1] + nums[3]==nums[2]{
                ans += 1;
            }
            //1
            if nums[0] + nums[3]==nums[2]{
                ans+=1;
            }
            //2
            if nums[0] + nums[3]==nums[1]{
                ans += 1;
            }
            //3
            if nums[0] + nums[2]==nums[1]{
                ans+=1;
            }
            return ans;
        }

        let mut tail = nums.clone();
        let mut head = nums.clone();

        for idx in 0..len {
            if idx > 1{
                head[idx] = head[idx - 2] + nums[idx];
                tail[len - 1 - idx] = tail[len - 1 - idx + 2] + nums[len - 1 - idx];
            }
            // if idx < len - 2{
                // tail[len - 1 - idx] = tail[len - 1 - idx + 2] + nums[idx];
            // }
        }
        let mut odd = true;
        
        //0
        if tail[1] == tail[2]{
            ans += 1;
        }
        //1
        if head[0] + tail[3] == tail[2]{
            ans += 1;
        }
        //len - 1
        if head[len - 3] == head[len - 2]{
            ans += 1;
        }
        //len - 2
        if tail[len - 1] + head[len - 4] == head[len - 3]{
            ans += 1;
        }
        for idx in 2..len - 2 {
            if odd{
                let odd_sum = head[idx - 1] + tail[idx + 2];
                let even_sum = head[idx - 2] + tail[idx + 1];
                if odd_sum == even_sum{
                    ans += 1;
                }
                odd = false;
                continue;
            }
            if !odd {
                let odd_sum = head[idx - 2] + tail[idx + 1];
                let even_sum = head[idx - 1] + tail[idx + 2];
                if odd_sum == even_sum{
                    ans += 1;
                }
                odd = true; 
            }
        }
        
        //if len is odd
        // odd:  f[n] = f[n+1] + nums[n]
        // even: f[n] = f[n+1] + nums[n]
        //if len is even
        //odd:  f[n] = f[n+1] + nums[n]
        //even: f[n] = f[n+1] + nums[n]


        ans
    }

    #[test]
    fn it_work() {

        let ans = ways_to_make_fair1(vec![1,1,1,1,1,1,1]);
        println!("{}",ans)
    }
}
