pub fn run() {
    // variables();
    // datatypes();
    // let z = another_function(3, "Yo");
    // let func = function_as_argument();
    // println!("{}", z);
    // func(32, "Bye", another_function);
    control_flow();
}

fn variables() {
    let x = 5;
    const A: i32 = 4;
    println!("{}", x);
    println!("{}", A);

    shadowing();
}

fn shadowing() {
    let x = 3;
    println!("{}", x);
    let x = "i am a string literal";
    println!("Before block, after change: {}", x);
    {
        let x = [1, 2, 3];
        println!("{:?}", x);
    }
    println!("After: {}", x);
}

fn datatypes() {
    // Scalar types
    let guess: u32 = "42".parse().expect("Cannot convert to a number!!");
    println!("{} is a type of 'u32'", guess);

    // Compund types
    let tup = ("Hello", 21, 45.0);
    // let tup = (String::from("Hello"), 21, 45.0); // This wil transfer the string when to extract data from the tuple
    let (x, _, _) = tup;
    let y = tup.1;
    println!("{}", x);
    println!("{}", y);
    println!("{:?}", tup);

    let arr = [0; 5];
    println!("{:?}", arr);
    let arr = [1, 2, 3, 4];
    println!("{:?}", arr);
    let first = arr[0];
    let second = arr[1];
    println!("{}", first);
    println!("{}", second);
}

fn another_function(x: i32, y: &str) -> i32 {
    println!("{}", x);
    println!("{}", y);
    let z = {
        let a = x + 3;
        a + 5 // Expression in the end of statement without ';' is equal to 'return [Expr]'
    };
    println!("{}", z);
    z
}

fn function_as_argument() -> fn(i32, &str, fn(i32, &str) -> i32) {
    fn wrapper(x: i32, y: &str, f: fn(i32, &str) -> i32) {
        println!("\nInside 'function_as_argument'");
        let num = f(x, y);
        println!("End: {}", num)
    }
    wrapper
}

fn control_flow() {
    println!("\nUsing loop{{}}. Instead of While True-loop");
    let mut i = 0;
    loop {
        i += 1;
        if i == 10 {
            println!("10");
            break;
        }
    }

    println!("\nIterate over array: ");
    let arr = ["Hello", "Something", "HAHA", "Bye"];
    for number in arr {
        println!("{}", number);
    }

    println!("\nRange in for loop: ");
    for i in 1..=10 {
        println!("{}", i);
    }

    println!("\nWhile-loop: ");
    let mut x = 5;
    while x <= 8 {
        println!("{}", x);
        x += 1;
    }
}
