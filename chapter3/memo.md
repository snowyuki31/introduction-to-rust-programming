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

#### 借用

値の参照を渡す方法を借用という．

immutable な参照を行う場合は，幾つでも参照を渡すことができる．（値がどこから参照されても問題ないため）

```rust
fn main() {
    let x = 5;
    let y = &x;
    let z = &x;

    dbg!(x);
    dbg!(y);
    dbg!(z);
}
```

一方で，mutable な参照をこなう場合は，一つしか渡すことができない．mutable な参照と immutable な参照を共存させることも許されていない．

```rust
fn main() {
    let mut x = 5;
    {
        let y = &mut x; // mutableな参照渡し
        let z = &mut x; // エラー

        dbg!(y);
        dbg!(z);
    }

    {
        let y = &x; // immutableな参照渡し
        let z = &mut x; // エラー

        dbg!(y);
        dbg!(z);
    }
}
```

また，参照は元の所有者のライフタイムより長く生存することはできない，（参照エラーを回避するため）

```rust
fn main() {
    let y;

    {
        let x = 5;
        y = &x;
        dbg!(x);
    } // xのライフタイム終了

    dbg!(y); // xよりもyが長く生存することはできない
}
```

#### ライフタイム決定の流れ

借用チェッカーは，リソースのライフタイムを調べるために，以下の二つのライフタイムに注目している

1. 参照のライフタイム (参照が利用されている期間に対応したライフタイム)
2. 値のライフタイム (値が解放される直前までの期間に対応したライフタイム)

```rust
fn main() {
    let mut x = 5;

    let y = &x;

    let z = &mut x;

    // zのライフタイム終了

    dbg!(y);

    // yのライフタイム終了

    dbg!(x);

    // xのライフタイム終了
    // immutableな参照(y)のライフタイムがある内に，mutableな参照がされているので実際にはエラー
}
```

参照のライフタイムを変えると，

```rust
fn main() {
    let mut x = 5;

    let y = &x;

    // yのライフタイム終了

    let z = &mut x;

    dbg!(z);
    // zのライフタイム終了

    dbg!(x);

    // xのライフタイム終了
}
```

y のライフタイムが終わった後に z の宣言がなされるので，これはコンパイルが通る(1.31 以降)．

Rust 1.31 以前では，借用チェッカーが変数のレキシカルスコープに基づいていた．(レキシカルスコープとは，字句呑みから決定できるスコープで，例えば，let から}まで)

#### RAII

"Resource Aquisition Is Initialization"

変数の初期化時にリソースの確保を行い，変数を破棄するときにリソースの解放を行うということ．

リソースというのは，メモリ以外にも開いているファイルやネットワーク接続も含まれる．

独自の型に保有しているリソースがあり，解放処理が必要な場合は Drop トレイトを用いてリソースの解放をする必要がある．

### スレッド安全性

#### スレッドを作る

```rust
use std::thread;

fn main() {
    thread::spawn(|| {
        println!("Hello, world!");
    });
}
```

これは表示されたりされなかったりする．
これを必ず表示してから終了するには以下のようにする．

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello, world!");
    });

    dbg!(handle.join());
}

```

spawn に渡すものはクロージャなので，変数をスレッドに渡すことができる

```rust
use std::thread;

fn main() {
    let mut handles = Vec::new();

    for x in 0..10 {
        handles.push(thread::spawn(|| {
            println!("Hello, world!: {}", x);
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }
}
```

このような for 文で参照を行うと，x のライフタイムよりもスレッドの生存期間が長くなってしまうので，適切に move を行う必要がある．

すなわち，

```rust
use std::thread;

fn main() {
    let mut handles = Vec::new();

    for x in 0..10 {
        handles.push(thread::spawn(move || {
            println!("Hello, world!: {}", x);
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }
}
```

#### スレッド間の情報共有

マルチスレッドプログラミングの際に，情報共有を行うには

- 共有メモリ
- メッセージパッシング

の二つの方法がある．

#### 共有メモリ

複数のスレッドで同じメモリ領域を共有する方法を見ていく

```rust
// エラーが起こるプログラム
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let mut data = vec![1; 10];

    for x in 0..10 {
        handles.push(thread::spawn(move || {
            data[x] += 1;
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);
}

```

この場合，move によって data の所有権が移動してしまうため，コンパイルエラーとなる．

これを避けるために，Rc という型を用いる．

Rc は所有権を共有するために，所有権を保持している所有者の数をカウントしている（参照カウンタ）.

所有者がゼロになたっところで，メモリを解放する，という仕組み．

```rust
// エラーが起こるプログラム
use std::rc::Rc;
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let mut data = Rc::new(vec![1; 10]);

    for x in 0..10 {
        let data_ref = data.clone(); // Rcの参照カウンタを増やす

        handles.push(thread::spawn(move || {
            data_ref[x] += 1;
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);
}

```

これでもエラーが起こる．Rc がスレッド間を安全に渡せないためである．

マルチスレッドでは Arc を用いる．

```rust
// エラーが起こるプログラム
use std::sync::Arc;
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let mut data = Arc::new(vec![1; 10]);

    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            data_ref[x] += 1;
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);
}
```

今度は，Arc が書き換え不可能なためにエラーが起こる．

排他制御を行うためには Mutex を利用する

```rust
// エラーが起こるプログラム
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let mut data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);
}
```

#### メッセージパッシング

スレッド間通信用のチャンネルを作ることもできる

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let data = rx.recv().unwrap();
        println!("{}", data);
    });

    let _ = tx.send("Hello, world!");
    let _ = handle.join();
}
```

チャンネルはメッセージをキューに蓄えるので，スレッド作成前に send しても問題がない．

双方向的なチャンネルを持つプログラム例は以下．

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let mut data = vec![1; 10];
    let mut snd_channels = Vec::new();
    let mut rcv_channels = Vec::new();

    for _ in 0..10 {
        // mainから各スレッドへのチャンネル
        let (snd_tx, snd_rx) = mpsc::channel();
        // 各スレッドからmainへのチャンネル
        let (rcv_tx, rcv_rx) = mpsc::channel();

        snd_channels.push(snd_tx);
        rcv_channels.push(rcv_rx);

        handles.push(thread::spawn(move || {
            let mut data = snd_rx.recv().unwrap();
            data += 1;
            let _ = rcv_tx.send(data);
        }));
    }

    // 各スレッドにデータの値を送信
    for x in 0..10 {
        let _ = snd_channels[x].send(data[x]);
    }

    // 各スレッドからの結果を受信
    for x in 0..10 {
        data[x] = rcv_channels[x].recv().unwrap();
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);
}
```

#### Send と Sync

Rust にはスレッド安全性を保証する Send と Sync というマーカトレイトが備わっている．

Send は，これを実装した型はその所有権をスレッドを跨いで転送することができることを示す．ほとんどの型に実装されているが，Rc などの一部の型には実装されていない．

Sync は，複数のスレッドから安全にアクセスできることを示す．

自作の型については，基本的には Send と Sync を実装するべきではない．

### 非同期処理

例えば，データベースにアクセスしてデータを受け取りたい際には下記のように書くことができる

```rust
use futures::executor;

struct User {
    // something
}

struct UserId(u32);

struct Db {}

impl Db {
    async fn find_by_user_id(&self, user_id: UserId) -> Option<User> {
        // connection to database
    }
}

async fn find_user_by_id(db: Db, user_id: UserId) -> Option<User> {
    db.find_user_by_id(user_id).await
}

fn main() {
    executor::block_on(find_user_by_id(Db {}, UserId(1)));
}
```

非同期プログラミングは，マルチプロセス・マルチスレッドを用いた並列処理機能の延長線上にある．
Apache HTTP Server では，クライアントからリクエストを受けるとリクエストごとにスレッドを作成し，スレッドごとに処理を行うことで並列処理を実現している．

しかし，同時にサーバーを利用するクライアントが万単位になると応答性能が一気に悪くなるという問題がある（C10K 問題)

この解決方法として，プロセス数やスレッド数は変えず，「必要なタイミングに必要な処理を，開いているリソースに割り当てて実行する」形で
動作するイベント駆動モデルの実装が普及している(nginx, Node.js など)

#### Future

Future とは「将来に値が確定する計算のステートマシンを抽象化したインターフェース」である．

Future トレイトを実装したタスクは「将来的に値が確定する計算」隣，タスクが作成された時点ではまだ実行されておらず，ランタイムに乗った時点で
スケジューリングされて実行される．実行するかどうかの判断は，ポーリングによってチェックされる．

```rust
use futures::{executor, future::join_all};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CountDown(u32);

impl Future for CountDown {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<String> {
        if self.0 == 0 {
            Poll::Ready("Zero!!!".to_string())
        } else {
            println!("{}", self.0);
            self.0 -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn main() {
    let countdown_future1 = CountDown(10);
    let countdown_future2 = CountDown(20);
    let cd_set = join_all(vec![countdown_future1, countdown_future2]);
    let res = executor::block_on(cd_set);

    for (i, s) in res.iter().enumerate() {
        println!("{} : {}", i, s);
    }
}

```

#### async/await

async/await は非同期処理を制御する際に用いられるキーワードで，内部的にはジェネレータとして表現されている．

```rust
use futures::executor;

async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

async fn something_great_async_function() -> i32 {
    let ans = async_add(2, 3).await; // この時点で5という値が取り出せる

    println!("{}", ans);
    ans
}

fn main() {
    executor::block_on(something_great_async_function()); // これがランタイムの起動ポイント
}
```

また，以下のような糖衣構文を用いた表現も可能

```rust
use futures::future::Future;

fn something_great_async_function() -> impl Future<Output = i32> {
    async {
        let ans = async_add(2, 3).await;
        println!("{}", ans);
        ans
    }
}
```

async ブロックや async fn の外で.await を呼び出すとコンパイルエラーとなる．

aysnc 構文または await が複数ある場合は，.await ごとに値を受け取裏，非同期関数内部で.await まで逐一処理を進める．

また，async 構文はその度に.await をしないと中身の評価が走らない．

```rust
async fn print_result(value: i32) {
    println!("{}", value);
}

async fn calculate() -> i32 {
    let add1 = async_add(2, 3).await;
    print_result(add1); // これだと出力されない， println_result(add1).await と評価しなくてはいけない

    let add2 = async_add(2, 3).await;
    print_result(add2); // 同様
    async_add(add1, add2).await
}

```

async ブロックとクロージャには move キーワードを用いることができる

```rust
fn move_to_async_block() -> impl Future<Output = ()> {
    let outside_variable = "this is outside".to_string();

    // moveしないとasyncブロックの前にライフタイムが終わってしまう
    async move {
        println!("{}", outside_variable);
    }
}
```

#### 非同期ランタイム

非同期ランタイムは，非同期計算の実行環境を指す．
非同期ランタイムには現在いくつか種類があり，tokio, async-std が代表的なクレートである．

トレイトに async をつける場合，async-trait といったクレートを用いる必要がある．

### テスト

Rust では，機能のためのコードとそれをテストするコードを同一のファイルに書くことができる．

```rust
pub fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

#[test]
fn test_add() {
    assert_eq!(0, add(0, 0));
    assert_eq!(1, add(0, 1));
    assert_eq!(1, add(1, 0));
    assert_eq!(2, add(1, 1));
}
```

`#[test]`というアトリビュートを追加する．これを実行するには cargo test コマンドを使用する．

アトリビュートには，`#[should_panic]`などさまざまな機能がある．
