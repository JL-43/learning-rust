#[derive(Debug)]

struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width*self.height
    }
}

fn main() {

    // calculate the size of a rectangle

    // 1

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle 1 is {} square pixels.",
        area1(width1, height1)
    );

    // 2 - with tuples
    let rect2 = (30, 50);

    println!(
        "The area of the rectangle 2 is {} square pixels.",
        area2(rect2)
    );

    // 3 - with structs
    // refer to struct defined outside of main

    let rect3 = Rectangle{width:30, height: 50};

    println!(
        "The area of the rectangle 3 is {} square pixels.",
        area3(&rect3)
    );

    // printing Rect

    // println!("rect3 is {}", rect3); // wont work. enable debug printing (see top of code)
    println!("rect3 is {:?}", rect3);
    println!("rect3 is {:#?}", rect3);

    //// 4 - methods
    // since the area function is specifically for Rectangles, we can create a method for Rectangles that computes for area
    // see "impl Rectangle" code outside of main

    let rect4 = Rectangle{width: 30, height: 50};

    println!(
        "The area of the rectangle 4 is {} square pixels.",
        rect4.area()
    );

}

// 1
fn area1(width: u32, height: u32) -> u32 {
    width*height
}

// 2
fn area2(dimensions:(u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// 3
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}