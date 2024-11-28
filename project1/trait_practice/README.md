# データ構造とトレイト

## データ構造

#### 構造体

構造体は `struct` で宣言するデータ構造体である。

```rust
struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User {
        name: "sato".to_string(),
        age: 30,
    };

    println!("name: {}, age: {}", user.name, user.age);
}
```

#### メソッド、関連関数

クラスのメソッドのように、特定の型構造体やenumに紐づく形で定義できるメソッドと関連関数がある。メソッドや関連関数は `impl` で宣言し、ブロック内に関数形式で定義できる。

```rust
struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    fn description(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

fn main() {
    let user = User {
        name: "John".to_string(),
        age: 30,
    };

    println!("{}", user.description());
}
```

メソッドは必ずインスタンスと紐づく関数のため、必ず最初の引数がインスタンス自身を表す `&self` という引数を取る。一方、 `&self` を受け取らない関数として関連関数がある。Rustにはコンストラクタはないが、慣習として `new` 関連関数を定義する。

また、インスタンス自身に変更を加えるようなメソッドを定義する場合は、 `&self` に `mut` で受け取り、インスタンス自身が `mut` であるように定義する。

```rust
    fn rename(&mut self, name: String) {
        self.name = name;
    }
```

## ジェネリック型

関数が未知の値の引数を受け取り、同じコードを複数の具体的な値として走らせる場合、具体的な型の代わりに、抽象的な型を受け取りたい場合がある。今まで出てきた `Option<T>` や `HashMap<K, V>` , `Result<T, E>` などはこれに該当する。

ジェネリクスは、様々な形に対応するために、型をパラメータとして与えて、その型に対応したクラスや関数を生成するものである。こうすることで同じようなロジックを型別に複製する必要がなくなる。

#### 関数定義

まずは関数において抽象的な型（ジェネリック）を受け取る記法を見る。

```rust
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

これで関数 `largest` は様々な型を受け取ることができるようになった。しかし、このままではコンパイルは通らない。

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

次のようなエラーが出る。

```rust
   Compiling trait_practice v0.1.0 (/home/yoshikan/project/Practice-Rust/project1/trait_practice)
error[E0369]: binary operation `>` cannot be applied to type `T`
(エラー: 2項演算`>`は、型`T`に適用できません)
 --> src/main.rs:4:17
  |
4 |         if item > largest {
  |            ---- ^ ------- T
  |            |
  |            T
  |
help: consider restricting type parameter `T`
```

#### 構造体定義

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

#### 列挙体定義

```rust
enum Option<T> {
    Some(T),
    None,
}
```

#### メソッド定義

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

## トレイト

トレイトは、任意のメソッドを実装することを強制する。

```rust
// Areaトレイト
trait Area {
    fn area(&self) -> u32;
}

// スクエア構造体
struct Square(u32);

// Square構造体にAreaを実装する
impl Area for Square {
    fn area(&self) -> u32 {
        self.0.pow(2)
    }
}

// コンストラクタを実装する
impl Square {
    fn new(side: u32) -> Self {
        Self(side)
    }
}

fn main() {
    let my_square = Square::new(5);
    println!("Area of square is: {}", my_square.area());
}
```

分数同士の足し算を実装する。

```rust
use std::ops::Add;

struct Fraction {
    numerator: u32,
    denominator: u32,
}

impl Fraction {
    fn new(numerator: u32, denominator: u32) -> Self {
        let gcf_value = Self::gcf(numerator, denominator);
        Fraction {
            numerator: numerator / gcf_value,
            denominator: denominator / gcf_value,
        }
    }

    fn gcf(value1: u32, value2: u32) -> u32 {
        // ユークリッドの互除法
        // 1. a / b で余り r を求める
        // 2. r > 0 なら b / r で余り r2 を求める
        // 3. r2 > 0 なら r / r2 で余り r3 を求める
        // 4. rnが0になるまで繰り返す
        // 5. 最後に余りが0になったときの除数が最大公約数

        let (mut a, mut b) = if value1 > value2 {
            (value1, value2)
        } else {
            (value2, value1)
        };

        let mut r = a % b;
        while r > 0 {
            a = b;
            b = r;
            r = a % b;
        }
        return b;
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let lcm = self.denominator / Fraction::gcf(self.denominator, other.denominator)
            * other.denominator;
        let a = self.numerator * (lcm / self.denominator);
        let b = other.numerator * (lcm / other.denominator);
        Fraction::new(a + b, lcm)
    }
}

fn main() {
    let a = Fraction::new(8, 18);
    println!("a = {}/{}", a.numerator, a.denominator);

    let b = Fraction::new(1, 2);
    println!("b = {}/{}", b.numerator, b.denominator);

    let c = a + b;
    println!("a + b = ");
    println!("{}/{}", c.numerator, c.denominator);
}
```