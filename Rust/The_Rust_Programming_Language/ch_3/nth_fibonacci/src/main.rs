fn main() {
    generate_nth_fibonacci_number(19)
}

fn generate_nth_fibonacci_number(n: i32) {
    if n == 0 {
        println!("Generated number: {}", n);
    } else if n == 1 {
        println!("Generated number: {}", n);
    } else if n > 1 {
        let mut f_prim = 0;
        let mut f = 1;
        let mut generated_number: i32 = 0;

        for _i in 2..=n {
            generated_number = f + f_prim;
            f_prim = f;
            f = generated_number;
        }

        println!("Generated number: {}", generated_number);
    } else {
        println!("Only positive numbers are allowed");
    }
}
