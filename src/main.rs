extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::env::args;

fn main() {
    let mut args = args().skip(1);
    let key = args.next().expect("Key must be the first arg");
    let city = args.next().unwrap_or("Wrocław".to_string());
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, key);
    let response: Response = reqwest::get(&url).expect("Request failed")
        .json().expect("Request body is not json");
    println!("{}: {}", city, response.create_report());
}

#[derive(Deserialize)]
struct Response {
    main: ResponseMain,
    weather: Vec<Weather>,
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
    Haze,
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
            WeatherMain::Haze
            | WeatherMain::Mist => '🌫',
            WeatherMain::Clear => '☀',
            WeatherMain::Clouds => '☁',
            _ => '⭐',
        };
        string.push(icon);
    }
}

#[derive(Deserialize)]
struct ResponseMain {
    temp: f64,
}

impl ResponseMain {
    fn get_celsius(&self) -> f64 {
        self.temp - 273.15
    }
}

impl Response {
    fn create_report(&self) -> String {
        let mut report = format!("{:.01}°C ", self.main.get_celsius());
        self.weather.iter()
            .for_each(|w| w.main.push_icon(&mut report));
        report
    }
}
