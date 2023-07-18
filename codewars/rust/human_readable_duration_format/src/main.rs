// https://www.codewars.com/kata/52742f58faf5485cae000b9a/train/rust

#[derive(Debug)]
struct Time {
    year: Option<u64>,
    day: Option<u64>,
    hour: Option<u64>,
    minute: Option<u64>,
    seconds: u64,
}

#[derive(Debug)]
struct TimeBuilder {
    year: Option<u64>,
    day: Option<u64>,
    hour: Option<u64>,
    minute: Option<u64>,
    seconds: u64,
}

impl Time {
    fn new(seconds: &u64) -> TimeBuilder {
        TimeBuilder {
            year: None,
            day: None,
            hour: None,
            minute: None,
            seconds: *seconds,
        }
    }
}

impl TimeBuilder {
    fn year(&mut self, year: u64) -> &mut Self {
        self.year = Some(year);
        self
    }

    fn day(&mut self, day: u64) -> &mut Self {
        self.day = Some(day);
        self
    }

    fn hour(&mut self, hour: u64) -> &mut Self {
        self.hour = Some(hour);
        self
    }

    fn minute(&mut self, minute: u64) -> &mut Self {
        self.minute = Some(minute);
        self
    }

    fn build(&mut self) -> Time {
        Time {
            year: self.year,
            day: self.day,
            hour: self.hour,
            minute: self.minute,
            seconds: self.seconds,
        }
    }
}

fn format(duration: Vec<String>) -> String {
    match duration.len() {
        1 => {
            return (0..duration.len())
                .map(|i| duration.get(i).unwrap().to_owned())
                .collect::<Vec<String>>()
                .join("")
        }
        2 => {
            return (0..duration.len())
                .map(|i| duration.get(i).unwrap().to_owned())
                .collect::<Vec<String>>()
                .join(" and ")
        }
        _ => {
            let mut temp = (0..duration.len() - 1)
                .map(|i| duration.get(i).unwrap().to_owned())
                .collect::<Vec<String>>()
                .join(", ");
            temp.push_str(" and ");
            temp.push_str(duration.get(duration.len() - 1).unwrap());

            temp
        }
    }
}

fn convert(time: Time) -> String {
    println!("Time:\t{:?}", time);

    let mut duration: Vec<String> = Vec::new();

    if time.year > Some(1) {
        duration.push(format!("{:?} years", time.year.unwrap()));
    } else if time.year == Some(1) {
        duration.push(format!("{:?} year", time.year.unwrap()));
    }

    if time.day > Some(1) {
        duration.push(format!("{:?} days", time.day.unwrap()));
    } else if time.day == Some(1) {
        duration.push(format!("{:?} day", time.day.unwrap()));
    }

    if time.hour > Some(1) {
        duration.push(format!("{:?} hours", time.hour.unwrap()));
    } else if time.hour == Some(1) {
        duration.push(format!("{:?} hour", time.hour.unwrap()));
    }

    if time.minute > Some(1) {
        duration.push(format!("{:?} minutes", time.minute.unwrap()));
    } else if time.minute == Some(1) {
        duration.push(format!("{:?} minute", time.minute.unwrap()));
    }

    if time.seconds > 1 {
        duration.push(format!("{:?} seconds", time.seconds));
    } else if time.seconds == 1 {
        duration.push(format!("{:?} second", time.seconds));
    }

    return format(duration);
}

fn format_duration(seconds: u64) -> String {
    if seconds <= 3600 {
        match seconds {
            0 => return String::from("now"),
            1 => return String::from("1 second"),
            _ => {
                return convert(
                    Time::new(&(seconds % 60))
                        .hour(&seconds / 3600)
                        .minute((&seconds % 3600) / 60)
                        .build(),
                )
            }
        }
    } else if seconds <= 86400 {
        return convert(
            Time::new(&(seconds % 60))
                .day(&seconds % 31536000 / 86400)
                .hour(&seconds / 3600)
                .minute((&seconds % 3600) / 60)
                .build(),
        );
    } else {
        return convert(
            Time::new(&(seconds % 60))
                .year(&seconds / 31536000)
                .day((seconds % 31536000) / 86400)
                .hour((&seconds % 86400) / 3600)
                .minute((&seconds % 3600) / 60)
                .build(),
        );
    }
}

fn main() {
    assert_eq!(format_duration(1), "1 second");
    assert_eq!(format_duration(62), "1 minute and 2 seconds");
    assert_eq!(format_duration(120), "2 minutes");
    assert_eq!(format_duration(3600), "1 hour");
    assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
    assert_eq!(
        format_duration(2286833),
        "26 days, 11 hours, 13 minutes and 53 seconds"
    );
}
