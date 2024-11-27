# Rust開発準備

## 1. インストール

- [Installation - The Rust Programming Language](https://doc.rust-lang.org/book/ch01-01-installation.html)
- [インストール - The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/ch01-01-installation.html)

## 2. Hello, World!

Rustのプロジェクトを `cargo` と呼ばれるRustのプロジェクト管理ソフトウェアで作成する。以下を実行すると、Rustプロジェクトファイルが作成される。

```bash
cargo new hello_world --bin
```

`hello_world` という名前のRustプロジェクトが作成される。プロジェクトの基本構造は以下のようになっている。

```text
 /
 ├ Cargo.toml
 └ src
    └ main.rs 
```

`Cargo.toml` は cargo の設定ファイルである。後に触れる。

Rustの中身は以下の通りである。

```rust
fn main() {
    println!("Hello, world!");
}
```

`fn` は関数宣言を表しており、 `main` 関数名で `{}` の中に処理を書き込む。 `cargo new` はこのように Hello World のプログラムを生成してくれる。 `cargo run` でRustプログラムをコンパイルして実行する。

## 3. 変数とデータ型

#### 束縛

まず変数の宣言は `let` キーワードを使う。宣言の直後に `=` で値を指定することで「値を変数に束縛する」という。

```rust
let message = "hello, foo";
```

宣言と束縛は分けることができるため、後で if の中で束縛することが可能である。

```rust
let message;
if true {
    message = "hello, foo";
} else {
    message = "hello, bar";
}
```

#### データ型

- 変数型　`i8`, `i16`, `i32`
- 浮動小数点型　`f32`, `f64`
- 論理型　`bool`
- 文字型　`char`
- タプル　`()`
- 配列型　`[]`
- ベクター型　`Vec<type>`
- 文字列型　`""`
- ハッシュマップ　`HashMap<K, V>`

## 4. 関数の実装

#### 戻り値

Rustの関数は全て戻り値を持っていって、結果を束縛することができる。しかし明示的な戻り値がない場合は、 **空のタプル** が戻り値となる。これは void に相当する。 Rustでは戻り値が必須であるため、関数の最後に式を置くと、暗黙的に return されるので、 `add` 関数は以下のように省略することも可能である。

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

#### enum

```rust
enum Color {
    Red,
    Blue, 
    Green, 
    Hex(String),
}

let red = Color::Red;
let hex = Color::Hex("ffffff".to_string());
```

## 5. エラー

#### 回復可能なエラー

Rustでは、エラーハンドリングに `Result` と呼ばれる列挙型を用いる。 `Result` は以下のような形式である。

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- `T`は、処理が成功した時に`Ok`列挙子に含まれて返される値の型である
- `E`は、失敗した時に`Err`列挙子に含まれて返される値の型である

例として、以下のような関数を呼び出してみる。この関数は失敗する可能性がいくつかあるが、最も考えられるのは、指定したファイルパスが存在しない場合である。

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

試しに、わざとコンパイルエラーを発生させる。`f`の戻り値の型は `std::result::Result<std::fs::File, std::io::Error>` だが、あえて誤った戻り値の型を注釈してみる。

```rust
use std::fs::File;

fn main() {
    let f: u32 = File::open("hello.txt");
}
```

すると、以下のようなエラーが発生する。

```bash
E:\Project\rust\Practice-Rust\project1\hello_cargo>cargo run
   Compiling hello_cargo v0.1.0 (E:\Project\rust\Practice-Rust\project1\hello_cargo)
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |            ---   ^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found `Result<File, Error>`
  |            |
  |            expected due to this
  |
  = note: expected type `u32`
             found enum `Result<File, std::io::Error>`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `hello_cargo` (bin "hello_cargo") due to 1 previous error
```

`File::open` が成功した場合、変数 `f` の値はファイルハンドリングを含む `Ok` インスタンスになる。失敗した場合は、発生したエラーの種類に関する情報を含む `Err` インスタンスが `f` の値となる。

では、エラーをハンドリングしてみる。まずは最も基本的な形である。

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            // ファイルを開く際に問題がありました
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
```

結果が `Ok` の時に、 `Ok`列挙子から中身の`file` の値を返すように指示し、それからそのファイルハンドル値を変数`f`に代入している。

```rust
use std::{fs::File, io::ErrorKind};

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            // ファイルが存在しないなら、作成する
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        //ファイルを作成しようとしましたが、問題がありました
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                }
            }
        }
        Err(error) => {
            // 予期せぬエラー
            panic!("Problem opening the file: {:?}", error)
        }
    };
}
```

File::openがErr列挙子に含めて返す値の型は、io::Errorであり、これは標準ライブラリで提供されている構造体である。この例では、エラーの種類毎に処理を変えている。`Err`の返す値である`io::ErrorKind`は、 kindメソッドによって、そのエラーを受け取ることができる。その値が、指定したファイルが存在しないことを表す `ErrorKind::NotFound` であれば、そのファイルを新規作成するような処理を挿れている。

#### unwrapとexpect

matchの使用は、十分に仕事をしてくれるが、いささか冗長になり得る上、必ずしも意図をよく伝えるとは限らない。 `unwrap` メソッドでは、Result値がOk列挙子なら、unwrapはOkの中身を返す。

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

`hello.txt` ファイル無しで実行すると

```bash
E:\Project\rust\Practice-Rust\project1\hello_cargo>cargo run
   Compiling hello_cargo v0.1.0 (E:\Project\rust\Practice-Rust\project1\hello_cargo)
warning: unused variable: `f`
 --> src/main.rs:4:9
  |
4 |     let f = File::open("hello.txt").unwrap();
  |         ^ help: if this is intentional, prefix it with an underscore: `_f`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `hello_cargo` (bin "hello_cargo") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running `target\debug\hello_cargo.exe`
thread 'main' panicked at src/main.rs:4:37:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "指定されたファイルが見つかりません。" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\hello_cargo.exe` (exit code: 101)
```

というように、unwrapに実装されたpanic!が実行される。また `expect`メソッドは、`unwrap` に似ているが、エラーメッセージを提供できる点が異なる。

```rust
use std::fs::File;

fn main() {
    // hello.txtを開くのに失敗しました
    let f = File::open("hello.txt").expect("hello.txtを開くのに失敗しました！！！");
}
```

実行すると以下のようなエラーが出現する。

```bash
E:\Project\rust\Practice-Rust\project1\hello_cargo>cargo run
   Compiling hello_cargo v0.1.0 (E:\Project\rust\Practice-Rust\project1\hello_cargo)
warning: unused variable: `f`
 --> src/main.rs:5:9
  |
5 |     let f = File::open("hello.txt").expect("hello.txtを開くのに失敗しました！！！");
  |         ^ help: if this is intentional, prefix it with an underscore: `_f`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `hello_cargo` (bin "hello_cargo") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target\debug\hello_cargo.exe`
thread 'main' panicked at src/main.rs:5:37:
hello.txtを開くのに失敗しました！！！: Os { code: 2, kind: NotFound, message: "指定されたファイルが見つかりません。" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\hello_cargo.exe` (exit code: 101)
```

#### 委譲

失敗する可能性のある何らかの処理を呼び出す場合、関数内でエラー処理する代わりに、呼び出し元に処理を委譲できる。

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

まず、関数の戻り値に注意したい。まず `Result<String, io::Error>` を返している。これは、`T`は `hello.txt` をロードして返されるテキスト文に埋められ、`E`は`File::open`のエラー型である `io:Error` をそのまま返している。

この処理では、まず

```rust
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
```

で `File::open`のエラーをハンドリングし、 `f`が`Ok`を受け取ったら、`f`にファイルハンドル値を代入する。エラー（＝`f`が`Err`型）なら、早期リターンで `Err(e)` を返している。

次に、テキストファイルをロードするために、まずは、テキストファイルの中身を保持するための`String`型変数 `s`を宣言し、次に`f.read_to_string`の処理をエラーハンドリングしている。最終行の式はそのままリターンとなるため、ハンドル値が返される。

#### `?`演算子

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    let f = read_username_from_file();
    match f {
        Ok(s) => println!("{}", s),
        Err(ref error) if error.kind() == io::ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(_) => println!("File created"),
                Err(e) => println!("Failed to create file: {:?}", e),
            }
        }
        Err(error) => println!("Error: {:?}", error),
    }
}
```

`Result`値の直後に置かれた `?` は、`Result`値を処理するために定義した `match` 式とほぼ同じ用に動作する。 `Result` 値が `Ok` ならその中身が帰ってきて、 `Err`なら `return`キーワードを使ったかのように、その`Err`発生時の値がそのまま返ってくる。

## 6. マクロ

#### 宣言的マクロ

紐づけられたRustコードの構造が特定の形に合致した場合に、コードを生成して置き換える機能である。

```rust
macro_rules! sum {}
```

`sum`というマクロを宣言する。

```rust
macro_rules! sum {
    ($($x:expr),*) => { }
}
```

- `expr` : 式にマッチする。 `$x:expr`  
- `(),*` : `(),*`と書かれた部分は、コンマで区切られた繰り返しを指定している
- `$($x:expr),*` : つまり、式を繰り返し設定している

```rust
macro_rules! sum {
    ($($x:expr),*) => {
        {
            let mut sum = 0;
            $( sum += $x; )*
            sum
        }
    };
}

fn main() {
    let x = sum!(1, 2, 3, 4, 5);
    println!("Sum of 1, 2, 3, 4, 5 is: {}", x);
}
```

ちょっとまだ良くわかってないので、後でちゃんと調べる...。

## 7. 制御文

#### パターンマッチング

`if else` で出てくるパターンとして、以下のようなものがある。

```rust
fn fizz_buzz(value: i32) -> String {
    let result = if value % 3 == 0 {
        "Fizz".to_string()
    } else if value % 5 == 0 {
        "Buzz".to_string()
    } else {
        value.to_string()
    };
    result
}

fn main() {
    println!("{}", fizz_buzz(1));
    println!("{}", fizz_buzz(3));
    println!("{}", fizz_buzz(5));
}
```

`match` 文でのパターンマッチング。

```rust
enum Color {
    Red,
    Blue,
    Green,
}

fn string_to_color_token(value: &str) -> Option<Color> {
    match value {
        "red" => Some(Color::Red),
        "blue" => Some(Color::Blue),
        "green" => Some(Color::Green),
        _ => None,
    }
}
```

次の例は、 `string_to_color_token` の戻り値の Option が Some の場合のみ printしたい場合の実装例である。

```rust
fn main() {
    if let Some(color) = string_to_color_token("red") {
        println!("Color: {:?}", color);
    }
}
```

## 8. 所有権

所有権はRustの機能の１つで、メモリ安全を実現するための仕組みである。プログラムの実行にはメモリ管理が必要で、多くの言語では **ガベージコレクション** という形で定期的な監査によりメモリの自動開放ができる。しかし、ガベージコレクションは実行時にオーバーヘッドが発生するため、パフォーマンス的には不利である。一方、Cのような低級言語では、実装者自身によるメモリの確保や開放操作により高いパフォーマンスを実現できるが、メモリの開放漏れやメモリーリークにより、実装者に負荷が掛かっている。

#### 概要

所有権のルールを見てみる。

- Rustの各値は、 **所有者** と呼ばれる変数と対応している
- いかなる時も所有者は１つである
- 所有者がスコープから抜けたら、値は破棄される

#### MoveとCopy

Rustでは **所有権** という仕組みによって、ガベージコレクションのようなプログラム実行のオーバーヘッドなしにメモリ安全性を実現している。

```rust
fn main() {
    let a = String::from("hoge");
    let b = a; // aの所有権はbムーブされる
    println!("{}", a);  // error: value used here after move
}
```
Rustである変数を他の変数に束縛するときは、 moveと呼ばれる所有権の移動が行われ、元の変数は無効化される。上記の例では `a` の値がmoveした後に再度 `a` を利用しようとしたため、エラーとなっている。moveによって所有権を失った値を利用するということは既にメモリ解放が行われた値を参照する可能性があり、予期せぬ挙動の原因となるため、コンパイルエラーとなっている。

以下の例も同様である。

```rust
fn foo(b: i32) {
    /* 直接的にbを所有して利用 */
} // この関数がbの所有権を持っているのでDropされる

fn main() {
    let a = 42;
    foo(a);
    // a は fooに所有されてしまうので、使えなくなる
    // println!("{}", a); // error[E0382]: use of moved value: `a`
}
```

関数`foo`は`b`を所有しているので、関数が終了する時に`b`は破棄される。そのリソースの所有権は複数ヵ所で持てない。所有者が最後にそのリソースを開放するため、常に一ヵ所である。

一方、以下のコードはコンパルが通る。

```rust
fn main() {
    let a = 100;
    let b = a;
    println!("a = {}, b = {}", a, b);
}
```

i32型はCopyトレイトを実装していて、moveではなく値のコピーが可能だからである。変数束縛を行う `=` は moveを行う場合とコピーを行う場合があり、Copyトレイトを実装していれば値のコピーが発生する。Copyは実際にメモリ上でデータ自身をコピーするが、moveの場合はポインタがコピーされる。

```rust
fn main() {
    // 100はi32型で Copy トレイトを実装しているので、aにコピーされる
    let a = 100;
    let b = a;

    // String型はCopyトレイトを実装していないので、所有権が移動する
    let a = String::from("Hello");
    let b = a;
}
```

#### 借用

値に対応する変数が必ず１つであると制限されると不便なケースも存在する。Rustでは **借用** といった仕組みが用意されている。

```rust
fn hello_print(message: String) {
    println!("hello, {}", message);
}

fn main() {
    let world = String::from("world");

    // worldの値(String)はhello_print関数に所有権がムーブされる
    hello_print(world);

    // worldの値はhello_print関数にムーブされたため、ここでのworldの値は使えない
    // println!("hello, {}", world); 
    //  ---> error[E0382]: borrow of moved value: `world`
}
```

worldの所有権がhello_printにムーブしているので、それ以降、worldは利用できない。しかし、本質的にhello_printは所有権を必要とするような操作（値の変更など）を行わないため、この場合は **参照** を渡すだけで十分である。

```rust
// &Stringは内部的に&strに変換されるので、明示的に&strを指定
fn hello_print(message: &str) {
    println!("hello, {}", message);
}

fn main() {
    let world = String::from("world");
    hello_print(&world);
    println!("main print: {}", world);
}
```

#### 値のClone

借用は便利な機能だが、どうしても所有権を複数回moveしたいようなケースも有る。そういった場合にはCloneトレイトのcloneメソッドを利用して、明示的に値をコピーする。

```rust
fn take<T>(_value: T) {
    // valueの所有権を奪うだけの関数
}

fn fizz(value: i32) -> String {
    let result = if value % 3 == 0 {
        String::from("fizz");
    } else {
        format!("{}", value);
    };
    take(result);
    result
}

fn main() {
    let value = 3;
    let result = fizz(value);
    println!("{}", result);
}
```

```bash
E:\Project\rust\Practice-Rust\project1\hello_cargo>cargo run
   Compiling hello_cargo v0.1.0 (E:\Project\rust\Practice-Rust\project1\hello_cargo)
error[E0308]: mismatched types
  --> src/main.rs:12:5
   |
5  | fn fizz(value: i32) -> String {
   |                        ------ expected `String` because of return type
...
12 |     result
   |     ^^^^^^ expected `String`, found `()`
   |
help: remove this semicolon to return this value
   |
7  -         String::from("fizz");
7  +         String::from("fizz")
   |
help: remove this semicolon to return this value
   |
9  -         format!("{}", value);
9  +         format!("{}", value)
   |

For more information about this error, try `rustc --explain E0308`.
error: could not compile `hello_cargo` (bin "hello_cargo") due to 1 previous error
```

このように値の所有権が複数の処理で必要な場合は、明示的に値をコピーするcloneメソッドを使うことで値をコピーできる。

```rust
fn take<T>(_value: T) {
    // valueの所有権を奪うだけで、何もしない
}

fn fizz(value: i32) -> String {
    let result = if value % 3 == 0 {
        "Fizz".to_string()
    } else {
        value.to_string()
    };

    let cloned_result = result.clone();
    take(cloned_result);
    result
}

fn main() {
    let result = fizz(3);
    println!("{}", result);
}
```

#### moveと借用のアンチパターン

**参照を受け取るが、関数内部でわざわざ複製を作る例**

```rust
// 良い例:
fn foo(b: String) {
    // bを所持して利用する処理
    // ...
    // この関数がbの所有権を持っているのでDropされる
}


// 悪い例:
fn foo(b: &Bar) {
  let b = b.clone(); // わざわざcloneする…。
  /* 複製後にbを所有して利用 */
  // 複製されたbがDropされるが、参照のbは借用なのでDropされない
} 
```

引数の所有権を要する関数は、借用して複製するのではなく所有権を受け取るべき。無意味なcloneはNG。moveされて困る場合は、以下のように呼び出し元に複製の決定権を与えるべき。

```rust
let a = Bar::new();
foo(a.clone()); // 呼び出し元が複製を作るかを決める
println!("{:?}", a);
```

**所有権を奪うが、関数内部で所有権を利用しない例**

もし関数が引数の所有権を必要としないのなら、 所有権を取って最終的にdropする代わりに可変あるいは非可変借用を受け取るべき

```rust
// 良い例:
fn foo(b: &Bar) {
  /* bを借用して利用 */
}

// 悪い例:
fn foo(b: Bar) {
  /* 実質的にbは所有されず借用のみ。関数の最後にDropされる */
} // この関数がbの所有権を持っているのでDropされる
```

詳しくは https://sinkuu.github.io/api-guidelines/flexibility.html 。