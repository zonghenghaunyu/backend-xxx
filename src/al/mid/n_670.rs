struct Solution {}

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let str = num.to_string();

        let mut temp: char = '0';
        let mut idx : usize = 0;
        for x in str.chars() {
            idx += 1;
            if temp < x {
                temp = x;
            }
        }
        let mut str_last: Vec<char> = Vec::new();
        if idx != 0 {
            let mut nidx = 0;
            let mut c = ' ';
            for x in str.chars() {
                if nidx == idx {
                    str_last.push(x);
                }
                if nidx == 0 {
                    c = x;
                }
                nidx += 1;
            }
            nidx = 0;
            for x in str.chars() {
                if nidx == 0 {
                    nidx +=1 ;
                    continue;
                }
                if nidx != idx {
                    str_last.push(x);
                }else {
                    str_last.push(c);
                }
                nidx += 1;
            }

        }else {
            for x in str.chars() {
                str_last.push(x);
            }
        }
        let ans = String::from_iter(str_last);
        let my_int = ans.parse::<i32>().unwrap();
        my_int
    }
}


#[cfg(test)]
mod tests {
    use crate::al::mid::n_670::Solution;

    #[test]
    fn it_works1() {
        let a = Solution::maximum_swap(555);
        println!("{}",a)
    }
}