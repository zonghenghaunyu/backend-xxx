pub fn partition(s: String) -> Vec<Vec<String>> {
    let chars = s.chars().collect::<Vec<char>>();

    let mut ans = Vec::<Vec<String>>::new();
    let mut inner = Vec::<String>::new();

    inner_way(&mut ans, &mut inner, &chars, 0);

    ans
}

fn inner_way(ans:&mut Vec<Vec<String>>,inner : &mut Vec<String>,chars : &Vec<char>,start_idx : usize){

    let len = chars.len();
    if start_idx == len {
        ans.push(inner.clone())
    }

    for i in start_idx..len {
        let s = chars[start_idx..=i].iter().map(|x| *x).collect::<Vec<char>>();
        if is_right(&s) {
            inner.push(s.iter().collect::<String>());
            inner_way(ans, inner, chars, i + 1);
            inner.pop();
        }
    }
}

fn is_right(arr: &Vec<char>) -> bool{

    let len = arr.len();
    let mid = len / 2;
    
    for i in 0..mid {
        if arr[i] != arr[len - 1 - i] {
            return false
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::{is_right,partition};

    #[test]
    fn test_partition() {
        let x = partition("aab".to_string());
        println!("{:?}", x)
    }


    #[test]
    fn test_is_right() {
        // let x = partition("aab".to_string());
        // println!("{:?}", x)
        let s = vec!['d','b','d','b','a'];
        let sd = is_right(&s);
        println!("{}",sd)
    }
}