use std::collections::HashMap;

fn main() {
    println!("Hello, statistics!");
    let res = calculate_mean(vec![1, 2, 2, 5, 3]);
    println!("mean: {}", res);
    let mut items = vec![1, 2, 3];
    let res = calculate_median(&mut items);
    println!("median: {}", res);
    let res = calculate_mode(&mut items);
    println!("mode: {}", res);
}

fn calculate_mean(data: Vec<i32>) -> f32 {
    let len = data.len();
    let mut counter: i32 = 0;
    for i in data {
        counter += i
    }
    return counter as f32 / len as f32;
}

fn calculate_median(data: &mut Vec<i32>) -> i32 {
    data.sort();
    let mid = data.len() / 2;
    return data[mid];
}

fn calculate_mode(data: &Vec<i32>) -> i32 {
    let mut occurrences = HashMap::new();
    for &i in data {
        *occurrences.entry(i).or_insert(0) += 1;
    }
    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Opps err")
}
