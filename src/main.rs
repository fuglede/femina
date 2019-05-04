#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use reqwest;
use regex::Regex;

fn main() {
    rocket::ignite().mount("/", routes![horoscope_handler]).launch();
}

#[get("/<zodiac>")]
fn horoscope_handler(zodiac: String) -> String {
    match &*zodiac {
        "krebs" | "vaedder" | "fisk" | "loeve" | "tvilling" | "tyr" | "jomfru"
            | "vaegt" | "skorpion" | "vandmand" | "stenbuk" | "skytte" => horoscope(zodiac),
        _ => format!("{} is not a valid zodiac", zodiac)
    }
}

fn horoscope(zodiac: String) -> String {
    let url = format!("https://www.femina.dk/horoskoper/{}/dagshoroskop", zodiac);
    let resp = reqwest::get(&url).unwrap().text().unwrap();
    let re = Regex::new(".+?<span style=\"color: rgb\\(51, 51, 51\\);\">([^<]+)</span>.+").unwrap();
    re.captures(&resp).unwrap()[1].to_string()
}