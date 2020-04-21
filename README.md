# Temperature Convertor
A rust library crate that will convert any temperature value given by the userfrom Celsius to Fahrenheit scale or from Fahrenheit
to Celsius scale.
## Usage
To use this library in your code, you just need to add following line in the dependencies section of your `cargo.toml` file.
```
[dependencies]
temp_convertor = "0.1.0"
```
Your `cargo.toml` file will look like this:
```
[package]
name = "temp_convertor"
version = "0.1.0"
authors = ["Haseeb ul Hassan <haseeb.ee12@gmail.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
temp_convertor = "0.1.0"
```
Now, just come in `src/main.rs` to use this library crate.
Write the following lines to get the conversion of temperature from Celsius to Fahrenheit. For example,the temperature entered
by the user is 96.8°.
```
use temp_convertor;
fn main () {
   temp_convertor::convertor::celsius_to_fahrenheit(96.8);  
}
```
Another way of using this crate:
```
use temp_convertor::convertor;
fn main () {
   convertor::fahrenheit_to_celsius(96.8);  
}
```
Now in the end, just use `cargo run` to get the desired conversion of temperature. 
### Syntax
Here, the `temp_convertor` is the name of crate, `convertor`is the module and `celsius_to_fahrenheit(96.8)` and `fahrenheit_to_celsius(96.8)`
are the functions that are converting the temperature from Celsius to Fahrenheit and from Fahrenheit to Celsius as well. 
Both are taking 96.8 as an argument.
##### Note: 
You can pass only floating point numbers as argument in the these function `celsius_to_fahrenheit(96.8)` and `fahrenheit_to_celsius(96.8)`.
### Output:
The output will be like this:
```
You entered: 96.8°
The temperature in Fahrenheit is: 206.24°
```

```
You entered: 96.8°
The temperature in Celsius is: 36.00°
```

