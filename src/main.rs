use std::io;

struct Rectangle{
    length: u32,
    width: u32
}

impl Rectangle{
    fn perimeter(&self)->u32{
        2*(self.length+self.width)
    }
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

    let area = area_tuple((length,width));

    println!("The area of the rectangle is {area} using a tuple");

    let rect_dim = Rectangle{
        length:50,
        width:50
    };
    println!("The area of the rectangle using Struct is {}", area_struct(&rect_dim));
    println!("The perimeter of the rectangle using Struct is {}", rect_dim.perimeter())
}

fn area_tuple(dim: (i32,i32)) -> i32 {
    dim.0 * dim.1
}

fn area_struct(dim: &Rectangle) -> u32 {
    dim.length * dim.width
}
