/// # enumにデータを持たせる
///
/// 1. 以下の要素を持つ列挙型`Shape`を定義
///
///   - Circle(f64)
///
///   - Rectangle(f64, f64)
///
///   - Triangle(f64, f64, f64)
///
/// 2. 各形状の面積を計算する関数を作成
///
/// # 学んだ事
///
/// 1. enum内のデータはmatchやif letで取り出す
///
/// ```
/// #[allow(dead_code)]
/// enum Shape {
///   Circle(f64),
///   Rectangle(f64, f64),
/// }
///
/// fn main() {
///   let shape = Shape::Circle(3.0);
///
///   match shape {
///     Shape::Circle(r) => println!("Circle with radius: {}", r),
///     Shape::Rectangle(w, h) => println!("Rectangle with width: {} and height: {}", w, h),
///   }
///
///   if let Shape::Circle(r) = &shape {
///     println!("Circle with radius: {}", r);
///   }
/// }
/// ```
///
/// 2. sqrtはチェーンして呼び出す
///
/// ```
/// fn main() {
///   println!("sqrt of 3.0: {}", 3f32.sqrt());
///   println!("sqrt of 3.0: {}", 3f64.sqrt());
/// }
/// ```

use std::f64::consts::PI;

enum Shape {
  Circle(f64),
  Rectangle(f64, f64),
  Triangle(f64, f64, f64),
}

fn calcurate_area(shape: &Shape) -> f64 {
  match shape {
    Shape::Circle(r) => r * r * PI,
    Shape::Rectangle(w, h) => w * h,
    Shape::Triangle(a, b, c) => {
      let s = (a + b + c) / 2.0;
      (s * (s - a) * (s - b) * (s - c)).sqrt()
    },
  }
}

fn main() {
  let c = calcurate_area(&Shape::Circle(3.0));
  let s = calcurate_area(&Shape::Rectangle(3.0, 3.0));
  let t = calcurate_area(&Shape::Triangle(3.0, 3.0, 3.0));

  println!("Area of the circle: {}", c);
  println!("Area of the rectangle: {}", s);
  println!("Area of the triangle: {}", t);
}
