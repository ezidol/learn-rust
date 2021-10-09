fn main() {
    let list = vec![3,4,1,2,6,7,8,9,5,33,6324,32123,222];
    let mean = get_mean(&list);
    let median = get_median(&list);

    println!("{} {}", mean, median);
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