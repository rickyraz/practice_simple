#![allow(dead_code)]
#![allow(unused_variables)]

use std::io;

fn main() {
    // temp_calc();
    // nth_fibonacci_number();
    // area_of_rectangle();

    let mut users = ["John", "Jane", "Jack", "Jill"];
    println!("Before: {}", users[0]);

    // mutate array element 0
    users[0] = "John Doe";
    println!("After: {}", users[0]);
}

fn temp_calc() {
    let celcius_temp: i32 = 90;
    let faren_temp: f32 = 132.8;

    let test1 = convert_c_to_f(celcius_temp);
    let test2 = convert_f_to_c(faren_temp);

    println!("{}°F", test1);
    println!("{}°C", test2);
    fn convert_c_to_f(temp: i32) -> i32 {
        (temp * 9 / 5) + 32
    }
    fn convert_f_to_c(temp: f32) -> f32 {
        (temp - 32.0) * 5.0 / 9.0
    }
}

fn nth_fibonacci_number() {
    // for nomer in 0..13 {
    //     println!(" fibonacci ({}) => {}", nomer, fib_sequence(nomer))
    // }

    println!("To end the program, type `exit` ");

    loop {
        println!("Type a positive integer");

        let mut int = String::new();
        io::stdin().read_line(&mut int).expect("");

        if int.trim() == "exit" {
            break;
        }

        // Shadowing to convert String to integer
        let int: u32 = match int.trim().parse() {
            Ok(int) => int,
            Err(_) => continue,
        };

        println!("Fibonacci ({}) => {}", int, fib_sequence(int));
    }

    fn fib_sequence(n: u32) -> u32 {
        if n <= 0 {
            return 0;
        } else if n == 1 {
            return 1;
        } else {
            // Fibonacci sequence by recursively
            return fib_sequence(n - 1) + fib_sequence(n - 2);
        }
    }
}

fn area_of_rectangle() {
    // let width = 30;
    // let height = 50;
    // let rect1 = (24, 90);

    let scale = 2;

    let rect2 = Rectangle {
        width: dbg!(38 * scale),
        height: 10,
    };

    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect4 = Rectangle {
        width: 60,
        height: 45,
    };

    dbg!(&rect2);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width, height)
    // );

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area2(rect1)
    // );

    println!(
        "The area of the rectangle is {} square pixels - using fn area.",
        area3(&rect2)
    );

    println!(
        "The area of the rectangle is {} square pixels - using impl area.",
        rect2.area()
    );

    if rect2.width() {
        println!("The rectangle has a nonzero width; it is {}", rect2.width);
    }

    println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect3));
    println!("Can rect1 hold rect3? {}", rect2.can_hold(&rect4));

    println!("rect2 is {:?}", rect2);

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // reading (&self), mutating (&mut self), or consuming (self).
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn width(&self) -> bool {
            self.width > 0
        }
        // Methods with More Parameters
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    // -- The area function is supposed to calculate the area of one rectangle, but the function we wrote has two parameters, and it’s not clear anywhere in our program that the parameters are related. It would be more readable and more manageable to group width and height together.

    // fn area(width: u32, height: u32) -> u32 {
    //     width * height
    // }

    // fn area2(dimensions: (u32, u32)) -> u32 {
    //     dimensions.0 * dimensions.1
    // }

    fn area3(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}
