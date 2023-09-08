pub fn celsius_to_fahrenheit(temp: f64) -> f64 {
    temp * 1.8 + 32.0
}

pub fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) / 1.8
}

pub fn test_temperature(temp: f64) {
    let room_temp_celsius = temp;
    let room_temp_fahrenheit = celsius_to_fahrenheit(room_temp_celsius);

    println!("Celsius Temp = {}", room_temp_celsius);
    println!("Fahrenheit Temp = {}", room_temp_fahrenheit);

    println!("The reverse:");
    let room_temp_fahrenheit = temp;
    let room_temp_celsius = fahrenheit_to_celsius(room_temp_fahrenheit);
    println!("Celsius Temp = {}", room_temp_celsius);
    println!("Fahrenheit Temp = {}", room_temp_fahrenheit);

    if (room_temp_fahrenheit - room_temp_celsius).abs() < f64::EPSILON {
        println!("This is some wizardry, they're exactly the same!!");
    }
}