#[cfg(test)]
mod hui_wen {

    #[test]
    fn its_work() {
        let ans = count_substrings(String::from("aaa"));
        println!("work!::::::::::::{}", ans);
    }
    #[test]
    fn is_sub() {
        let a = "aaccbbccaa";
        let arr = a.chars().collect::<Vec<_>>();
        let b = is_susbtr(&arr, 0, arr.len() - 1);
        println!("{}", b);
    }

    pub fn count_substrings(s: String) -> i32 {
        let arr = s.chars().collect::<Vec<_>>();
        let len = arr.len();
        if len == 0 {
            return 0;
        }
        let mut last_size = 1;

        for i in 1..len {
            let mut cur_num = 0;

            for j in 0..=i {
                if is_susbtr(&arr, j, i) {
                    cur_num += 1;
                }
            }

            last_size += cur_num;
        }
        last_size
    }

    fn is_susbtr(arr: &Vec<char>, start: usize, end: usize) -> bool {
        if start == end {
            return true;
        } else if start > end {
            return false;
        }
        let mut start = start;
        let mut end = end;
        while start <= end {
            if arr[start] != arr[end] {
                return false;
            }
            start += 1;
            end -= 1;
        }

        return true;
    }
}
