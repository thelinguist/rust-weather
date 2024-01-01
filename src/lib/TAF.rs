use reqwest;
use crate::constants::TAF_URL;

/**
 * TAF.rs
 *
 * This file contains the functions to fetch the TAF data from the NOAA website.
 *
**/
// this function takes a string and returns a string
pub fn fetch_taf(airport_code: &str) -> String {
    let url = format!("{}{}&taf=true", TAF_URL, airport_code);
    println!("Fetching TAF from {}", url);
    let resp = reqwest::blocking::get(url).unwrap();
    let body = resp.text().unwrap();
    body
}

pub fn parse_taf(taf: String) -> TAF {
    let lines: Vec<&str> = taf.split("\n").collect();
    // METAR on line 0, then blank line, then TAF
    if lines.len() < 3 {
        panic!("No TAF available");
    }

    // get the all lines after the first 2
    let taf_lines = &lines[1..];
    let taf_initial: Vec<&str> = taf_lines[0].split(" ").collect();
    let mut conditions: Vec<Condition> = Vec::new();

    // first line of TAF
    let airport = taf_initial[0].to_string();
    let reporting_time = taf_initial[1].to_string();
    let initial_condition = Condition {
        time: taf_initial[2].to_string(),
        wind: taf_initial[3].to_string(),
        visibility: taf_initial[4].to_string(),
        weather: taf_initial[5].to_string(),
        sky: taf_initial[6].to_string(),
    };
    conditions.push(initial_condition);

    // rest of TAF
    for line in &lines[2..] {
        if line.len() == 0 {
            break;
        }
        let tokens: Vec<&str> = line.split(" ").into_iter().filter(|&x| x != "").collect();
        let condition = Condition {
            time: tokens[0].to_string(),
            wind: tokens[1].to_string(),
            visibility: tokens[2].to_string(),
            weather: tokens[3].to_string(),
            sky: tokens[4].to_string(),
        };
        conditions.push(condition);
    }

    let taf_obj = TAF {
        airport,
        reporting_time: reporting_time,
        conditions,
    };
    return taf_obj
}


pub struct TAF {
    pub airport: String,
    pub reporting_time: String,
    pub conditions: Vec<Condition>,
}

impl TAF {
    pub fn to_string(&self) -> String {
        let mut taf_str = format!("TAF for {}", self.airport);
        taf_str = format!("{}\nReporting time: {}", taf_str, self.reporting_time);
        taf_str = format!("{}\nConditions:", taf_str);
        taf_str = format!("{}\n{}", taf_str, self.conditions[0].to_string());
        for condition in &self.conditions[1..] {
            taf_str = format!("{}\n{}", taf_str, condition.to_string());
        }
        taf_str
    }

}

pub struct Condition {
    pub time: String,
    pub wind: String,
    pub visibility: String,
    pub weather: String,
    pub sky: String,
}

impl Condition {
    pub fn to_string(&self) -> String {
        let mut condition_str = format!("Time: {}", self.time);
        condition_str = format!("{}\nWind: {}", condition_str, self.wind);
        condition_str = format!("{}\nVisibility: {}", condition_str, self.visibility);
        condition_str = format!("{}\nWeather: {}", condition_str, self.weather);
        condition_str = format!("{}\nSky: {}", condition_str, self.sky);
        condition_str
    }
}