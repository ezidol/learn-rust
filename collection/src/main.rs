fn main() {
    for c in "안녕하세요".chars() {
        println!("{}", c);
    }
    for b in "안녕하세요".bytes() {
        println!("{}", b);
    }
    let s1 = String::from("Hello ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    let s4 = &s3[..];
    println!("{}", s4);

    for c in s4.chars() {
        println!("{}", c);
    }

    let s5 = "Hello world";
    let s6 = s5;
    let s7 = String::from("Bye");
    let s8 = &s7;

    println!("{}", s6);
    println!("{}", s5);
    println!("{}", s8);
    println!("{}", s7);
}