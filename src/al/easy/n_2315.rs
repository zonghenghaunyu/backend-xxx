#[cfg(test)]

mod test {

    pub fn count_asterisks(s: String) -> i32 {
        s.chars().into_iter().fold((0,true), |(ans,odd),a| {

            let mut odd = odd;
            let mut ans = ans;
            if a == '|' && odd{
                odd =  false;
            }else if a == '|' && !odd{
                odd = true;
            }else if a == '*' && odd{
                ans+=1;
            }

            (ans,odd)
        }).0
    }
    #[test]
    pub fn test() {
        let s = count_asterisks("s".to_string());
        println!("{}", s)
    }
}
