

fn main () {

}

// https://www.notion.so/Calculate-average-5af64a089e234ef188abe7da0af40cae?pvs=4
// Write a function which calculates the average of the numbers in a given list.
//
// Note: Empty arrays should return 0.

fn find_average(slice: &[f64]) -> f64 {
    let length = slice.len();
    if length == 0 { 0.0 } else { slice.iter().sum::<f64>() / length as f64 }
}

// https://www.codewars.com/kata/5513795bd3fafb56c200049e/solutions/rust
// Create a function with two arguments that will return an array of the first n multiples of x.
//
// Assume both the given number and the number of times to count will be positive numbers greater than 0.
//
// Return the results as an array or list ( depending on language ).

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
    use crate::find_average;

    #[test]
    fn count_by_works() {
        let result = count_by(1, 10);
        assert_eq!(vec![1,2,3,4,5,6,7,8,9,10], result);
    }

    #[test]
    fn find_average_works() {
        let input = [
            17.0, 16.0, 16.0, 16.0, 16.0, 15.0, 17.0, 17.0, 15.0, 5.0, 17.0, 17.0, 16.0,
        ];

        let result = find_average(&input);
        assert_eq!(15.384615384615385, result);
    }
}

