///////////
//// 0 ////
///////////

// http://openweathermap.org/
// http://api.openweathermap.org/data/2.5/weather?q=Wroclaw&appid=40212611efdf88f5b7b0a0a552e250b3

// Cargo - rustc wrapper, kombajn do kompilacji

// Initialise project

cargo init --bin




///////////
//// 1 ////
///////////

fn main() {
    println!("Hello world!");
}




///////////
//// 2 ////
///////////

// Include std lib in namespace

use std::env::args;

// args() returns iterator
// iterators can be used in for

for arg in args() {
    println!("{}", arg)
}

//use std::env;
//
//fn main() {
//    for arg in args() {
//        println!("{}", arg)
//    }
//}




///////////
//// 3 ////
///////////

// let creates variable

let args = args().skip(1);

// Iterator returns option
// https://doc.rust-lang.org/std/option/enum.Option.html

let key = args.next().expect("Key must be the first arg");

//

let city = args.next().unwrap_or("Wrocław".to_string());

// Because macro, can get N args

println!("Key: {}\nCity: {}", key, city);

// next needs mut, must be mut explicitly

mut

//use std::env;
//
//fn main() {
//    let mut args = args().skip(1);
//    let key = args.next().expect("Key must be the first arg");
//    let city = args.next().unwrap_or("Wrocław".to_string());
//    println!("Key: {}\nCity: {}", key, city);
//}




///////////
//// 4 ////
///////////

// Need HTTP request, not in std
// Cargo.toml steruje Cargo
// https://crates.io/
// http request client BY DOWNLOAD RECENT
// Paste in Cargo.toml
// Compile, downloads!

reqwest = "0.8.5"

// Take to namespace

extern crate reqwest;

//

let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, key);

//

reqwest::get(&url).expect("Request failed");

// Look ma, no exceptions!

let text = reqwest::get(&url).expect("Request failed")
    .text().expect("Request body is not text");

//

println!("RESPONSE:\n{}", text);

//extern crate reqwest;
//
//use std::env::args;
//
//fn main() {
//    let mut args = args().skip(1);
//    let key = args.next().expect("Key must be the first arg");
//    let city = args.next().unwrap_or("Wrocław".to_string());
//    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, key);
//    let text = reqwest::get(&url).expect("Request failed")
//        .text().expect("Request body is not text");
//    println!("RESPONSE:\n{}", text);
//}




///////////
//// 5 ////
///////////

// We recreate JSON in structures

struct Response {
    pub main: ResponseMain,
}

struct ResponseMain {
    pub temp: f64,
}




///////////
//// 6 ////
///////////

// https://docs.rs/reqwest/0.8.5/reqwest/struct.Response.html#method.json
// Response can deserialize from JSON to Serde Deserialize
// What is Serde?
// We will implement Deserialize for Response our structs (with macro!)

serde = "1.0.37"
serde_derive = "1.0.37"

//

extern crate serde;
#[macro_use]
extern crate serde_derive;

// Automatic generate deserialize implementation

#[derive(Deserialize)]

//

let response: Response = reqwest::get(&url).expect("Request failed")
    .json().expect("Request body is not json");
println!("TEMP: {}", response.main.temp);





///////////
//// 6 ////
///////////

// Kelvin, really?

impl ResponseMain {
    fn get_celsius(&self) -> f64 {
        self.temp - 273.15
    }
}

//

pub

//

println!("TEMP: {}", response.main.get_celsius());





///////////
//// 7 ////
///////////

impl Response {
    fn create_report(&self) -> String {
        format!("{:.01}°C ", self.main.get_celsius())
    }
}

//

pub

//

println!("{}: {}", city, response.create_report());




// END?


///////////
//// 8 ////
///////////

weather: Vec<Weather>,

//

#[derive(Deserialize)]
struct Weather {
    pub main: WeatherMain,
}

//

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

//

impl Response {
    pub fn create_report(&self) -> String {
        let mut report = format!("{:.01}°C ", self.main.get_celsius());
        self.weather.iter()
            .for_each(|w| w.main.push_icon(&mut report));
        report
    }
}

//

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
