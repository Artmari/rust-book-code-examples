use std::io;

pub fn rect_area() {
    #[derive(Debug)]
    struct Rect {
        width: i32,
        height: i32,
    }

    let mut width = String::new();
    println!("Enter width");
    io::stdin()
        .read_line(&mut width)
        .expect("Failed to read line");

    let width: i32 = width
        .trim() // ignore whitespace around input
        .parse() // convert to integers
        .expect("Input not an integer");

    let mut height = String::new();
    println!("Enter height");
    io::stdin()
        .read_line(&mut height)
        .expect("Failed to read line");

    let height: i32 = height.trim().parse().expect("Input not an integer");

    println!("You entered: width: {}, height: {}", width, height);

    fn set_dimentions(width: i32, height: i32) -> Rect {
        Rect { width, height }
    }

    fn calculate_rect_area(dimentions: (i32, i32)) -> i32 {
        dimentions.0 * dimentions.1
    }

    let current_rect = set_dimentions(width, height);

    println!(
        "The area of the rectangle is {} square pixels.",
        calculate_rect_area((current_rect.width, current_rect.height))
    );
}
