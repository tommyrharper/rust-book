// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectanlge is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// // Tuples
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectanlge is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Structs
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

    println!(
        "The area of the rectanlge is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 is {:#?}", rect1);

    dbg!(rect1);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);
}

// Make sure to borrow the rectangle, don't move it!
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
