

#[cfg(test)]
mod tests {
    // use std::cmp::min;

    #[test]
    fn it_works() {

        let s = vec![10,15,20];
        let ans = min_cost_climbing_stairs(s);
        println!("{}",ans)

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
}

