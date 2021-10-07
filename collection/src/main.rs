fn main() {
    let mut v1 = Vec::new();
    let v2 = vec![1,2,3,4];
    v1.push(1);
    v1.push(2);

    println!("{:?} {:?}", v1, v2);
    
    let e1 = &v1[0];
    v1.push(1);
    v1.push(1);
    v1.push(1);
    v1.push(1);
    v1.push(1);
    v1.push(1);
    v1.push(1);
    println!("{}", e1);
}
