


fn main()
{
    //basic representation of differents types
    let hexadecimal = 0x10;
    let decimal = 10;
    let octal = 0o10;
    let binary = 0b10;
    println!("{} {} {} {}", hexadecimal, decimal, octal, binary);

    //representation of numbers using "_" as a separator
    let hexadecimal = 0x_00FF_F7A3;
    let decimal = 1_234_567;
    let octal = 0o_777_205_162;
    let binary = 0b_0110_1001_1111_0001;
    println!("{} {} {} {}", hexadecimal, decimal, octal, binary);

    //representation of nunbers using exponential
    let one_thousand = 1e3;
    let one_million = 1e6;
    let thirteen_billions_and_half = 13.5e9;
    let twelve_millionths = 12e-6;
    println!("{} {} {} {}", one_thousand, one_million, thirteen_billions_and_half, twelve_millionths);

    //explicit type
    let a: i8 = 5;          //char
    let b: i16 = 5;         //short
    let c: i32 = 5;         //int
    let d: i64 = 5;         //long
    println!("{} {} {} {}", a, b, c, d);

    //print!("{}", a + b);  this generates a error because a is i8 and b is i16
    
    let a: u8 = 5;          //unsigned char
    let b: u16 = 5;         //unsigned short
    let c: u32 = 5;         //unsigned int
    let d: u64 = 5;         //unsigned long
    println!("{} {} {} {}", a, b, c, d);


    let arr = [11, 22, 33];
    let i: usize = 2;       //usize is better for cross compilation
    println!("{}", arr[i]);

    let a: f64 = 4.6;       //double
    let b: f32 = 3.91;      //float
    println!("{} {}", a, b);

    //explicit conversion
    let a: i16 = 12;
    let b: u32 = 4;
    let c: f32 = 3.7;
    println!("{}", a as i8 + b as i8 + c as i8);

    let a: bool = true; print!("[{}]", a);
    let b: char = 'a'; println!("[{}]", b);

    //rust doesnt allow math using char and bool ('a' + 'b'), but using a explicit conversion this types can be numbers
    println!("{} {} {} {} {}", true as u8, false as u8, 'A' as u32, 'à' as u32, '€' as u32);

    //array and vector using explicit types
    let _array1: [char; 3] = ['x', 'y', 'z'];
    let _array2: [f32; 200] = [0f32; 200];
    let _vector1: Vec<char> = vec!['x', 'y', 'z'];
    let _vector2: Vec<i32> = vec![0; 5000];
    
    //using const to create an array
    const N: usize = 20;
    let _ = [0; N];

    //using enum
    enum Continent {
        Europe,
        Asia,
        Africa,
        America,
        Oceania,
    }

    let contin = Continent::Asia;
    //match is like switch-case
    match contin {
        Continent::Europe => println!("E"),
        Continent::Asia => println!("As"),
        Continent::Africa => println!("Af"),
        Continent::America => println!("Am"),
        Continent::Oceania => println!("O"),
    }

    enum CardinalPoint { North, South, West, East };
    let direction = CardinalPoint::South;
    match direction {
        CardinalPoint::North => println!("NORTH"),
        CardinalPoint::South => println!("SOUTH"),
        _ => {},                                     //default
    }

    //enum with data
    enum Result {
        Success(f64),
        Failure(u16, char),
        Uncertainty,
       }
    let outcome = Result::Success(23.67);
       //let outcome = Result::Failure(1200, 'X');

       match outcome {
        Result::Success(value) =>
            println!("Result: {}", value),
        Result::Failure(error_code, module) =>
            println!("Error n. {} in module {}",
        error_code, module),
        Result::Uncertainty => {},
       }
    
    //match with guards
    for n in -2..5 {
        println!("{} is {}.", n, match n {
        0 => "zero",
        1 => "one",
        _ if n < 0 => "negative", 
        _ => "plural",
        });
       }
}
