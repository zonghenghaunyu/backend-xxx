

pub fn restore_ip_addresses(s: String) -> Vec<String> {

    let mut ans = Vec::<String>::new();
    let chars = s.chars().into_iter().collect::<Vec<char>>();

    let mut tmp = Vec::<Vec<char>>::new();

    inner_way(1, &mut ans, &mut tmp, &chars, 0);

    ans
}

fn inner_way(level : usize,ans : &mut Vec<String>,tmp : &mut Vec<Vec<char>>,chars : &Vec<char>,idx : usize){

    if level == 4 {
        let x = chars[idx..].iter().map(|a| *a).collect::<Vec<char>>();
        if is_right(&x){
            tmp.push(x);
            let mut s = String::new();
            for i in 0..4 {
                let ss = tmp[i].iter().collect::<String>();
                s = s + &ss;
                if i != 3 {
                    s = s + ",";
                }
            }
            ans.push(s)
        }
        return;
    }

    let len = chars.len();

    for i in idx..len {

        let inn = chars[idx..=i].iter().map(|a| * a).collect::<Vec<char>>();
        if is_right(&inn){
            tmp.push(inn);
            inner_way(level + 1, ans, tmp, chars, i + 1);
            tmp.pop();
        }
    }
}

fn is_right(arr : &Vec<char>) -> bool {
    let s = arr.iter().collect::<String>();

    let option = s.parse::<i32>();
    match option {
        Ok(num) => {
            if num <= 255 {
                true
            }else {
                false
            }
        },
        Err(_e) => {
            false
        }
    }

}


#[cfg(test)]
mod test {
    use super::restore_ip_addresses;

    #[test]
    fn test_restore_ip_addresses() {
        let x = restore_ip_addresses("12345".to_string());
        println!("{:?}", x)
    }
}