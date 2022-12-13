use std::collections::{HashMap, HashSet};

pub fn trap(height: Vec<i32>) -> i32 {
    let map = get_map(&height);

    if map.len() == 1 {
        return 0;
    }

    let mut arr = height.clone();
    arr.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut used_idx = HashSet::<usize>::new();

    let mut ans = 0;
    let len = arr.len();
    let max = arr[0].clone();

    let max_idxs = map.get(&max).unwrap();
    let mut left = max_idxs[0];
    let mut right = max_idxs[max_idxs.len() - 1];

    let mut max_not_unique = max_idxs.len() != 1;

    for i in 0..len {
        if max_not_unique {
            let tall = height[left];
            for j in (left + 1)..right {
                ans = ans + tall - height[i];
                used_idx.insert(j);
            }
            used_idx.insert(left as usize);
            used_idx.insert(right as usize);
            max_not_unique = false;
            continue;
        }
        let list1 = map.get(&height[i]).unwrap();

        let mut list = Vec::new();

        for ele in list1 {
            if !used_idx.contains(ele) {
                list.push(*ele)
            }
        }
        if list.len() == 0 {
            continue;
        } else if list.len() == 1 {
            let lower = list[0];
            let tall = height[lower];
            if lower < left {
                if used_idx.contains(&(lower + 1)) && used_idx.contains(&(left - 1)) {
                    continue;
                }
                for x in (lower + 1)..left {
                    if used_idx.contains(&x) {
                        continue;
                    }
                    ans = ans + tall - height[x];
                    used_idx.insert(x);
                }
                used_idx.insert(lower);
                left = lower;
            } else {
                if used_idx.contains(&(right + 1)) && used_idx.contains(&(lower - 1)) {
                    continue;
                }
                for x in (right + 1)..lower {
                    if used_idx.contains(&x) {
                        continue;
                    }
                    ans = ans + tall - height[x];
                    used_idx.insert(x);
                }
                used_idx.insert(lower);
                right = lower;
            }
        } else {
            list.sort();
            let left_wall = left;
            let right_wall = right;
            if left > list[0] {
                left = list[0];
            }
            if right < list[list.len() - 1] {
                right = list[list.len() - 1];
            }
            if left < left_wall {
                if used_idx.contains(&(left + 1)) && used_idx.contains(&(left_wall - 1)) {
                    continue;
                }
                let left_tall = height[left];
                for x in (left + 1)..left_wall {
                    if used_idx.contains(&x) {
                        continue;
                    }
                    ans = ans + left_tall - height[i];
                    used_idx.insert(x);
                }
                used_idx.insert(left_wall);
            }

            if right > right_wall {
                if used_idx.contains(&(right_wall + 1)) && used_idx.contains(&(right - 1)) {
                    continue;
                }
                let right_tall = height[right];
                for x in (left + 1)..left_wall {
                    if used_idx.contains(&x) {
                        continue;
                    }
                    ans = ans + right_tall - height[i];
                    used_idx.insert(x);
                }
                used_idx.insert(right_wall);
            }
        }

        used_idx.insert(left);
        used_idx.insert(right);
    }

    fn get_map(arr: &Vec<i32>) -> HashMap<i32, Vec<usize>> {
        let mut map = HashMap::<i32, Vec<usize>>::new();
        let len = arr.len();
        for i in 0..len {
            let op = map.get(&arr[i]);
            match op {
                Some(x) => {
                    let mut nr = x.clone();
                    nr.push(i);
                    map.insert(arr[i], nr);
                }
                None => {
                    let mut xx = Vec::new();
                    xx.push(i);
                    map.insert(arr[i], xx);
                }
            }
        }
        map
    }

    ans
}

pub fn get_map(arr: &Vec<i32>) -> HashMap<i32, Vec<usize>> {
    let mut map = HashMap::<i32, Vec<usize>>::new();
    let len = arr.len();
    for i in 0..len {
        let op = map.get(&arr[i]);
        match op {
            Some(x) => {
                let mut nr = x.clone();
                nr.push(i);
                map.insert(arr[i], nr);
            }
            None => {
                let mut xx = Vec::new();
                xx.push(i);
                map.insert(arr[i], xx);
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {

    use super::trap;

    #[test]
    fn it_works() {
        let a = trap(vec![]);

        println!("{:?}", a)
    }
}
