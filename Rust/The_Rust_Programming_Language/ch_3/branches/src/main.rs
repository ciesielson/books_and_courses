fn main() {
    let number = 3;

    if number < 5 {
        if_in_let_statement();
    } else {
        println!("condition was false");
    }
}

fn if_in_let_statement() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is... {number}");
}
