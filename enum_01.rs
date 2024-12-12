/// # 基本的なenumの定義
///
/// 1. RGBを表す列挙型`Color`を定義
///
/// 2. 各色素の名前を表示する関数を作成
///
/// # 学んだ事
///
/// 1. デフォルトでenumはムーブされる
///
/// 2. enumに一つでも未使用の要素がある -> #[warn(dead_code)]
///
/// ```
/// enum Color {
///   Red,
///   Green,
///   Blue,
/// }
///
/// fn print_color_name(color: &Color) {
///   match color {
///     Color::Red   => println!("Red"),
///     Color::Green => println!("Green"),
///     Color::Blue  => println!("Blue"),
///   }
/// }
///
/// fn main() {
///   print_color_name(&Color::Red);
///   print_color_name(&Color::Green);
///   // print_color_name(&Color::Blue);
/// }
/// ```
///
/// 3. rustdocは文書化するアイテムの上に置かないとエラーが出る
///
/// ```
/// fn main() {
/// }
///
/// /// # Inappropriate documentation
/// ```

enum Color {
  Red,
  Green,
  Blue,
}

fn print_color_name(color: &Color) {
  match color {
    Color::Red => println!("Red"),
    Color::Green => println!("Green"),
    Color::Blue => println!("Blue"),
  }
}

fn main() {
  print_color_name(&Color::Red);
  print_color_name(&Color::Green);
  print_color_name(&Color::Blue);
}
