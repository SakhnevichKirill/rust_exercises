use std::collections::HashMap;

// Given a list of integers, use a vector and return the median (when sorted, the value in
// the middle position) and mode (the value that occurs most often; a hash map will be
// helpful here) of the list
// fn median(v: &Vec<u8>){
//     v
// }

fn calc_average(numbers: &Vec<i32>) -> f32 {
    // calculate average
    // let mut sum: i32 = 0;
    // for x in numbers {
    //     sum += x;
    // }
    // sum as f32 / numbers.len() as f32
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn calc_median(numbers: &mut Vec<i32>) -> i32 {
    // calculate median
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn calc_mode(numbers: &Vec<i32>) -> i32 {
    // calculate mode
    // new HashMap
    let mut times = HashMap::new();

    // count
    for &x in numbers {
        *times.entry(x).or_insert(0) += 1;
    }

    times
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

// fn calc_mode(numbers: &[i32]) -> Option<i32> {
//     let mut counts = HashMap::new();
//
//     numbers.iter().copied().max_by_key(|&n| {
//         let count = counts.entry(n).or_insert(0);
//         *count += 1;
//         *count
//     })
// }

pub fn test1() {
    let mut numbers = vec![42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43];

    // let avg: f32;
    // let median: i32;
    // let mode: i32;
    println!("AVERAGE: {}", calc_average(&numbers));

    println!("MEDIAN: {}", calc_median(&mut numbers));

    println!("MODE: {}", calc_mode(&numbers));

    // { // calculate mode
    //
    //
    // println!("AVERAGE: {}", avg);
    // println!("MEDIAN: {}", median);
    // println!("MODE: {}", mode)
}
