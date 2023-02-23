#[cfg(test)]
mod tset {
    use std::collections::HashSet;

    #[test]
    fn its_work() {
        partition001(String::from("abcbaa"));
    }

    fn partition001(s: String) -> Vec<Vec<String>> {
        let arr = s.chars().collect::<Vec<_>>();
        let mut cache = Vec::<String>::new();
        let mut ans = Vec::<Vec<String>>::new();
        let mut is_right = HashSet::<(usize, usize)>::new();
        get_all_sub_right(&mut is_right, &arr);
        inner_partition001(
            &is_right,
            &arr,
            0,
            arr.len() - 1,
            &mut cache,
            &mut ans,
            arr.len(),
        );

        println!("{:?}", ans);
        ans
    }

    fn inner_partition001(
        is_right: &HashSet<(usize, usize)>,
        arr: &Vec<char>,
        start: usize,
        end: usize,
        cache: &mut Vec<String>,
        ans: &mut Vec<Vec<String>>,
        len: usize,
    ) {
        if start >= len {
            let in_ans = cache.clone();
            ans.push(in_ans);
            return;
        }
        for i in (start + 1)..=len {
            if !is_right.contains(&(start, i - 1)) {
                continue;
            }
            let in_str = arr[start..i].iter().collect::<String>();
            cache.push(in_str);
            inner_partition001(is_right, arr, i, i, cache, ans, len);
            cache.pop();
        }
    }

    fn get_all_sub_right(is_right: &mut HashSet<(usize, usize)>, arr: &Vec<char>) {
        let len = arr.len();

        for i in 0..len {
            for j in i..len {
                if is_susbtr(arr, i, j) {
                    is_right.insert((i, j));
                }
            }
        }
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
