use crate::constants::TAF_URL;
use crate::lib::condition::Condition;
use reqwest;

/**
 * TAF.rs
 *
 * This file contains the functions to fetch the TAF data from the NOAA website.
 *
**/
pub fn fetch_taf(airport_code: &str) -> String {
    let url = format!("{}{}&taf=true", TAF_URL, airport_code);
    println!("Fetching TAF from {}", url);
    let resp = reqwest::blocking::get(url).unwrap();
    let body = resp.text().unwrap();
    body
}

pub fn parse_taf(taf: String) -> TAF {
    let lines: Vec<&str> = taf.split("\n").collect();
    // METAR on line 0, then TAF
    if lines.len() < 3 {
        panic!("No TAF available");
    }

    // get the all lines after the first
    let taf_lines = &lines[1..];
    let taf_initial: Vec<&str> = taf_lines[0].trim().split(" ").collect();
    let mut conditions: Vec<Condition> = Vec::new();

    // first line of TAF
    let airport = taf_initial[0].to_string();
    let reporting_time = taf_initial[1].to_string();
    let initial_winds_vis_clouds = &taf_initial[taf_initial.len() - 3..].join(" ").to_string().trim().to_string();
    let initial_condition_string = format!("{} {}", reporting_time, initial_winds_vis_clouds);
    let initial_condition = Condition::parse_condition(initial_condition_string);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_taf_with_forecasts() {
        let taf_data = "KPVU 290156Z 00000KT 10SM CLR 33/03 A3008 RMK AO2 SLP123 T03330028 $
KPVU 282325Z 2900/2924 27006KT P6SM FEW200
  FM290400 15006KT P6SM SCT200
  FM292100 25007KT P6SM FEW200
";
        let taf = parse_taf(taf_data.to_string());

        assert_eq!(taf.airport, "KPVU");
        assert_eq!(taf.reporting_time, "282325Z");
        assert!(!taf.conditions.is_empty());
        assert_eq!(taf.conditions[0].time, "282325Z");
        assert_eq!(taf.conditions[0].wind.direction, 270);
        assert_eq!(taf.conditions[0].wind.speed, 6);
        assert_eq!(taf.conditions[0].sky, "FEW200");
        assert_eq!(taf.conditions[0].visibility, "P6SM");

        assert_eq!(taf.conditions[1].time, "FM290400");
        assert_eq!(taf.conditions[1].wind.direction, 150);
        assert_eq!(taf.conditions[1].wind.speed, 6);
        assert_eq!(taf.conditions[1].sky, "SCT200");
        assert_eq!(taf.conditions[1].visibility, "P6SM");

        assert_eq!(taf.conditions[2].time, "FM292100");
        assert_eq!(taf.conditions[2].wind.direction, 250);
        assert_eq!(taf.conditions[2].wind.speed, 7);
        assert_eq!(taf.conditions[2].sky, "FEW200");
        assert_eq!(taf.conditions[2].visibility, "P6SM");
    }
}
