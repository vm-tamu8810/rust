enum Color {
  Red,
  Green,
  Blue,
}

fn print_color_name(color: &Color) {
  match color {
    Color::Red   => println!("赤色"),
    Color::Green => println!("緑色"),
    Color::Blue  => println!("青色"),
  }
}

fn main() {
  let color1 = Color::Red;
  let color2 = Color::Green;
  let color3 = Color::Blue;

  print_color_name(&color1);
  print_color_name(&color2);
  print_color_name(&color3);
}
