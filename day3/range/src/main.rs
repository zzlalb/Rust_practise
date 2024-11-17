use num::complex::Complex;

fn main() {
    for i in 1..5{
        print!("{}",i);
    }

    print!("\n");

    for i in 1..=5{
        println!("{}",i);
    }

    println!("---------------------------------");

    let a=Complex{ re: 2.1, im: -1.2 };
    let b=Complex::new(11.1, 22.2);
    let result=a + b;

    println!("{} + {}i",result.re,result.im);

    println!("Hello, world!")
}
