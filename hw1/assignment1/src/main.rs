const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}


fn main() {
    let temperature_f: f64 = 32.0;
    
    println!("{}°F is {:.2}°C", temperature_f, fahrenheit_to_celsius(temperature_f));
    
    for i in 1..=5 {
        let next_temp_f = temperature_f + i as f64;
        println!("{}°F is {:.2}°C", next_temp_f, fahrenheit_to_celsius(next_temp_f));
    }
}