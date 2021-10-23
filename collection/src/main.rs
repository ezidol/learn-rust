fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![11,332,55,33,22];
    let result = largest_i32(&number_list);
    println!("{}", result);
    
    let char_list = vec!['e','a','c'];
    let result = largest_char(&char_list);
    println!("{}", result);
}