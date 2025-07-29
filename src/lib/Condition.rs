
pub struct Condition {
    pub time: String,
    pub wind: Wind,
    pub visibility: String,
    pub weather: String,
    pub sky: String,
}

impl Condition {
    pub fn parse_condition(condition: String) -> Condition {
        let tokens: Vec<&str> = condition.split(" ").into_iter().filter(|&x| x != "").collect();
        let mut weather: String = String::new();
        let mut sky_index: usize = 3;
        if tokens.len() > 4 {
            weather = tokens[3].to_string();
            sky_index = 4;
        }
        Condition {
            time: tokens[0].to_string(),
            wind: Wind::parse_wind(tokens[1]),
            visibility: tokens[2].to_string(),
            weather: weather,
            sky: tokens[sky_index].to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut condition_str = format!("Time: {}", self.time);
        condition_str = format!("{}\n\tWind: {}", condition_str, self.wind.to_string());
        condition_str = format!("{}\n\tVisibility: {}", condition_str, self.visibility);
        condition_str = format!("{}\n\tWeather: {}", condition_str, self.weather);
        condition_str = format!("{}\n\tSky: {}", condition_str, self.sky);
        condition_str
    }
}

pub struct Wind {
    pub direction: u16,
    pub speed: u16,
}

impl Wind {
    pub fn parse_wind(wind: &str) -> Wind {
        let direction = wind[..3].parse::<u16>().unwrap();
        let speed = wind[3..5].parse::<u16>().unwrap();
        Wind {
            direction,
            speed,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} at {} KT", self.direction, self.speed)
    }
}