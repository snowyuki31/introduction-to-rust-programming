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
