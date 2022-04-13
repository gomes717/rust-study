

fn main() {
    let number1 = 10; // same as const int number1
    let mut number2 = 33; //same as int number2
    println!("num1 = {}, num2 = {}", number1, number2);
    number2 = 13;
    println!("num1 = {}, num2 mut = {}", number1, number2);
    let _number3 = 1; // the "_" silence the warning for variables not used.
    let flut = 1.;
    println!("ponto flutuante = {}", flut);

    //re-declaration
    let x = 120;
    print!("{} ", x);
    let x = "abcd";
    print!("{} ", x);
    let mut x = true;
    print!("{} ", x);
    x = false;
    println!("{}", x);

    //assignment
    let mut a = 12;
    a += 1;
    a -= 4;
    a *= 7;
    a /= 6;
    println!("{}", a);

    //std library (don't need include)
    println!("{} {}", str::len("abcde"), "abcde".len());
    
    //Control statements
    let n = 2;
    if n > 0 
    {
        println!("number is positive");
    }
    else 
    {
        println!("number is non positive");
    }

    // expression conditional
    let n = 4;
    println!("{}",
        if n > 1000 
        {
            "big"
        }
        else if n > 0 
        {
            "small"
        }
        else if n < 0 
        {
            "negative"
        }
        else 
        {
            "neither positive nor negative"
        }
    );

    //Loop
    let mut i = 1;
    while i <= 10 
    {
        print!("{} ", i * i);
        i += 1;
    }
    println!();

    //infinite loop
    let mut i = 1;
    loop // same as while true in C
    {
        let ii = i * i;
        if ii >= 200 { break; }
        print!("{} ", ii);
        i += 1;
    }
    println!();

    //for loop (for(int i = 1; i < 11; i++))
    for i in 1..11 
    {
        print!("{} ", i * i);
    }
    println!();

    //variable in parameters
    let mut limit = 4;
    for i in 1..limit 
    {
        limit -= 1; //altering the value doesnt affect the number of interation
        print!("{} ", i);
    }
    println!(":{}", limit);

    //scopes
    let mut _i = 1;
    if true { let _i = 2; /*creates a new _i and destroy*/ }
    print!("{} ", _i);
    while _i > 0 { _i -= 1; let _i = 5; /*creates a new _i and destroy every iteration*/ }
    print!("{} ", _i);
}
