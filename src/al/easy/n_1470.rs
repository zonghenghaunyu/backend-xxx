pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let res = vec![];
    // for i in 0..n{
    //     res.push(nums[i as usize]);
    //     res.push(nums[(i+n) as usize]);
    // }

    // res

    let ccc = nums.iter().skip(n as usize).collect::<Vec<_>>();

    let ans = nums
        .iter()
        .zip(ccc.iter())
        .map(|(&x, &y)| vec![x, *y])
        .flatten()
        .collect::<Vec<_>>();
    println!("{:?}", ccc);
    println!("{:?}", ans);
    // nums.iter()
    // .zip(nums.iter().skip(n as usize))
    // .map(|(&x, &y)| vec![x, y])
    // .flatten()
    // .collect()
    println!("--------------------------------------------------------------------");
    res
}

#[cfg(test)]
mod tests {
    use super::shuffle;

    #[test]
    fn it_works1() {
        let a = vec![1, 2, 3, 4, 5, 6];
        let b = shuffle(a, 3);
        println!("{:?}", b)
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
