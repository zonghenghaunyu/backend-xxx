#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let x = integer_break(10);
        println!("{}", x)
    }

    pub fn integer_break(n: i32) -> i32 {
        //dp[i] = max(dp[i] * (j-i),i*(j-i))
        let idx = n as usize;
        let mut arr = vec![0; idx + 1];
        arr[0] = 0;
        arr[1] = 1;
        // 0 1 1 2 4
        //         3
        for i in 2..=idx {
            for j in 1..i {
                let cur = ((j * (i - j)) as i32).max(arr[j] * ((i - j) as i32));
                arr[i] = arr[i].max(cur);
            }
        }
        arr[idx]
    }
}
