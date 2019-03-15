struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect = Rectangle {width: 30, height: 30};
    let mut s = String::from("foo");





    println!("The area of the rectangle is {}", area(&rect));
    println!("The dimensions are {} x {}", rect.width, rect.height);
    
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height;

}
