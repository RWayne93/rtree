#[allow(dead_code)]
pub enum Color {
    Blue,
    Yellow,
    Green,
    Red,
    Cyan,
    Magenta,
    White,
    Reset,
}

impl Color {
    pub fn paint(&self, text: &str) -> String {
        match self {
            Color::Blue => format!("\x1B[34m{}\x1B[0m", text),
            Color::Yellow => format!("\x1B[33m{}\x1B[0m", text),
            Color::Green => format!("\x1B[32m{}\x1B[0m", text),
            Color::Red => format!("\x1B[31m{}\x1B[0m", text),
            Color::Cyan => format!("\x1B[36m{}\x1B[0m", text),
            Color::Magenta => format!("\x1B[35m{}\x1B[0m", text),
            Color::White => format!("\x1B[37m{}\x1B[0m", text),
            Color::Reset => format!("\x1B[0m{}\x1B[0m", text),
        }
    }
}

// pub fn blue(text: &str) -> String {
//     format!("\x1B[34m{}\x1B[0m", text)
// }

// #[allow(dead_code)]
// pub fn yellow(text: &str) -> String {
//     format!("\x1B[33m{}\x1B[0m", text)
// }
// #[allow(dead_code)]
// pub fn green(text: &str) -> String {
//     format!("\x1B[32m{}\x1B[0m", text)
// }
// #[allow(dead_code)]
// pub fn red(text: &str) -> String {
//     format!("\x1B[31m{}\x1B[0m", text)
// }
// #[allow(dead_code)]
// pub fn cyan(text: &str) -> String {
//     format!("\x1B[36m{}\x1B[0m", text)
// }
// #[allow(dead_code)]
// pub fn magenta(text: &str) -> String {
//     format!("\x1B[35m{}\x1B[0m", text)
// }
// #[allow(dead_code)]
// pub fn white(text: &str) -> String {
//     format!("\x1B[37m{}\x1B[0m", text)
// }
// #[allow(dead_code)]
// pub fn reset(text: &str) -> String {
//     format!("\x1B[0m{}\x1B[0m", text)
// }
