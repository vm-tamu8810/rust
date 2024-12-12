// ------------------
// DuckDuckGo AI Chat
// ------------------
//
// 初級問題
//
// 次のenumを定義し、各バリアントに対して値を持たせてください。
//
// バリアントは「赤」「青」「緑」とします。各色の名前を表示する関数を実装してください。
//
// enum Color {
//     // ここにバリアントを定義
// }
//
// fn display_color(color: Color) {
//     // ここに色を表示するロジックを実装
// }

// --------
// Comments
// --------
//
// - 値を持たせる意味は無いので値無しとする
//
// - display_color(color: Color)だとムーブが起きるので修正した

enum Color {
  Red,
  Blue,
  Green,
}

fn display_color(color: &Color) {
  match color {
    Color::Red => println!("Red"),
    Color::Blue => println!("Blue"),
    Color::Green => println!("Green"),
  }
}

fn main() {
  display_color(&Color::Red);
  display_color(&Color::Blue);
  display_color(&Color::Green);
}
