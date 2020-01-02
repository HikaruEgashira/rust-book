fn main() {
    let mut x = 5;
    println!("The value of x is {}", x); // 5
    x = 6;
    println!("The value of x is {}", x); // 6
    // 変数

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x); // 12
    // → シャドーイング

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is {}", spaces); // 3

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is {}", guess); // 42
    // → 型注釈

    let minus: i8 = -3;
    let plus: u8 = 6;
    let float: f64 = 3.14;
    println!("The value of minus is {}, plus is {}, float is {}", minus, plus, float);

    let t = true;
    let f: bool = false;
    println!("{} {}", t, f);

    let c = 'z';
    let z = 'ℤ';
    println!("{} {}", c, z);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{} {} {} {}", tup.1, x, y, z); // 6.4

    let a = [1, 2, 3, 4, 5];
    println!("{}", a[0]);

    // function
    another_function();

    fn another_function() {
        println!("Another function");
    }

    fn five() -> i32 {
        5
    }
    let x = five();
    println!("{}", x);

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("false");
    }

    let number: i32 = if true {
        5
    } else {
        6
    };
    println!("{}", number);

    for elem in a.iter() {
        println!("{}", elem);
    }

    for number in (1..4).rev() {
        println!("{}", number)
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        if a < b {
            return gcd(b, a);
        }
        return gcd(b, a % b);
    }
    println!("{}", gcd(40, 6));

    fn fib(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return 1;
        }
        return fib(n - 1) + fib(n - 2);
    }
    println!("{}", fib(10)); // 89
}
