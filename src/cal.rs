use scraper::{ Html, Selector };
use transpose::transpose;
use colored::*;
use terminal_size::{Width, Height, terminal_size};

pub fn cal(username: &String, colour: bool, raw: bool) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://github.com/".to_owned() + &username;

    println!("{}", url);
    
    let res = reqwest::blocking::get(url)?;

    if !res.status().is_success() {
        eprintln!("That username can't be found!");
        return Ok(());
    }

    let document = Html::parse_document(&res.text()?);
    let g_selector = Selector::parse("g").unwrap();
    let rect_selector = Selector::parse("rect").unwrap();

    let mut weeks: [char; 7 * 53] = ['X'; 7 * 53];

    let mut day = 0;
    for element in document.select(&g_selector).skip(1) {
        for rect in element.select(&rect_selector) {
            weeks[day] = rect.value().attr("data-level").unwrap().chars().next().unwrap();
            day += 1;
        }
    }

    let mut output = vec!['X'; weeks.len()];
    transpose(&weeks, &mut output, 7, 53);

    let size = terminal_size();
    let character = if let Some((Width(w), Height(_h))) = size {
        if w >= 110 {
            "▇▇"
        } else {
            "■"
        }
    } else {
        "■"
    };
    
    for (day_index, day) in output.chunks(53).enumerate() {
        match day_index {
            1 => print!("Mon "),
            3 => print!("Wed "),
            5 => print!("Fri "),
            _ => print!("    "),
        }

        for day in day.iter() {
            if raw {
                print!("{}", &day)
            } else if colour && colored::control::ShouldColorize::from_env().should_colorize() {
                print!("{}", match day {
                    '0' => character.truecolor(22, 27, 34),
                    '1' => character.truecolor(14, 68, 41),
                    '2' => character.truecolor(0, 109, 50),
                    '3' => character.truecolor(38, 166, 65),
                    '4' => character.truecolor(57, 211, 83),
                    _ => " ".normal()
                })
            } else {
                print!("{}", match day {
                    '0' => " ",
                    '1' => ".",
                    '2' => "+",
                    '3' => "#",
                    '4' => "%",
                    _ => " "
                })
            }
        }
        print!("\n")
    }

    Ok(())
}
