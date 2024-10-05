use std::io;


fn convert_f_to_c(input_temp: &f64) -> f64 {
    let output_temp: f64 = (input_temp - 32.0) * 5.0/9.0;
    return output_temp
}

fn convert_f_to_k(input_temp: &f64) -> f64 {
    let output_temp: f64 = (input_temp + 459.66999999999996) * 5.0/9.0;
    return output_temp
}

fn convert_c_to_f(input_temp: &f64) -> f64 {
    let output_temp: f64 = (input_temp * 1.8) + 32.0;
    return output_temp
}

fn convert_c_to_k(input_temp: &f64) -> f64 {
    let output_temp: f64 = input_temp + 273.15;
    return output_temp
}

fn convert_k_to_c(input_temp: &f64) -> f64 {
    let output_temp: f64 = input_temp - 273.15;
    return output_temp
}

fn convert_k_to_f(input_temp: &f64) -> f64 {
    let output_temp: f64 = (input_temp * 1.8) - 459.66999999999996;
    return output_temp
}


fn set_input_value(input: &String) -> f64 {

    let mut value = input 
        .trim()
        .chars();

    value.next_back();
    
    let value = value.as_str();

    let value: f64 = value
        .trim()
        .parse()
        .expect("could not read the temperature value as a number");

    return value
}


fn set_input_unit(input: &String) -> char {

    let unit: char = input
        .trim()
        .chars()
        .last()
        .unwrap();

    return unit
}


fn main() {

    let mut input: String = String::new();

    println!("Enter a temperature. Use a unit:- F or C or K");

    io::stdin()
        .read_line(&mut input)
        .expect("could not read line");

    let input_temp: f64 = set_input_value(&input);

    let input_unit: char = set_input_unit(&input);

    // let valid_units: [char;3] = ['F', 'C', 'K'];

    // if ! valid_units.contains(&input_unit) {
    //     println!("did not recognise that unit. you entered '{input_unit}' Please enter one of F or C or K.");
    //     continue;
    // }

    // if input_unit = 'K' and input_temp < 0 {
    //     println!("Kelvin temperatures cannot be below 0. you entered '{input_temp}")
    // }
    
    let mut output_temp_a: f64 = 0.0;
    let mut output_unit_a: char = '\0';
    let mut output_temp_b: f64 = 0.0;
    let mut output_unit_b: char = '\0';

    match input_unit {
        'F' => {
            output_temp_a = convert_f_to_c(&input_temp); 
            output_unit_a = 'C';
            output_temp_b = convert_f_to_k(&input_temp);
            output_unit_b = 'K';
        },
        'C' => {
            output_temp_a = convert_c_to_f(&input_temp); 
            output_unit_a = 'F';
            output_temp_b = convert_c_to_k(&input_temp);
            output_unit_b = 'K';
        },
        'K' => {
            output_temp_a = convert_k_to_c(&input_temp); 
            output_unit_a = 'C';
            output_temp_b = convert_k_to_f(&input_temp);
            output_unit_b = 'F';
        },
        _ => ()

    };

    println!("{input_temp}°{input_unit} = \n{output_temp_a}°{output_unit_a} \n{output_temp_b}°{output_unit_b}");
}
