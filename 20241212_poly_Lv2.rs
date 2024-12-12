// ------------------
// DuckDuckGo AI Chat
// ------------------
//
// 中級問題
//
// Animalというトレイトを定義し、speakメソッドを持たせます。
//
// このメソッドは、動物の鳴き声を返すものとします。
//
// DogとCatという構造体を作成し、それぞれの構造体に対してAnimalトレイトを実装します。
//
// Vec<Box<dyn Animal>>を使って、DogとCatのインスタンスを格納するベクタを作成し、各動物の鳴き声を表示します。

// --------
// Comments
// --------
//
// - らくしょーですわぁ～
//
// - マクロ呼び出しは()でも{}でも[]でもOKだって
//
// - メンバにアクセスしない関数でも.で呼び出すなら&self必要だってよっ！！

trait Animal {
  fn speak(&self);
}

struct Dog;

impl Animal for Dog {
  fn speak(&self) {
    println!("ぐるをｄくえｋｊｌすぃえｊｋｌｊぁ！！！！！！！！！");
  }
}

struct Cat;

impl Animal for Cat {
  fn speak(&self) {
    println!("にゅうっゆゆゆうゆゆいぇうゆゆゆゆいぇゆゆえゆｙれうゆえゆｙるいぇうゆえっゆれｒ！！！！！！！！！！！");
  }
}

fn main() {
  let mut animals: Vec<Box<dyn Animal>> = vec![];

  for _ in 0..10 {
    animals.push(Box::new(Dog {}));
    animals.push(Box::new(Cat {}));
  }

  for animal in animals {
    animal.speak();
  }
}
