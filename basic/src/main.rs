fn main() {
    // 浮動小数点
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // 足し算
    let sum = 5 + 10;
    // 引き算
    let difference = 95.5 - 4.3;
    // 掛け算
    let product = 4 * 30;
    // 割り算
    let quotient = 56.7 / 32.2;
    // 余り
    let remainder = 43 % 5;

    // 複合型
    let tup: (i32, f64, u8) = (100, 6.4, 1);
    let (a, b, c) = tup;
    println!("The value of a is {}", a);
    println!("The value of tup.1 is {}", tup.1);

    // 配列型
    let l = [1, 2, 3, 4, 5];
    let first = l[0];
    let second = l[1];
    println!("first is {}. second is {}", first, second);

    // 別関数の呼び出し
    another_func(5, 10);

    // 戻り値のある関数呼び出し
    let plus_one_val = plus_one(10);
    println!("plus_one_val is {}", plus_one_val);

    // if文
    let condition = true;
    let number = if condition { 5 } else { 10 };
    println!("The number is {}", number);

    // while文,for文
    let list_2 = [10, 20, 30, 40, 50];
    let mut index = 0;
    println!("=======while文で書いた場合=======");
    while index < 5 {
        // 値は{}です
        println!("the value is: {}", list_2[index]);

        index = index + 1;
    }

    println!("=======for文で書いた場合=======");
    for element in list_2.iter() {
        // 値は{}です
        println!("the value is: {}", element);
    }
}

fn another_func(x: i32, y: i64) {
    println!("x is {},  y is {}", x, y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
