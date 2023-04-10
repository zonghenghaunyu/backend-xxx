
#[cfg(test)]
mod test{

    //g hungry s cokkie
    pub fn find_content_children(mut g: Vec<i32>,mut  s: Vec<i32>) -> i32 {

        g.sort();
        s.sort();
        
        let mut idx = 0;
        let mut ans = 0;
        for ele in &g {
            
            while idx < s.len() && *ele < s[idx] {
                idx += 1;
            }
            if idx < s.len(){
                ans +=1 ;
                idx += 1 ;
            }else {
                break;
            }

        }
        ans
    }

    #[test]
    pub fn work(){
        println!("works")
    }

}