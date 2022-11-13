fn main() {
    temp_calc();
    nth_fibonacci_number();
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
    for nomer in 0..13 {
        println!(" fibonacci ({}) => {}", nomer, fib_sequence(nomer))
    }

    fn fib_sequence(n: i32) -> i32 {
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
