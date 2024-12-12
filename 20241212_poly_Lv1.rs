// ------------------
// DuckDuckGo AI Chat
// ------------------
//
// 初級問題
//
// Shapeというトレイトを定義し、areaメソッドを持たせます。
//
// このメソッドは、面積を計算して返すものとします。
//
// CircleとRectangleという構造体を作成し、それぞれの構造体に対してShapeトレイトを実装します。
//
// main関数で、CircleとRectangleのインスタンスを作成し、面積を計算して表示します。

// --------
// Comments
// --------
//
// - AI君がdynの事を「動的ディスパッチ」って言ってた (なにそれ？)
//
//   - 実行時にどのメソッドを呼び出すかを決定する仕組みらしい
//
//     - へぇぇぇぇぇぇぇぇぇぇぇぇぇぇええええぇぇぇぇえぇぇぇぇえぇええぇぇえ
//
// - 大文字Selfは自分を表す型らしい
//
// - 小文字selfは自分を表すポインタ？参照？なんかそんなイメージ
//
//   - インスタンスにアクセスできる (C++の*thisみたいなやつ)
//
// - 生成時に変数名と型名が同じだったら省略した書き方が出来る
//
//   - ほぇ～って感じ
//
//   - あと、セミコロン無しでreturnを省略出来る (値を返すようになるから)
//
//     - ほえぇぇぇぇ～～～って感じ
//
//     - ケアレスミス量産仕様だなぁ～
//
// - new関数作ると生成楽だわ(笑)
//
// - 言われてないけどBox,dyn使った卍 (ポリモってるねぇ)
//
// - forに渡すときもムーブされるから&付けようね...(´;ω;｀)
//
// - Rustは可変長引数を受け取る関数が定義できないからVecを初期化するのもマクロらしい
//
//   - ほえぇぇぇぇ～～～ほえぇぇぇぇ～～～ほえぇぇぇぇ～～～
//
// - Box::new()忘れんなよっ！！ <-- 絶対忘れる
//
//   - あと、dynもなっ！！ <-- 絶対忘れる

use std::f64::consts::PI;

trait Shape {
  fn area(&self) -> f64;
}

struct Circle {
  radius: f64,
}

impl Circle {
  fn new(radius: f64) -> Self {
    Self { radius }
  }
}

impl Shape for Circle {
  fn area(&self) -> f64 {
    self.radius * self.radius * PI
  }
}

struct Rectangle {
  width: f64,
  height: f64,
}

impl Rectangle {
  fn new(width: f64, height: f64) -> Self {
    Self { width, height }
  }
}

impl Shape for Rectangle {
  fn area(&self) -> f64 {
    self.width * self.height
  }
}

fn main() {
  let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle::new(10.0)),
    Box::new(Rectangle::new(10.0, 10.0)),
  ];

  for shape in &shapes {
    println!("{}", shape.area());
  }
}
