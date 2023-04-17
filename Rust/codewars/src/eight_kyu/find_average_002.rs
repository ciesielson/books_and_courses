// https://www.notion.so/Calculate-average-5af64a089e234ef188abe7da0af40cae?pvs=4
// Write a function which calculates the average of the numbers in a given list.
//
// Note: Empty arrays should return 0.

pub fn find_average(slice: &[f64]) -> f64 {
    let length = slice.len();
    if length == 0 { 0.0 } else { slice.iter().sum::<f64>() / length as f64 }
}

#[cfg(test)]
mod tests {
    use crate::eight_kyu::find_average_002::find_average;

    #[test]
    fn find_average_works() {
        let input = [
            17.0, 16.0, 16.0, 16.0, 16.0, 15.0, 17.0, 17.0, 15.0, 5.0, 17.0, 17.0, 16.0,
        ];

        let result = find_average(&input);
        assert_eq!(15.384615384615385, result);
    }
}
