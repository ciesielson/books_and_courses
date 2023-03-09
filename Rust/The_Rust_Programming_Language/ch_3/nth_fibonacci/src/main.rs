fn main() {
    fibonacci_iterative(14)
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

fn fibonacci_iterative(n: i32) {
    let mut result = vec![0, 1];

    for _i in 2..=n {
        let a = result[result.len() - 1];
        let b = result[result.len() - 2];

        result.push(a + b)
    }

    println!("The nth number is... {}", result[result.len() - 1])
}
