// https://www.codewars.com/kata/5513795bd3fafb56c200049e/solutions/rust
// Create a function with two arguments that will return an array of the first n multiples of x.
//
// Assume both the given number and the number of times to count will be positive numbers greater than 0.
//
// Return the results as an array or list ( depending on language ).

fn main () {

}

fn count_by(x: u32, n: u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();

    for i in 1..=n {
        v.push(x * i);
    }
    v
}

#[cfg(test)]
mod tests {
    use crate::count_by;

    #[test]
    fn count_by_works() {
        let result = count_by(1, 10);
        assert_eq!(vec![1,2,3,4,5,6,7,8,9,10], result);
    }
}

