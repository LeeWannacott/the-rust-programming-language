#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50, 
    };
  let rect2 = Rectangle {
        width: 60,
        height: 90, 
    };

    println!(
        "The area of the rectangle one is {} square pixels.",
        area(&rect1));

    println!("rect2 struct is {:?}",rect2);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
