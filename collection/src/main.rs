use collection::{Human, Dog, Greeting, hi};
fn main() {
    let h1 = Human {
        name: String::from("영민"),
        age: 32,
    };
    let d1 = Dog {
        name: String::from("사모예드"),
        age: 2,
    };
    fn returns_human() -> impl Greeting {
        Human {
            name: String::from("사모"),
            age: 11,
        }
    }
    println!("{}이 {}라고 인사했다", h1.name, h1.hello_all());
    println!("{}가 {}라고 인사했다", d1.name, d1.hello_all());
    hi(&d1);
    println!("{}", returns_human().hello_all());
    
    
}