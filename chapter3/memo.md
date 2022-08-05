# chapter 3

## Box 型

Box 型は値をヒープ領域に格納する.

この場合，コンパイル時にサイズがわかっていなくても良い

また，共通のトレイトを実装した様々な型を画一的にポインタで扱うためにも用いる

```rust
fn main() {
    let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    print(Box::new(byte_array));
    let byte_array = [b'w', b'o', b'r',b'l',b'd', b'!'];
    print(Box::new(byte_array));
}

fn print(s: Box<[u8]>) {
    println!("{:?}", s);
}
```

## const と static

任意のスコープで利用できるキーワード

const は常に変更不可能で，コンパイル時に実際の値に置き換えられる

一方で static は安全の保証ができないので unsafe ブロックで扱う必要がある

実行時に決まる定数を定義する場合は，lazy_static という外部クレートを用いると良い

## 制御文

if は式として扱われるので，変数に束縛したり関数の引数にしたりすることができる

```rust
let number = 1;
let result = if 0 <= number {
    number
}else {
    -number
};
```

ループの一種である loop も同様のことができる

その際は break に戻り値を明示する

```rust
let mut count = 0;
let result = loop {
    println!("count: {}", count);
    count += 1;
    if count == 10 {
        break count;
    }
}
```

for 文

```rust
let count: i32;

for count in 0..12{
    println!("count: {}", count);
}

let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

for element in &array {
    println!("element: {}", element);
}
```

break 文でラベルを指定して抜けることができる

```rust
'main: loop {
    println!("main loop start");
    'sub: loop {
        println!("sub loop start");

        break 'main;

        println!("sub loop end"); // 表示されない
    }
    println!("main loop end"); // 表示されない
}
```

変数によって処理を分岐させることができる Rust の match は非常に強力

```rust
let i: i32 = 1;
match i {
    1 => println!("1"),
    2 => println!("2"),
    3 => println!("3"),
    _ => println!("misc"),
}
```

列挙型でも同様．以下のように分岐を網羅していないとエラーが出る．

```rust
enum Color {
    Red, Blue, Green,
}
let c = Color::Red;
match c {
    Color::Red => println!("Red"),
    Color::Blue => println!("Blue"),
    // Color::Green => println!("Green"),
}
```

match は式である

```rust
let result: Result<i32, String> = Ok(100);
let result_number = match result {
    Ok(number) => number,
    Err(message) => {
        println!("Error: {}", message);
        -1
    },
}
```

Range の書き方

```rust
for number in 1..5 {
    println!("Number: {}", number);
}
```

`1..=5`とすると，RangeInclusive 型になり，閉区間となる

Iterator トレイトを実装すると自作の型に for 文を用いることができる

```rust
fn main() {
    let it = Iter {
        current: 0,
        max: 10,
    };
    for num in it {
        println!("Number: {}", num);
    }
}

struct Iter {
    current: usize,
    max: usize,
}

impl Iterator for Iter {
    type Item = usize; // 出力する型の紐付け

    // next()メソッドの実装
    fn next(&mut self)-> Option<usize> {
        self.current += 1;
        if self.current -1 < self.max {
            Some(self.current - 1)
        } else {
            None
        }
    }
}
```

## 関数

関数は fn を用いる．関数の最後にセミコロンなしで表記された値は，return で明示しなくても返り値としてみなされる

```rust
fn main() {
    let x = add(1, 2);
    println("x: {}", x);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## impl

impl を使うことで構造体にメソッドを加える

```rust
struct Person {
    name: String,
    age: u32,
}

impl Preson {
    fn say_name(&self) {
        println!("I am {}.", self.name);
    }

    fn say_age(&self) {
        println!("I am {} year(s) old.", self.age);
    }
}

fn main() {
    let p = Person{
        name: String::from("Taro"),
        age: 20,
    };

    p.say_name();
    p.say_age();
}
```

impl のメソッドの返り値に自分自身の型を明示することでメソッドチェーンを実装できる

```rust
impl Person {
    fn say_name(&self) -> &Self {
        println!("I am {}.", self.name);
        self
    }

    fn say_age(&self) -> &Self {
        println!("I am {} year(s) old.", self.age);
        self
    }
}

fn main() {
    let p = Person{
        name: String::from("Taro"),
        age: 20,
    };

    p.say_name().say_age();
}
```

メソッドによる構造体のメンバ変数の変更をさせるには，`&mut self`とする

また，第一引数に self を使わないと，関連関数と定義される．
関連関数は，インスタンスからではなく，型から関数を呼ぶ

```rust
impl Person {
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: String::from(name),
            age: age,
        }
    }
}

fn main() {
    let p = Person::new("Taro", 20); // 関連関数である
    p.say_name().say_age();
}
```

## 各種マクロ

`format!`, `concat!`

```rust
let s = format!("{}--{:?}", s, ("D", 5));
let s = format!("{}--{}", "abc", "def")
let s = concat!("A", "b2", 3)
```

標準出力，エラー系

`print!`, `println!`, `eprint!`, `eprintln!`, `write!`, `writeln`, `dbg!`

異常終了

`panic!`

```rust
fn main() {
    panic!("it will panic")
}
```

プログラム外のリソースにアクセスする系

`file!`, `line!`, `cfg!`, `env!`

アサーション

`assert!`, `debug_assert!`, `assert_eq!`, `assert_ne!`, `debug_assert_eq!`, `debug_assert_ne!`

実装補助用マクロ

`unimplemented!`: 何らかの理由があり実装されていないことを示すマクロ

`todo!`: 今後実装しなければならないことを表現するマクロ

`unreachable!`: 処理が到達しないことを示すマクロ

```rust
fn f(x: usize) -> &'static str{
    match x {
        n if n * n % 3 == 0 => "3n",
        n if n * n % 3 == 1 => "3n + 1 or 3n + 2",
        _ => unreachable!(), // コンパイラが上記の条件網羅を把握できない
    }
}
```

## トレイトの導出

いくつかのトレイトは，自作の型に対する標準的な実装を自動的に導出することができる

```rust
#[derive(Eq, PartialEq)]
struct A(i32);

fn main() {
    // 一致比較可能
    println!("{:?}", A(0)==A(1));
}
```

他にも，`Ord`, `PartialOrd`, `Copy`, `Clone`, `Debug`, `Default`などがある．

`PartialEq`は，浮動小数の NaN のように反射律($a=a$)を満たさない場合に必要となる，

`==`などの演算子は，`PartialEq`しか要求しないので問題なく浮動小数でも動作する

一方，`Vec`の sort の場合，`Ord`が要求されるので`Vec<f32>`はソートすることができない

以下のように扱うことができるが，NaN が含まれていると unwrap で panic が生じる

```rust
fn main() {
    let mut x = vec![0.1, 0.5, 0.3, 0.4, 0.2];
    x.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", x);
}
```

## ゼロコスト抽象化

抽象化を行うと，抽象化されたコードから具体的な動作を導くための処理が必要となり，メモリを多く使用した処理が発生しがちである

ゼロコスト抽象化においては，抽象化されたトレイトやジェネリクスを使用する型に落とし込む作業をコンパイル時に行うことで余計なコストをゼロにしている

### trait

トレイトは，様々な型に共通のメソッドを実装するように促すことができる仕組みである

```rust
trait Tweet {
    fn tweet(&self);

    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }

    fn shout(&self) {
        println!("Uooh!!");
    }
}
```

実装する際は以下のように実装できる

```rust
struct Dove;
struct Duck;

impl Tweet for Dove {
    fn tweet(&self) {
        println!("coo!");
    }
}

impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack!");
    }
}

fn main() {
    let dove = Dove {};
    dove.tweet();
    dove.tweet_twice();
    dove.shout();

    let duck = Duck {};

    let brid_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
    for bird in bird_vec {
        bird.tweet();
    }
}
```

一般に，メソッドの呼び出しには動的ディスパッチと静的ディスパッチが存在する．

動的ディスパッチでは，メソッドが呼び出された時，呼び出したインスタンスを確認し，それに合わせた処理を実行時に決める方式．

静的ディスパッチは，コンパイル時に実行する処理を決めてしまう．

Rust では，`dyn`キーワードを使うときのみ，動的ディスパッチを適用するようにしている．

### マーカトレイト

`Copy`, `Sync`などメソッドのない，マーカー的な役割をするトレイトもある．

コンパイラが安全性の検査や最適化をする際に使用する

### 所有権と借用

Rust では，それぞれの値には所有権があり，その所有権を持っているのは必ず一つの変数（所有者）だけであると定められている．

所有者のスコープが外れたら，値は破棄されることでメモリの二重解放といったエラーを回避している．

ただし，この方法だと関数に値を引数で渡すときに所有権が移ってしまうという問題がある．

このような場合に値の参照を貸し出すことができ，これを借用と呼ぶ．

値の参照が借用されている間に，所有者である変数が値を破棄してしまうと参照エラーが生じてしまうので，これを防ぐために「ライフタイム」が明確に定まっている

#### ムーブセマンティクス

Rust では，変数は値に束縛され，その所有権を持つことになる．

変数同士を等号で結ぶと，所有権が譲渡される（ムーブセマンティクス）

```rust
struct Color {
    r: i32,
    g: i32,
    b: i32,
}

fn main() {
    let a = Color{r:255,g:255,b:255};
    let b = a; // 所有権の譲渡
    println!("{} {} {}", b.r, b.g, b.b)
}
```
