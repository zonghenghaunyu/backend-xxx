mod n_152;
pub mod n_654;
mod n_670;
mod n_816;

#[cfg(test)]
mod tests {
    fn print_number(maybe_number: Option<u16>) {
        println!("printing: {}", maybe_number.unwrap());
    }
    #[test]
    fn do_test() {
        print_number(Option::Some(13));
        print_number(Option::Some(99));

        let mut numbers = Vec::new();
        for iter in 0..5 {
            let number_to_add = ((iter * 1235) + 2) / (4 * 16);

            numbers.push(Some(number_to_add));
            // numbers[iter as usize] = Option::Some(number_to_add as u16);
        }
    }
}
