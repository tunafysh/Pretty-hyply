use hyperpolyglot::{get_language_breakdown, Language};
use colored::Colorize;
use hex_rgb::{convert_hexcode_to_rgb, Color};

fn main() {
    let breakdown = get_language_breakdown("./");
    let mut total_files = 0;
    for (_language, detections) in &breakdown {
        total_files += detections.len();
    }

    let percentage = breakdown.iter().map(|(language, detections)| {
        format!("{}: {}%", language, ((detections.len() as f64 / total_files as f64) * 100.0).round().to_string())
    }).collect::<Vec<_>>().join(", ");
        for lang in percentage.split(", ") {
            let language_struct = Language::try_from(lang.split(":").next().unwrap()).unwrap();
            let hex_color = language_struct.color;
            match hex_color {
                Some(hex_color) => {
                    let color: Color = convert_hexcode_to_rgb(hex_color.to_string()).unwrap();
                    print!("{}\n", format!("{}", lang.to_string()).to_string().truecolor(color.red, color.green, color.blue).bold());
                }
                None => {
                    print!("{}\n", format!("{}", lang.to_string()).to_string().truecolor(255, 255, 255));
                }
            }

        }
  

}