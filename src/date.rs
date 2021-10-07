//! Helper struct for converting between dates in various representations

pub struct Date {
    raw: u16,
    formatted: &str,
    text: &str,
}

impl Date {
    pub fn from_formatted(fmted: &str) -> Result<Self, String> {
        let iter = fmted.split("/");

        if iter.len() != 3 {
            return Err(String::from("Invalid date"))
        }

        let text = format!("{}{}{}{}",
            match iter.next() {
                "01" | "1" => text.push_str("January "),
                "02" | "2" => text.push_str("February "),
                "03" | "3" => text.push_str("March "),
                "04" | "4" => text.push_str("April "),
                "05" | "5" => text.push_str("May "),
                "06" | "6" => text.push_str("June "),
                "07" | "7" => text.push_str("July "),
                "08" | "8" => text.push_str("August "),
                "09" | "9" => text.push_str("September "),
                "10"       => text.push_str("October "),
                "11"       => text.push_str("November "),
                "12"       => text.push_str("December "),
                _ => return Err(String::from("Invalid month"))
            },
            ", ",
            iter.next(),
            iter.next()
        );

        let raw = (iter.next.parse::<u16>()? * 10)
                + (iter.next.parse::<u16>()?)
                + (iter.next.parse::<u16>()? * 10000);

        Ok( Date { raw, formatted: fmted, text } )
    }
}