use chrono::{Date, Utc, Datelike, Duration, TimeZone, Weekday};
use getopts::Options;
use std::env;
use std::fmt;

// Define the enum for the liturgical seasons
#[derive(PartialEq, Copy, Clone)]
enum LiturgicalSeason {
    Advent,
    Christmastide,
    Epiphany,
    Lent,
    Easter,
    Pentecost,
    OrdinaryTime,
}

// Define the struct for each liturgical day
struct LiturgicalDay {
    date: Date<Utc>,
    first_reading: String,
    second_reading: String,
    psalm: String,
    collect: String,
    season: LiturgicalSeason,
    sunday_of_season: i32,
}

impl fmt::Display for LiturgicalSeason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LiturgicalSeason::Advent => write!(f, "Advent"),
            LiturgicalSeason::Christmastide => write!(f, "Christmastide"),
            LiturgicalSeason::Epiphany => write!(f, "Epiphany"),
            LiturgicalSeason::Lent => write!(f, "Lent"),
            LiturgicalSeason::Easter => write!(f, "Easter"),
            LiturgicalSeason::Pentecost => write!(f, "Pentecost"),
            LiturgicalSeason::OrdinaryTime => write!(f, "Ordinary Time"),
        }
    }
}

// Function to calculate the date of Easter 
// using the anonymous Gregorian algorithm 
// https://en.wikipedia.org/wiki/Date_of_Easter#Anonymous_Gregorian_algorithm
fn calculate_easter(year: i32) -> Date<Utc> {
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = (h + l - 7 * m + 114) / 31;
    let day = ((h + l - 7 * m + 114) % 31) + 1;
    Utc.ymd(year, month as u32, day as u32)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("y", "year", "Set the year", "YEAR");

    let matches = opts.parse(&args[1..]).unwrap_or_else(|f| panic!("{}", f.to_string()));

    let year: i32 = matches.opt_str("year").unwrap_or_else(|| {
        println!("Please provide a year with --year option.");
        std::process::exit(1);
    }).parse().unwrap_or_else(|_| {
        println!("Invalid year provided.");
        std::process::exit(1);
    });

    let easter = calculate_easter(year);
    let ash_wednesday = easter - Duration::days(46);
    let pentecost = easter + Duration::days(50);
    let christmas = Utc.ymd(year, 12, 25);
    let advent_start = christmas - Duration::days(((christmas.weekday().num_days_from_sunday() + 22) % 28) as i64);

    let mut days = Vec::new();
    let mut current_season = LiturgicalSeason::Christmastide;
    let mut sunday_of_season = 2;

    for day in 1..=365 {
        let current_date = Utc.ymd(year, 1, 1).and_hms(0, 0, 0) + chrono::Duration::days((day - 1) as i64);
        let mut season = LiturgicalSeason::Christmastide;

        let first_reading = String::new();
        let second_reading = String::new();
        let psalm = String::new();
        let collect = String::new();

        // Check for special seasons
        if current_date.date_naive() >= Utc.ymd(year, 12, 1).naive_utc() && current_date.date_naive() < christmas.naive_utc() {
            season = LiturgicalSeason::Advent;
        } else if current_date.date().month() == 12 && current_date.date().day() >= 25
        || current_date.date().month() == 1 && current_date.date().day() <= 6 {
            season = LiturgicalSeason::Christmastide;
        } else if (current_date.date().month() == 1 && current_date.date().day() >= 7) || (current_date.date() < ash_wednesday) {
            season = LiturgicalSeason::Epiphany;
        } else if current_date.date_naive() >= ash_wednesday.naive_utc() && current_date.date_naive() < easter.naive_utc() {
            season = LiturgicalSeason::Lent;
        } else if current_date.date_naive() == easter.naive_utc() {
            season = LiturgicalSeason::Easter;
        } else if current_date.date_naive() > easter.naive_utc() && current_date.date_naive() < pentecost.naive_utc() {
            season = LiturgicalSeason::Easter;
        } else if current_date.date_naive() >= pentecost.naive_utc() && current_date.date_naive() <= (pentecost + Duration::days(6)).naive_utc() {
            season = LiturgicalSeason::Pentecost;
        } else if current_date.date_naive() >= (pentecost + Duration::days(7)).naive_utc() {
            season = LiturgicalSeason::OrdinaryTime;
        }

         // Check if the season has changed
         if season != current_season {
            // Reset the Sunday of the season
            sunday_of_season = 1;
            current_season = season;
        } else if current_date.weekday() == Weekday::Sun {
            // Increment the number of the Sunday in the season
            sunday_of_season += 1;
        }

        // Example logic to set readings and collects based on the liturgical calendar
        // This is where you'd implement your specific logic based on the 1928 Book of Common Prayer
        // For now, it's just placeholders
        let mut liturgical_day = LiturgicalDay {
            date: current_date.date(),
            season: season,
            sunday_of_season: sunday_of_season,
            first_reading,
            second_reading,
            psalm,
            collect,
        };

        // Example: Set special readings for Easter
        if current_date.date_naive() == easter.naive_utc() {
            liturgical_day.season = LiturgicalSeason::Easter;
            liturgical_day.first_reading = String::from("Easter First Reading");
            liturgical_day.second_reading = String::from("Easter Second Reading");
            liturgical_day.psalm = String::from("Easter Psalm");
            liturgical_day.collect = String::from("Easter Collect");
        }

        // Add the liturgical day to the vector
        days.push(liturgical_day);
    }

    // Print out the liturgical days (or process them as needed)
    for day in days {
        println!("Date: {}, Season: {}, Sunday of Season: {}",
                 day.date, day.season, day.sunday_of_season);
    }
}
