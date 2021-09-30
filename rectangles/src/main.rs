#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.width
    }
    fn square(size:u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    // Listing 5-8: Calculating the area of a rectangle specified by separate width and height variables
    let width1 = 55;
    let height1 = 100;
    
    println!(
        "The area of the reactangle is {} square pixels.",
        area1(width1, height1)
    );

    //Listing 5-9 shows another version of our program that uses tuples.
    let rect1 = (30, 50);

    println!(
        "The area of the reactangle is {} square pixels.",
        area2(rect1)
    );

    //Refactoring with Structs: Adding More Meaning 
    let rect1 = Rectangle {
        width: 300,
        height: 500,
    };
    println!(
        "The area of the reactangle is {} square pixels.",
        area3(&rect1)
    );
    println!("rect1 is {:#?}", rect1);
    
    //Defining an area method on the Rectangle struct
    let rect1 = Rectangle {
        width: 500,
        height: 400,
    };

    println!("rect1 is {}", rect1.area());

    //Methods with More Parameters
    let rect2 = Rectangle {
        width: 300,
        height: 200,
    };
    let rect3 = Rectangle {
        width: 600,
        height: 400,
    };

    println!("rect1 can hold rect2 {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3 {}", rect1.can_hold(&rect3));

    //Associated Functions

    let sq1 = Rectangle::square(30);
    println!("{:#?}", sq1);
}   

fn area1(width1: u32, height1: u32) -> u32 {
    width1 * height1
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}