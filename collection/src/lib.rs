pub trait Greeting {
    fn hello(&self) -> String {
        String::from("기본이다")
    }
    fn hello_all(&self) -> String {
        format!("Everybody {}", self.hello())
    }
}

pub trait Speaking {
    fn talk(&self) -> String {
        String::from("그냥")
    }
}

pub struct Human {
    pub name: String,
    pub age: u32,
}

impl Greeting for Human {
    fn hello(&self) -> String {
        String::from("안녕하세요")
    }
}

impl Speaking for Human {}

pub struct Dog {
    pub name: String,
    pub age: u32,
}

impl Greeting for Dog {}

impl Speaking for Dog {}

pub fn hi<T: Greeting + Speaking>(item: &T) {
    println!("{} {}", item.hello_all(), item.talk());
}