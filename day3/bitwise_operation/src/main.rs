fn main() {
    let a:u8=2;

    let b:u8=0b_0011;
    println!("a value is       {:08b}",a);

    println!("a value is       {:08b}",b);

    println!("(a & b) value is {:08b}",a&b);

    println!("(a | b) value is {:08b}",a|b);

    println!("(a ^ b) value is {:08b}",a^b);

    println!("(!b) value is    {:08b}",!b);

    println!("(a << b) value is {:08b}",a<<b);

    println!("(a >> b) value is {:08b}",a>>b);

    let mut a=a;
    a<<=b;
    println!("(a << b) value is {:08b}",a);

}
