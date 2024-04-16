//Que tu le pongas si es interior o exterior, día o noche y
//el nombre de la locación y te dé el encabezado ya armado

use core::fmt;

enum HeadingType {
    //ENUM del tipo
    Interior,
    Exterior,
}

enum HeadingDayTime {
    Day,
    Night,
}

struct Heading {
    heading_type: HeadingType,
    location: String,
    day_time: HeadingDayTime,
}

impl Heading {
    fn new(heading_type: HeadingType, location: String, day_time: HeadingDayTime) -> Heading {
        Heading {
            heading_type,
            location: location.to_uppercase(),
            day_time,
        }
    }
}

impl fmt::Display for Heading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let heading_type = match self.heading_type {
            HeadingType::Exterior => "EXT",
            HeadingType::Interior => "INT",
        };
        let day_time = match self.day_time {
            HeadingDayTime::Day => "DAY",
            HeadingDayTime::Night => "NIGHT",
        };

        let heading = format!("{}. {} - {}", heading_type, self.location, day_time);

        write!(f, "{}", heading)
    }
}

fn main() {
    let h1 = Heading::new(
        HeadingType::Exterior,
        String::from("Highway"),
        HeadingDayTime::Night,
    );
    let h2 = Heading::new(
        HeadingType::Interior,
        String::from("Roque's House"),
        HeadingDayTime::Day,
    );
    let h3 = Heading::new(
        HeadingType::Interior,
        String::from("Bathroom"),
        HeadingDayTime::Night,
    );

    println!("{}", h1);
    println!("{}", h2);
    println!("{}", h3);
}
