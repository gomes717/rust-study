
fn main() {
    let x = ["val1", "val2"];
    println!("{} {}", x[0], x[1]);
    println!("size of x: {}", x.len());
    let x2 = [1.; 100]; //creates an array with 100 elements with the initial value equal 1
    println!("{}", x2[27]);

    let mut fib = [1; 15];
    for i in 2..fib.len() 
    {
        fib[i] = fib[i - 2] + fib[i - 1];
    }
    for i in 0..fib.len() 
    {
        print!("{}, ", fib[i]);
    }
    println!();

    let mut x = [[[[23; 4]; 6]; 8]; 15]; // multidimensional array
    x[14][7][5][3] = 56;
    println!("{}, {}", x[0][0][0][0], x[14][7][5][3]);

    let mut x3 = vec![1,2,3,4,5];
    println!("{} {} {}", x3[1], x3[2], x3.len());
    x3.push(6);
    println!("{}", x3[5]);
    x3.insert(2, 4);
    println!("{}", x3[2]);
    x3.remove(0);
    println!("{}", x3[0]);

    println!("{:?} {:?}", [1, 2, 3], vec![4, 5]); // a way for print the content of a array and vector
}
