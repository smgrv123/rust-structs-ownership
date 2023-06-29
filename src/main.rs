use std::io;

struct Rectangle{
    length: String,
    width: String
}

fn main() {
    let mut dim_rect:(String, String)=("".to_owned(),"".to_owned());

    println!("Enter the length of the rectangle");

    io::stdin()
        .read_line(&mut dim_rect.0)
        .expect("Please enter the correct value");

    println!("Enter the width of the rectangle");

    io::stdin()
        .read_line(&mut dim_rect.1)
        .expect("Please enter the correct value");

    let length: i32 = dim_rect.0.trim().parse().expect("Please type a number");
    let width: i32 = dim_rect.1.trim().parse().expect("Please type a number");

    let area = area((length,width));

    println!("The area of the rectangle is {area}")
}

fn area(dim: (i32,i32)) -> i32 {
    dim.0 * dim.1
}
