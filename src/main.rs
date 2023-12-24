use std::io; 

fn fahrenheit_to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) * (5.0/9.0)
}

fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    (temperature * (9.0/5.0)) + 32.0
}    


fn main() {
    println!("Choose what kind of conversion you want to do");
    println!("Fahrenheit to Celsius [1]");
    println!("Celsius to Fahrenheit [2]");

    let mut option: u32 = 0;
    let mut temperature: f64 = 0.0;

    let mut input_options = String::new();

    io::stdin()
        .read_line(&mut input_options)
        .expect("Failed to read line");
    
    let parsed_option: Result<u32, _> = input_options.trim().parse();
    match parsed_option {
        Ok(number) => {
            option = number;
            println!("You entered: {}", option);
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
    
    println!("Introduce the temperature");
    
    let mut input_temperature = String::new();

    io::stdin()
        .read_line(&mut input_temperature)
        .expect("Failed to read line");

    let parsed_temperature: Result<f64, _> = input_temperature.trim().parse();
    match parsed_temperature {
        Ok(number) => {
            temperature = number;

            println!("You entered: {}", temperature);
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }

    if option == 1 {
       let result = fahrenheit_to_celsius(temperature);
       println!("The converted temperature is: {result}");
    } else if option == 2 {
       let result = celsius_to_fahrenheit(temperature);
       println!("The converted temperature is: {result}");
    } else {
        println!("Incorrect option");
    }
}


