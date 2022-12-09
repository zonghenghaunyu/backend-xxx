
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {

        let arr :Vec<char> = s.chars().into_iter().collect();

        let mut all = Vec::new();

        let len = arr.len();

        for i in 1..len {
            let mut inner = Vec::new();
            for ele in &arr {
                if inner.len() == i {
                    inner.push(',');
                }
                inner.push(*ele);
            }
            let sp = inner.split(|a| *a == ',').collect::<Vec<_>>();
            println!("{:?}",sp);
            let first = sp.get(0).unwrap().clone().iter().map(|a| *a).collect::<Vec<_>>();
            let sceond = sp.get(1).unwrap().clone().iter().map(|a| *a).collect::<Vec<_>>();
            all.push((first,sceond));
        }

        let len = all.len();

        for i in 0 .. len{

            let tuple = all[i].clone();
            let first = tuple.0;
            let _second = tuple.1;

            let _flen = first.len();

            

            let _slen = first.len();


            println!("{:?},{},{}",second,flen,slen)
       }

        
        // println!("{:?}",all);
        vec![]
    }


#[cfg(test)]
mod tests {
    use super::ambiguous_coordinates;




    #[test]
    fn it_works1() {
        ambiguous_coordinates("1234".to_string());

    }

}
