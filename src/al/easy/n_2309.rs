#[cfg(test)]
mod test {
    use std::collections::HashSet;

    pub fn greatest_letter(s: String) -> String {
        let mut word = HashSet::<char>::new();

        for ele in s.chars() {
            if ele >= 'a' && ele <= 'z' || ele >= 'A' && ele <= 'Z' {
                word.insert(ele);
            }
        }
        let mut biggest: char = 0 as char;
        for ele in &word {
            if *ele > biggest && *ele < 'Z' && word.contains(&((*ele as u8 + 32) as char)) {
                biggest = *ele;
            }
        }
        if biggest == 0 as char {
            "".to_string()
        } else {
            biggest.to_string()
        }
    }
    pub fn hhh(s: String) -> String {
        let word = s
            .chars()
            .filter(|ele| *ele >= 'a' && *ele <= 'z' || *ele >= 'A' && *ele <= 'Z')
            .collect::<HashSet<_>>();
        // word.iter().fold(0 as char, |ele,biggest| {
        //     if ele > *biggest && ele < 'Z' && word.contains(&((ele as u8+ 32) as char)){
        //         ele
        //     }else {
        //         *biggest
        //     }
        // }).to_string()
        // let mut biggest: char  = 0 as char;

        // word.iter().for_each(|ele| {
        //     if *ele > biggest && *ele < 'Z' && word.contains(&((*ele as u8+ 32) as char)){
        //         biggest = *ele;
        //     }
        // });
        let biggest = word.iter().fold(0 as char, |biggest, ele| {
            if *ele > biggest && *ele < 'Z' && word.contains(&((*ele as u8 + 32) as char)) {
                *ele
            } else {
                biggest
            }
        });
        if biggest == 0 as char {
            "".to_string()
        } else {
            biggest.to_string()
        }
    }

    #[test]
    pub fn test() {
        let s = hhh("lEeTcOdE".to_string());
        // let mut s = 0 as char;
        let w = s.to_string();
        println!("{}", s)
    }
}
