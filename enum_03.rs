/// # enumの列挙
///
/// 1. 信号機を表す列挙型`TrafficLight`を定義
///
/// 2. 以下の通りに信号機を状態遷移させる関数を作成
///
///   1. 赤 --> 緑
///
///   2. 緑 --> 黄
///
///   3. 黄 --> 赤
///
/// # 学んだ事
///
/// 1. 参照先の値を変更する時は*を付ける
///
/// ```
/// fn change_i32(value: &mut i32) {
///   *value = 10;
/// }
///
/// fn main() {
///   let mut value = 0;
///
///   println!("value: {}", value);
///
///   change_i32(&mut value);
///   println!("value: {}", value);
/// }
/// ```
///
/// 2. matchは式なので戻り値を代入可能
///
/// ```
/// fn main() {
///   let flag = true;
///
///   let value = match flag {
///     true => 10,
///     false => 0,
///   };
///
///   println!("value: {}", value);
/// }
/// ```

enum TrafficLight {
  Red,
  Green,
  Yellow,
}

fn transition_traffic_light(light: &mut TrafficLight) {
  *light = match light {
    TrafficLight::Red => TrafficLight::Green,
    TrafficLight::Green => TrafficLight::Yellow,
    TrafficLight::Yellow => TrafficLight::Red,
  };
}

fn print_traffic_light(light: &TrafficLight) {
  match light {
    TrafficLight::Red => println!("赤"),
    TrafficLight::Green => println!("緑"),
    TrafficLight::Yellow => println!("黄"),
  }
}

fn main() {
  let mut light = TrafficLight::Red;

  print_traffic_light(&light);

  transition_traffic_light(&mut light);
  print_traffic_light(&light);

  transition_traffic_light(&mut light);
  print_traffic_light(&light);

  transition_traffic_light(&mut light);
  print_traffic_light(&light);
}
