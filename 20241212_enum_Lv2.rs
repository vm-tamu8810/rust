// ------------------
// DuckDuckGo AI Chat
// ------------------
//
// 中級問題
//
// 次のenumを定義し、バリアントに異なるデータ型を持たせてください。
//
// バリアントは「整数」「浮動小数点数」「文字列」とします。
//
// 各バリアントの値を表示する関数を実装してください。
//
// enum Value {
//     // ここにバリアントを定義
// }
//
// fn display_value(value: Value) {
//     // ここに値を表示するロジックを実装
// }

// --------
// Comments
// --------
//
// - 20241212_enum_Lv1.rsと殆ど同じ問題...
//
//   - 一応、値の取得がタスクに追加されている (中級...？)
//
// - display_value(value: Value)だとムーブが起きるので修正した
//
//   - これも20241212_enum_Lv1.rsで書いた...

enum Value {
  Int(i32),
  Float(f32),
  Str(String),
}

fn display_value(value: &Value) {
  match value {
    Value::Int(value) => println!("{}", value),
    Value::Float(value) => println!("{}", value),
    Value::Str(value) => println!("{}", value),
  }
}

fn main() {
  display_value(&Value::Int(10));
  display_value(&Value::Float(10f32));
  display_value(&Value::Str("なめたけ舐めたっけ？"));
}
