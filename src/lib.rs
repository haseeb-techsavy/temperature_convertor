pub mod convertor {
    pub fn celsius_to_fahrenheit(temp: f64) {
        println!("You entered: {}°", temp);
        let temp = (temp * 1.8) + 32.0;
        println!("The temperature in Fahrenheit is: {:.2}°",temp);
}
    pub fn fahrenheit_to_celsius(temp: f64) {
        println!("You entered: {}°", temp);
        let temp = (temp - 32.0) / 1.8 ;
        println!("The temperature in Celsius is: {:.2}°",temp);
    }
}
