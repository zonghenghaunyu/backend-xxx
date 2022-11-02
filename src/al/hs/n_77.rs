pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut aa = vec![0; k as usize];

    let mut ans = Vec::<Vec<i32>>::new();
    xxx(1, n - k + 1, &mut aa, &mut ans, 0, k as usize);
    ans
}

fn xxx(si: i32, ei: i32, a: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, f: usize, k: usize) {
    if f >= k {
        ans.push(a.clone());
        return;
    }
    for i in si..=ei {
        a[f] = i;
        xxx(i + 1, ei + 1, a, ans, f + 1, k);
    }
}

#[cfg(test)]
mod tests {
    use super::combine;

    #[test]
    fn it_works() {
        let a = combine(4, 2);
        println!("{:?}", a)
    }
}
