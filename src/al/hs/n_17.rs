use std::collections::HashMap;

struct Solution{}
impl Solution {
    
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map = Self::get_map();
        let s: Vec<char> = digits.chars().into_iter().collect();

        let len = s.len();
        let mut tmp = vec!['0'; len];

        let mut ans = Vec::<String>::new();
        fn inner(
            s: &Vec<char>,
            map: &HashMap<char, Vec<char>>,
            len: usize,
            temp: &mut Vec<char>,
            ans: &mut Vec<String>,
            idx: usize,
        ) {
            if idx >= len {
                // src1.iter().collect::<String>()
                ans.push(temp.clone().iter().collect::<String>());
                // ans.push(String::from_iter(temp.clone()));
                return;
            }
            let next = s[idx];
            let oo = map.get(&next);
            let arr = oo.unwrap();
            for ele in arr {
                temp[idx] = *ele;

                inner(s, map, len, temp, ans, idx + 1)
            }
        }

        inner(&s, &map, len, &mut tmp, &mut ans, 0);
        ans
    }

    
    fn get_map() -> HashMap<char, Vec<char>> {
        let mut map = HashMap::new();
        map.insert('2', vec!['a', 'b', 'c']);
        map.insert('3', vec!['d', 'e', 'f']);
        map.insert('4', vec!['g', 'h', 'i']);
        map.insert('5', vec!['j', 'k', 'l']);
        map.insert('6', vec!['m', 'n', 'o']);
        map.insert('7', vec!['p', 'q', 'r', 's']);
        map.insert('8', vec!['t', 'u', 'v']);
        map.insert('9', vec!['w', 'x', 'y', 'z']);
        map
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;



    #[test]
    fn it_works() {
        let a = Solution::letter_combinations("23".to_string());
        // let digits = String::from("23");
        // let s : Vec<char> = digits.chars().into_iter().collect();

        println!("{:?}", a)
    }
}



// Line 22, Char 34: no function or associated item named `from_iter` found for struct `std::string::String` in the current scope (solution.rs)
//    |
// 22 |                 ans.push(String::from_iter(temp.clone()));
//    |                                  ^^^^^^^^^ function or associated item not found in `std::string::String`
//    |
//    = help: items from traits can only be used if the trait is in scope
// help: the following trait is implemented but not in scope; perhaps add a `use` for it:
//    |
// 1  | use std::iter::FromIterator;
//    |
// help: there is an associated function with a similar name
//    |
// 22 |                 ans.push(String::from_utf8(temp.clone()));
//    |                                  ~~~~~~~~~
// For more information about this error, try `rustc --explain E0599`.
// error: could not compile `prog` due to previous error
// mv: cannot stat '/leetcode/rust_compile/target/release/prog': No such file or directory