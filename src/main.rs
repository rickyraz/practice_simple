#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::io;

fn temp_calc() {
    let celcius_temp: i32 = -275;
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

    loop {
        println!("Type a positive integer");

        let mut int: String = String::new();
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
}

fn area_of_rectangle() {
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

    // let width = 30;
    // let height = 50;
    // let rect1 = (24, 90);

    let scale = 2;

    let rect2 = Rectangle {
        width: dbg!(22 * scale),
        height: 42,
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
        // cause addition to providing method syntax and not having to repeat the type of self in every method’s signature.
        "The area of the rectangle is {} square pixels - using impl area.",
        rect2.area()
    );

    if rect2.width() {
        println!("The rectangle has a nonzero width; it is {}", rect2.width);
    }

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));
    println!("rect2 is {:?}", rect2);
}

fn mode_median_average() {
    fn average(numbers: &[i32]) -> f32 {
        numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
    }

    fn median(numbers: &mut [i32]) -> f64 {
        // numbers.sort();
        // let mid = numbers.len() / 2;
        // numbers[mid]

        if (numbers.len() % 2) == 0 {
            let ind_left = numbers.len() / 2 - 1;
            let ind_right = numbers.len() / 2;
            (numbers[ind_left] + numbers[ind_right]) as f64 / 2.0
        } else {
            numbers[(numbers.len() / 2)] as f64
        }
    }

    fn mode(numbers: &[i32]) -> i32 {
        let mut occurrences = HashMap::new();

        for &value in numbers {
            *occurrences.entry(value).or_insert(0) += 1;
        }

        occurrences
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .expect("Cannot compute the mode of zero numbers")
    }

    let mut numbers = [42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43, 78];

    println!("AVERAGE: {}", average(&numbers));
    println!("MEDIAN: {}", median(&mut numbers));
    println!("MODE: {}", mode(&numbers));
}

fn main() {
    // temp_calc();
    // nth_fibonacci_number();
    area_of_rectangle();
    // mode_median_average();
}
