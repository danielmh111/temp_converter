use std::io;


fn set_input_unit() -> char {
    loop {
        
        let mut input_unit = String::new();

        println!("enter the unit of the input (options: F or C or K)");

        io::stdin()
            .read_line(&mut input_unit)
            .expect("could not read line");

            let input_unit: char = match input_unit
            .trim()
            .parse() {
                Ok(chr) => chr,
                Err(_) => {
                    println!("did not recognise that unit. you entered '{input_unit}' Please enter one of F or C or K.");
                    continue;
                },
        };

        let valid_units: [char;3] = ['F', 'C', 'K'];

        if ! valid_units.contains(&input_unit) {
            println!("did not recognise that unit. you entered '{input_unit}' Please enter one of F or C or K.");
            continue;
        }

        return input_unit

    };

}


fn main() {
    
    let mut input_temp = String::new();
    
    let input_unit = set_input_unit();

    println!("the unit is {input_unit}");

    println!("enter a temperature. Do not enter the unit.");

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
