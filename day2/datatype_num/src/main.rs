fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

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

    let _x1=2.0;
    let _y1:f32=3.0;//Default is f64

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
    //assert!(xyz.0+xyz.1==xyz.2);   // will not pass

    println!("------------------------------------");

    /*NaN*/
    let x2=(-42.0_f32).sqrt();//  _f32 is totally same as f32
    //assert_eq!(x2,x2);  // will not pass.  because all interact with NaN will return NaN

    let x3=(-42.0f32).sqrt();
    if x3.is_nan(){
        println!("undefined mathmatical behaviour");
    }

    println!("--------------------------------------");
    //mathmatical operation
    let _sum=5+10;
    let _difference=95.5-4.3;
    let _product= 4*30;
    let _quotient=56.7/32.2;
    let _remainder=43%5;

    let twenty=20i32;     // if change to i64 there will be error
    let twenty_one:i32=21;// because only same type can operate
    let twenty_two=22_i32;
    let addition=twenty+twenty_one+twenty_two;
    println!("{}+{}+{}={}",twenty,twenty_one,twenty_two,addition);


    // for long num, can use _ to seperate them
    let one_million:i64=1_000_000;
    println!("{}",one_million.pow(2));

    let forty_twos=[
        42.0,
        42f32,
        42.0_f32,
    ];
    println!("{:.2}",forty_twos[0]);
    print_type_of(&forty_twos[0]);
    
}
