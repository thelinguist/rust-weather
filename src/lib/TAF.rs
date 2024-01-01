use crate::constants::TAF_URL;
use crate::lib::Condition::Condition;
use reqwest;

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
    let initial_condition = Condition::parse_condition(taf_initial[2..].join(" ").to_string());
    conditions.push(initial_condition);

    // rest of TAF
    for line in &lines[2..] {
        if line.len() == 0 {
            break;
        }
        let condition = Condition::parse_condition(line.to_string());
        conditions.push(condition);
    }

    let taf_obj = TAF {
        airport,
        reporting_time,
        conditions,
    };
    return taf_obj;
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
