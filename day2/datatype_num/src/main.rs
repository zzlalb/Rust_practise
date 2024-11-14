fn main() {
    let x:u8=255;
    //let guess:i32="42".parse().expect("hellp");
    println!("255u8.wrapping_add(10)={}",x.wrapping_add(10));
    println!("{}",100u8);

    println!("----------------------");

    println!("255u8.saturating_add(1)={}",255u8.saturating_add(1));
    assert_eq!(u8::MAX.saturating_add(1),u8::MAX);


    println!("------------------------------------");
    println!("Next is Floating type");

    println!("\n");

    let x1=2.0;
    let y1:f32=3.0;//Default is f64

    assert!((0.1f64+0.2-0.3).abs()<0.00001);

    let abc:(f32,f32,f32)=(0.1,0.2,0.3);
    let xyz:(f64,f64,f64)=(0.1,0.2,0.3);

    println!("abc (f32)");
    println!("  0.1+0.2:{:x}",(abc.0+abc.1).to_bits());//Rust use .num to catch tuble ele
    println!("      0.3:{:x}",(abc.2).to_bits());
    println!();


    println!("xyz (f64)");
    println!("  0.1+0.2: {:x}",(xyz.0+xyz.1).to_bits());
    println!("      0.3: {:x}",(xyz.2).to_bits());
    println!();

    assert!(abc.0+abc.1==abc.2);
    assert!(xyz.0+xyz.1==xyz.2);

}
