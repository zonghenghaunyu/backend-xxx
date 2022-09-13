struct Solution{}

impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {

        let mut v = n.to_string().into_bytes();
        println!("{:?}",v);
        v.iter_mut().for_each(|x| *x -= b'0');//b'0' = 48
        println!("{:?}",v);
        println!("{}",b'0');
        if let Some(mut i) = (1..v.len()).find(|&i| v[i - 1] > v[i]) {
            while i > 0 && !(v[i - 1] <= v[i]) {
                v[i - 1] -= 1;
                i -= 1;
            }
            ((i + 1)..v.len()).for_each(|i| v[i] = 9);
        }
        println!("{:?}",v);
        v.into_iter().fold(0, |acc, v| acc * 10 + v as i32)
    }

    pub fn monotone_increasing_digits1(n: i32) -> i32 {

        let mut v = n.to_string().into_bytes();
        v.iter_mut().for_each(|x| *x -= 48);//b'0' = 48
        let len = v.len();
        let mut flag = len - 1 as usize;
        for idx in (1 ..= len - 1).rev() {
            println!("{},{}",idx,v[idx]);
            if v[idx] < v[idx - 1] {
                v[idx - 1] = v[idx - 1] - 1;
                flag = idx;
            }
        }
        println!("{:?}",v);
        for idx in flag .. len {
            v[idx] = 9
        }

        v.into_iter().fold(0, |acc, v| acc * 10 + v as i32)
    }
    
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        let ans = Solution :: monotone_increasing_digits1(332);

        // let a = 'b';
        // let b = b'b';
        println!("{}",ans)
    }

    #[test]
    fn it_works2() {

        for idx in (0 .. 10).rev() {
            println!("{}",idx)

        }

    }

}
