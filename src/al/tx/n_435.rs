
#[cfg(test)]
mod test{
    use std::process::id;


    //g hungry s cokkie
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {

        intervals.sort_by(|a,b| {

            if a[1] < b[1]{
                std::cmp::Ordering::Less
            }else if a[1] > b[1]{
                std::cmp::Ordering::Greater
            }else {
                if a[0] < b[0]{
                    std::cmp::Ordering::Less
                }else {
                    std::cmp::Ordering::Greater
                }
            }
        });

        let mut ans = 0;
        let mut a = &intervals[0];
        
        let mut idx = 1;
        while idx < intervals.len() {
            
            if intervals[idx][1] >= a[1] && intervals[idx][0] < a[1]{
                ans+=1;
            }else {
                a = &intervals[idx];
                
            }
            idx+=1;
        }

        ans
    }

    #[test]
    pub fn work(){
        let mut s = vec![[81,97],[-71,60],[36,97],[76,96],[59,68],[54,88],[-65,40],[83,84],[27,50],[-59,-50],[73,78],[50,57],[-49,81],[-16,90],[-83,-23],[-58,98],[78,99],[-57,81],[-2,85],[-88,45],[85,90],[-64,17],[76,78],[-17,5],[-98,15],[86,100]];
        let mut a = vec![];
        for ele in s {
            a.push(ele.to_vec())
        }
        let ans = erase_overlap_intervals(a);
        println!("{:?}",ans)
    }

}