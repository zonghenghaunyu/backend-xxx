mod chidouzi;
mod n_343;
mod n_62;
mod n_746;

#[cfg(test)]
mod tests {
    use std::f32::consts::E;


    #[test]
    fn it_works() {
        let mut s = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        // let mut s = vec![4,3,5,67,8,66,5,23];
        quick_sort(&mut s);


        println!("{:?}",s);
    }

    fn quick_sort(arr :& mut Vec<i32>){

        quick_inner(arr,0,arr.len() - 1 );

    }



    fn quick_inner(arr :& mut Vec<i32>,start:usize,end : usize){

        if start > end {
            return;
        }

        let mut low = start;
        let mut high = end;

        let temp = arr[high];
        while low < high {
            
            while low < high && arr[low] <= temp {
                low += 1;
            }

            while low < high && arr[high] >= temp {
                high -= 1;
            }
            if low < high {
                let t = arr[low];
                arr[low] = arr[high];
                arr[high] = t; 
            }
            println!("{:?}",arr)

        }
        arr[end] = arr[high];
        arr[high] = temp;
        if high > 0 {
            quick_inner(arr, start, high - 1);
        }
         
         quick_inner(arr, high + 1, end)


    }

} 