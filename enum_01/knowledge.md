### デフォルトでenumはムーブされる

```rust
enum Sample {
  A,
}

fn print_sample(sample: Sample) {
  match sample {
    Sample::A => println!("A"),
  }
}

fn main() {
  let sample = Sample::A;

  print_sample(sample);
  print_sample(sample); // [E0382]
}
```

### enumに一つでも未使用の要素があると警告される

```rust
enum Sample {
  A,
  B, // #[warn(dead_code)]
}

fn print_sample(sample: &Sample) {
  match sample {
    Sample::A => println!("A"),
    Sample::B => println!("B"),
  }
}

fn main() {
  let sample = Sample::A;

  print_sample(&sample);
}
```

### ドキュメントはアイテムの上に書かないとエラーが出る

```rust
fn main() {

}

/// expected item after doc comment
```
