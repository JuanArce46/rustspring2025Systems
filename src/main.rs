
struct Car {
    body: String,
    year: u16,
    color: String,
}

impl Car {
    fn new(b:String, y:u16,c:String) -> Car{
        Car{
            body: b,
            year: y,
            color: c,
        }
    }
    fn show_info(&self){
        println!("{} {} {} ", self.body, self.year, self.color);
    }

    fn change_color(&mut self, new_color:String){
        self.color = new_color;
    }
}

fn main() {
    let mut my_car = Car::new("Sedan".to_string(),2020,"Purple".to_string());
    // println!("{:?}",my_car);
    my_car.show_info();
    my_car.change_color("Black".to_string());
    my_car.show_info();
}
