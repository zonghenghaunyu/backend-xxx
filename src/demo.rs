#[cfg(test)]
mod tests {

    use rand::Rng;

    #[test]
    fn it_works() {
        let mut data: Vec<*const u8> = Vec::new();

        for _ in 0..5 {
            let mut num: Vec<u8> = Vec::new();
            for _ in 0..16 {
                let rand_num: u8 = rand::thread_rng().gen();
                num.push(rand_num)
            }
            println!("num is : {:?},num.as_ptr(): {:?}", num, num.as_ptr());
            data.push(num.as_ptr());
        }

        println!("data is: {:?}", data);
    }

    #[test]
    fn it_works2() {
        let mut data2: Vec<*const u8> = Vec::new();
        let mut data: Vec<*const [u8]> = Vec::new();
        for _i in 0..5 {
            let mut num: Vec<u8> = Vec::new();
            for _j in 0..16 {
                let rand_num: u8 = rand::thread_rng().gen();
                num.push(rand_num);
            }
            println!("num({:?}) is {:?} = failed", num.as_ptr(), num);
            data2.push(num.as_ptr());
            println!("num({:p}) is : {:?}", &*num, num);
            let boxed = num.into_boxed_slice();
            data.push(Box::into_raw(boxed) as _);
        }
        println!("data is: {:?}", data);
        println!("failed data is: {:?}", data2);

        for ele in data {
            let x = unsafe { Box::from_raw(ele as *mut [u8]) };
            println!("inner = {:?}", x.into_vec());
        }
        for ele in data2 {
            unsafe {
                let slice = std::slice::from_raw_parts(ele, 16);
                println!("{:?}", slice);
                println!("{}", *ele)
            }
        }
    }

    #[test]
    fn get_addr() {
        // let a = Box::new(4);
        let xx = vec![1, 2, 3];
        // let b = 5;
        println!("{:p}", xx.as_ptr());
        println!("{:p}", &*xx)
    }
}
