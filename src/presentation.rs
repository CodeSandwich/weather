///////////
//// 0 ////
///////////

// http://openweathermap.org/
// http://api.openweathermap.org/data/2.5/weather?q=Wroclaw&appid=40212611efdf88f5b7b0a0a552e250b3

// Intellij - just editor

// Cargo - rustc wrapper, kombajn do kompilacji

// Initialise project

cargo init --bin

// open in Intellij
// Cargo.toml
// main.rs
// cargo run


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

// let creates variables

let args = args();

//

let key = args.next();

//

mut

//

println!("Key: {}", key);

// https://doc.rust-lang.org/std/option/enum.Option.html

.expect("Key must be the first arg")

//

.skip(1)






///////////
//// 3A ////
///////////

let city = args.next();

//

.unwrap_or("Wrocław".to_string())

//

println!("Key: {}\nCity: {}", key, city);






///////////
//// 4 ////
///////////

// Need HTTP request, not in std
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

let text = reqwest::get(&url);

// https://doc.rust-lang.org/std/result/enum.Result.html

.expect("Request failed")

// Look ma, no exceptions!


    .text().expect("Request has no body")

//

println!("RESPONSE:\n{}", text);




///////////
//// 5 ////
///////////

// https://docs.rs/reqwest/0.8.5/reqwest/struct.Response.html#method.json

// What is serde?

serde = "1.0.37"
serde_derive = "1.0.37"

//

extern crate serde;
#[macro_use]
extern crate serde_derive;

// http://api.openweathermap.org/data/2.5/weather?q=Wroclaw&appid=40212611efdf88f5b7b0a0a552e250b3
// Temperature first
// We recreate JSON in structures

struct Response {
    pub main: ResponseMain,
}

//

struct ResponseMain {
    pub temp: f64,
}

// Automatic generate deserialize implementation

#[derive(Deserialize)]

//

.json().expect("Request body is not json");

//

response: Response

//

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

// mutable ref

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

//

//impl Response {
//    pub fn create_report(&self) -> String {
        let mut report = format!("{:.01}°C ", self.main.get_celsius());
        self.weather.iter()
            .for_each(|w| w.main.push_icon(&mut report));
        report
    }
//}
