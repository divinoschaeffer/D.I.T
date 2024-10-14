use colored::Colorize;
pub enum Color {
     BLUE,
     RED,
     GREEN,
     DEFAULT
}

pub fn display_message(string: &str, color: Color) {
     match color {
          Color::BLUE => println!("{}", string.blue()),
          Color::RED => println!("{}", string.red()),
          Color::GREEN =>  println!("{}", string.green()),
          Color::DEFAULT => println!("{}", string),
     }
}