fn main() {
    let list = vec![3,4,1,2,6,7,8,9,5,4,1,2,2,2,2,2,33,6324,32123,222];
    let mean = get_mean(&list);
    let median = get_median(&list);
    let mode = get_mode(&list);
    let a = 3;
    let b = &a;

    println!("{} {} {} {}", mean, median, mode, b);

}

fn get_mean(list:&Vec<i32>) -> i32 {
    let mut mean = 0;
    let list_len = list.len() as i32;
    for x in list {
        mean += x;
    }
    mean = mean / list_len;
    mean
}

fn get_median(list:&Vec<i32>) -> i32 {
    let mut list_clone:Vec<i32> = list.clone();
    let list_len = list.len() / 2;

    list_clone.sort();

    let list_sorted = list_clone;

    list_sorted[list_len]    
}

fn get_mode(list:&Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut mode_list = HashMap::new();
    let mut mode_count = 0;
    let mut mode = 0;
    for x in list {
        let count = mode_list.entry(x).or_insert(0);
        *count += 1;
    }
    for (&key, _) in &mode_list {
        let value = mode_list[key];
        if value >= mode_count {
            mode_count = value;
            mode = *key;
        }
    }
    mode
}