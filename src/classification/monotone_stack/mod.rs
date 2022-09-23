pub mod N_738;

struct Node{

}
#[cfg(test)]
mod tests {
    use std::{rc::Rc, cell::Cell};

    #[test]
    fn itas(){

        let mut a = Rc::new(Cell::new(5));

        let mut b = a.clone();
        
        let c = b.get();

        b.set(9999);

        println!("{}",c);


        println!("{}",b.get());

        println!("{}",a.get());
        
    }

    #[test]
    fn it_works3() {

        let mut arr1 = Rc::new(vec![1,2,3]);

        
        printarr( arr1.clone());
        printarr( arr1.clone());
        printarr( arr1.clone());
        printarr( arr1);
    }

    fn printarr(arr1 : Rc::<Vec::<i32>>){
        
        for ele in &*arr1 {
            println!("{}",ele)
        }
        println!("<<<<<<<<<<<<")
    }

    #[test]
    fn it_works2() {

        let mut arr = vec![1,2,3];

        change( &mut arr);
        change( &mut arr);

        println!("{:?}",arr)
    }

    fn change( arr : &mut Vec::<i32>){

        arr.push(4);
        change2(arr)
    }

    fn change2( arr : &mut Vec::<i32>){

        arr.push(5)
        
    }

}