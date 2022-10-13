#[cfg(test)]
mod tests {
    // use std::cmp::min;

    #[test]
    fn it_works() {
        let s = vec![1,100,1,1,1,100,1,1,100,1];
        let ans = min_cost_climbing_stairs4(s);
        println!("{}", ans)
    }

    #[test]
    fn it_works2() {
        let s = vec![1,100,1,1,1,100,1,1,100,1];
        let ans = min_cost_climbing_stairs2(s);
        println!("{}", ans)
    }

    #[test]
    fn it_works3() {
        let s = vec![10, 15, 20];
        let ans = min_cost_climbing_stairs3(s);
        println!("{}", ans)
    }
    #[test]
    fn it_works5() {
        let s = vec![10, 15, 20];
        let ans = min_cost_climbing_stairs(s);
        println!("{}", ans)
    }
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {

        // if(cost.len() <= 1){
        //     return 0;
        // }
        // if(cost.len() == 2){
        //     let a = cost.get(0).unwrap();
        //     let b = cost.get(1).unwrap();
        //     return min(*a,*b);
        // }
        let mut costa = *cost.get(0).unwrap();
        let mut costb = *cost.get(1).unwrap();
        let mut ans;
        for i in 2..cost.len() {
            ans = costa.min(costb) + cost[i];
            costa = costb;
            costb = ans;
        }
        costa.min(costb)
    }
    pub fn min_cost_climbing_stairs4(cost: Vec<i32>) -> i32 {
        let mut a = (0,0);

        for i in 0..cost.len() {
            a = ((a.1).min(a.0 + cost[i]), a.0 + cost[i])
        }
        a.0
    }

    pub fn min_cost_climbing_stairs2(cost: Vec<i32>) -> i32 {
        cost.into_iter().fold((0, 0), |a, c| ((a.1).min(a.0 + c), a.0 + c)).0
    }

    pub fn min_cost_climbing_stairs3(cost: Vec<i32>) -> i32 {
        (2..=cost.len()).fold((0, 0), |t, i| (t.1, (t.1 + cost[i - 1]).min(t.0 + cost[i - 2]))).1
    }
}

