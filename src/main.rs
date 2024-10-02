use std::io;

fn main() {
    let mut input_temp = String::new();

    println!("enter a temperature in Faranheit. Do not enter the unit.");

    io::stdin()
        .read_line(&mut input_temp)
        .expect("could not read line");

    let input_temp: f64 = input_temp
        .trim()
        .parse()
        .expect("could not read value as a number");

    let output_temp: f64 = (input_temp - 32.0) * 5.0/9.0;

    println!("{input_temp}f = {output_temp}c");
}
