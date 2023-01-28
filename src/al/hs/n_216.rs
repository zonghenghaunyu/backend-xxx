// 找出所有相加之和为 n 的 k 个数的组合。组合中只允许含有 1 - 9 的正整数，并且每种组合中不存在重复
// 的数字。
// 说明：
// 所有数字都是正整数。
// 解集不能包含重复的组合。
// 示例 1:
// 输⼊: k = 3, n = 7
// 输出: [[1,2,4]]
// 示例 2:
// 输⼊: k = 3, n = 9
// 输出: [[1,2,6], [1,3,5], [2,3,4]]

pub fn combination_sum_xxxx(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut ans = Vec::<Vec<i32>>::new();

    let mut cache = Vec::<i32>::new();
    fn inner(
        ans: &mut Vec<Vec<i32>>,
        cache: &mut Vec<i32>,
        start: usize,
        end: usize,
        cur_level: i32,
        k: i32,
        n: i32,
    ) {
        if cur_level == k {
            let sum: i32 = cache.iter().sum();
            if sum == n {
                ans.push(cache.clone())
            }
            return;
        }

        for i in start..end {
            cache.push(i as i32);
            inner(ans, cache, i + 1, end, cur_level + 1, k, n);
            cache.pop();
        }
    }

    inner(&mut ans, &mut cache, 1, 9, 0, k, n);
    ans
}

pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut max = n;
    if n > 9 {
        max = 9;
    }
    let mut ans = Vec::<Vec<i32>>::new();
    let mut v = vec![0; k as usize];

    fn xx(
        sin: i32,
        k: i32,
        n: i32,
        curl: i32,
        arr: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
        max: i32,
    ) {
        if curl + 1 > k {
            let s: i32 = arr.iter().sum();
            // println!("{:?}",arr);
            if s == n {
                ans.push(arr.clone())
            }
            return;
        }
        for i in sin..=max {
            arr[curl as usize] = i;
            xx(i + 1, k, n, curl + 1, arr, ans, max);
        }
    }
    xx(1, k, n, 0, &mut v, &mut ans, max);
    ans
}

#[cfg(test)]
mod test {
    use super::{combination_sum3, combination_sum_xxxx};

    #[test]
    fn test_empty() {
        let x = combination_sum_xxxx(3, 7);
        println!("{:?}", x)
    }
}
