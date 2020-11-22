pub fn say_hello() {
    println!("Hello world!");
}

pub fn print() {
    let array_numbers: [u8; 5] = [1, 2, 3, 4, 5];
    output_sequence(&array_numbers);
    let vector_numbers: Vec<u8> = vec![1, 2, 3, 4, 5];
    output_sequence(&vector_numbers);
}

fn output_sequence(numbers: &[u8]) {
    for n in numbers {
        println!("{}", n);
    }
}
