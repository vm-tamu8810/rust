// ------------------
// DuckDuckGo AI Chat
// ------------------
//
// 実践級問題
//
// 次のenumを定義し、バリアントに異なるデータ型を持たせ、さらにそれぞれのバリアントに対して異なる処理を行う関数を実装してください。
//
// バリアントは「成功」「失敗」とし、成功時には結果を、失敗時にはエラーメッセージを持たせます。
//
// 結果を表示する関数を実装してください。
//
// enum Result<T, E> {
//     // ここにバリアントを定義
// }
//
// fn display_result(result: Result<i32, String>) {
//     // ここに結果を表示するロジックを実装
// }

// --------
// Comments
// --------
//
// - 劣化版Result作らされた(笑)
//
//   - 標準ライブラリにあるや～つ
//
// - 問題としてはジェネリックになっただけ (実践級...？)
//
// - ジェネリックはC++みたいに場合分け出来るみたい
//
// - 文字列リテラルをいちいちto_string()しないといけないのメンドイ...
//
// - 5個くらいエラー出たけどタイポのせいだった(´;ω;｀)

enum Result<T, E> {
  Success(T),
  Failure(E),
}

fn display_result(result: &Result<i32, String>) {
  match result {
    Result::Success(value) => println!("成功！ {}", value),
    Result::Failure(msg) => println!("失敗！ {}", msg),
  }
}

fn main() {
  display_result(&Result::Success(10));
  display_result(&Result::Failure("ざんね～ん(笑)".to_string()));
}
