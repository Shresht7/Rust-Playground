use std::collections::HashMap;

fn calculate_mean(list: &[i32]) -> f64 {
    let mut sum = 0.0;
    for number in list {
        sum += *number as f64;
    }
    sum / list.len() as f64
}

fn calculate_median(list: &[i32]) -> f64 {
    let sorted_list = sort_list(&list);
    let len = list.len();
    if len % 2 == 0 {
        sorted_list[(len) / 2] as f64
    } else {
        let n = sorted_list[(len - 1) / 2];
        let m = sorted_list[(len) / 2];
        ((n + m) / 2) as f64
    }
}

fn sort_list(list: &[i32]) -> Vec<i32> {
    let mut v = Vec::from(list);
    for j in 0..list.len() {
        for i in 0..list.len() - 1 - j {
            if v[i] > v[i + 1] {
                let temp = v[i];
                v[i] = v[i + 1];
                v[i + 1] = temp;
            }
        }
    }
    println!("{:?}", v);
    v
}

fn calculate_mode(list: &[i32]) -> i32 {
    let mut map = HashMap::new();
    for number in list {
        let freq = map.entry(number).or_insert(0);
        *freq += 1;
    }
    let mut mode = 0;
    for (key, value) in &map {
        let current = map.get(&mode).unwrap_or(&0);
        if value > current {
            mode = **key;
        }
    }
    mode
}

fn main() {
    let list = vec![7, 5, 2, 7, 7, 1, 8, 4, 7, 6];
    let mean = calculate_mean(&list);
    let median = calculate_median(&list);
    let mode = calculate_mode(&list);
    println!("Mean: {mean}, Median: {median}, Mode: {mode}");
}
