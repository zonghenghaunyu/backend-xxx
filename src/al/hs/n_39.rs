
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

    let level : usize = 0;
    let mut list = Vec::<i32>::new();
    let mut ans = Vec::<Vec<i32>>::new();

    // let mut candidates = candidates.clone();
    // candidates.sort();

    fn inner (candidates: &Vec<i32>,list : &mut Vec<i32>,ans : &mut Vec<Vec<i32>>,level : usize,up_sum : i32,target: i32,start : usize){
        let len = candidates.len();
        if up_sum == target {
            let s = list[0..level].iter().map(|x| *x).collect::<Vec<_>>();
            ans.push(s.clone());
            return;
        }
        if up_sum > target {
            return;
        }
        for i in start..len {
            if level < list.len() {
                list.insert(level, candidates[i]);
            } else {
                list.push(candidates[i]);
            }
            let sum : i32= list[0..=level].iter().map(|x| *x).sum();
            if sum > target {
                // return;

                continue;
            }
            // let sum : i32 = list.iter().sum();
            inner(candidates,list,ans,level + 1,sum,target,i);

        }


    }
    inner(&candidates, &mut list, &mut ans, level, 0, target,0);
    ans
}

pub fn combination_sum1(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

    fn inner (candidates: &Vec<i32>,list : &mut Vec<i32>,ans : &mut Vec<Vec<i32>>,up_sum : i32,target: i32,start : usize){
        let len = candidates.len();
        if up_sum == target {
            ans.push(list.clone());
            return;
        }else if up_sum > target {
            return;
        }
        for i in start..len {
            list.push(candidates[i]);
            let sum : i32= list.iter().map(|x| *x).sum();
            if sum > target {
                list.pop();
                continue;
            }
            inner(candidates,list,ans,sum,target,i);
            list.pop();
        }
    }

    let mut list = Vec::<i32>::new();
    let mut ans = Vec::<Vec<i32>>::new();
    inner(&candidates, &mut list, &mut ans,  0, target,0);
    ans
}

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

    fn inner (candidates: &Vec<i32>,list : &mut Vec<i32>,ans : &mut Vec<Vec<i32>>,up_sum : i32,target: i32,start : usize){
        let len = candidates.len();
        if up_sum == target {
            ans.push(list.clone());
            return;
        }else if up_sum > target {
            return;
        }
        let mut used = Vec::<i32>::new();
        for i in start..len {
            let val = candidates[i];
            if used.contains(&val) {
                continue;
            }
            list.push(val);
            used.push(val);
            let sum : i32= list.iter().map(|x| *x).sum();
            if sum > target {
                list.pop();
                return;
            }
            inner(candidates,list,ans,sum,target,i + 1);
            list.pop();
        }
    }

    let mut candidates = candidates.clone();
    candidates.sort();
    let mut list = Vec::<i32>::new();
    let mut ans = Vec::<Vec<i32>>::new();
    inner(&candidates, &mut list, &mut ans,  0, target,0);
    ans
}


#[cfg(test)]
mod tests {

    use super::{ combination_sum1,combination_sum2};
    #[test]
    fn it_works2() {
        let a = combination_sum2(vec![10,1,2,7,6,1,5],8);
        println!("{:?}",a)
    }
    #[test]
    fn it_works() {

        let a = combination_sum1(vec![2,7,6,3],7);

        println!("{:?}",a)
    }


}