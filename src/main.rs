extern crate reqwest;

extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::env;

const KELVIN_TO_CELSIUS: f64 = -273.15;

fn main() {
    let mut args = env::args().skip(1);
    let key = args.next().expect("Key must be the first arg");
    let city = args.next().expect("City must be the second arg");
//    println!("Key: {}\nCity: {}", key, city);
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, key);
    let response: Response = reqwest::get(&url).expect("Request failed")
        .json().expect("Request body is not text");
    println!("{}", response.create_report());
}

#[derive(Deserialize)]
struct Response {
    main: ResponseMain,
    weather: Vec<Weather>,
}

impl Response {
    pub fn create_report(&self) -> String {
        let mut report = format!("{:.01}°C ", self.main.get_celsius());
        self.weather.first()
            .expect("No weather field")
            .main.push_icon(&mut report);
        report
    }
}

#[derive(Deserialize)]
struct ResponseMain {
    temp: f64,
}

impl ResponseMain {
    fn get_celsius(&self) -> f64 {
        self.temp + KELVIN_TO_CELSIUS
    }
}

#[derive(Deserialize)]
struct Weather {
    pub main: WeatherMain,
}

#[derive(Deserialize)]
enum WeatherMain {
    Thunderstorm,
    Drizzle,
    Rain,
    Snow,
    Mist,
    Clear,
    Clouds,
    Extreme,
    Additional,
}

impl WeatherMain {
    pub fn push_icon(&self, string: &mut String) {
        let icon = match *self {
            WeatherMain::Thunderstorm => '⚡',
            WeatherMain::Drizzle
            | WeatherMain::Rain => '💧',
            WeatherMain::Snow => '❄',
            WeatherMain::Mist => '🌫',
            WeatherMain::Clear => '☀',
            WeatherMain::Clouds => '☁',
            _ => '⭐',
        };
        string.push(icon);
    }
}

//use std::env;
//
//fn main() {
//    let args = env::args().skip(1);
//    let key = args.next().expect("Key must be the first arg");
//    let city = args.next().expect("City must be the second arg");
//    println!("Key: {}\nCity: {}", key, city);
//}

//use std::env;
//
//fn main() {
//    for arg in env::args() {
//        println!("{}", arg)
//    }
//}

//fn main() {
//    println!("Hello, world!");
//}
