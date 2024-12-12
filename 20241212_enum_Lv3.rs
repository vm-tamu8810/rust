// ------------------
// DuckDuckGo AI Chat
// ------------------
//
// 上級問題
//
// 次のenumを定義し、バリアントに構造体を持たせてください。
//
// バリアントは「円」「四角形」とし、それぞれの形状の面積を計算するメソッドを実装してください。
//
// struct Circle {
//     radius: f64,
// }
//
// struct Rectangle {
//     width: f64,
//     height: f64,
// }
//
// enum Shape {
//     // ここにバリアントを定義
// }
//
// impl Shape {
//     fn area(&self) -> f64 {
//         ここに面積を計算するロジックを実装
//     }
// }

// --------
// Comments
// --------
//
// - トレイトっていう便利なものがあってだな...
//
//   - enumでポリモフィズムは無理があるな～
//
//     - さすがに冗長すぎるぜ！！
//
// - enumのバリアントと値の型名は被っても良いみたい
//
//   - ほぇ～ってなった
//
// - 構造体は初期化関数作らんと生成めんどいな～
//
//   - メンバ名をわざわざ書かないといけないみたい (C++スタイルの初期化が出来ない)
//
//     - コードが長げぇよ！！

use std::f64::consts::PI;

struct Circle {
  radius: f64,
}

struct Rectangle {
  width: f64,
  height: f64,
}

enum Shape {
  Circle(Circle),
  Rectangle(Rectangle),
}

impl Shape {
  fn area(&self) -> f64 {
    match self {
      Shape::Circle(circle) => circle.radius * circle.radius * PI,
      Shape::Rectangle(rectangle) => rectangle.width * rectangle.height,
    }
  }
}

fn main() {
  let circle = Shape::Circle(Circle { radius: 10.0 });
  let rectangle = Shape::Rectangle(Rectangle { width: 10.0, height: 10.0 });

  println!("area of circle: {}", circle.area());
  println!("area of rectangle: {}", rectangle.area());
}
