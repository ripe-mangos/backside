extern crate proc_macro;
extern crate backside;

use backside::*;

// enum Section {
//     Info,
// }

// impl Section {
//     fn header(&self) -> &str {
//         match self {
//             Section::Info => "Script Info",
//         }
//     }
// }

// TODO: use a range instead of "*"
//const STYLE_RE: &str = r"([^:]*): (.*)";

// enum Section {
//     Info,
//     Styles,
//     Fonts,
//     Events,
// }
// impl std::fmt::Display for Section {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}",
//             match self {
//                 Section::Info   => "Info",
//                 Section::Styles => "Styles",
//                 Section::Fonts  => "Fonts",
//                 Section::Events => "Events",
//             }
//         )
//     }
// }

// struct Subtitles {
//     pub styles: Vec<Style>,
// }

fn main() {
    let input = std::fs::read_to_string("test.ass").unwrap();

    //loop {
        print!("{}", backside_parser::parse_sections(input.as_str()));
    //    std::thread::sleep_ms(100);
    //}
    //println!("{}", String::from_utf8(uuenc::uuencode(Bytes::from("This is a test.")).unwrap()).unwrap());
    // let lines: Vec<&str> = input.lines().collect();
    // let format = style::parse_format(lines.clone());
    //let styles = style::parse_style(lines.clone(), format);

//     for style in styles {
//         println!("
// {}
// '- Font: {} @ {} px", style.name, style.font_name, style.font_size)
//     }

    // r"\[(.*)\]"
    // let re = Regex::new(STYLE_RE).unwrap();

    // let caps = re.captures_iter(input.as_str());
    // for cap in caps {
    //     let line_type = cap.get(1).unwrap().as_str();
    //     let fields = cap.get(2).unwrap().as_str();

    //     if line_type != "Dialogue" {
    //         println!("{}", line_type);
    //         for field in fields.split(",") {
    //             println!("'-> {}", field);
    //         }
    //     }
    // }
   // Ok(())
}
