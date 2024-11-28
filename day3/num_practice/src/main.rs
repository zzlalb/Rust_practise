fn main() {
    /*P1*/
    let _x:i32=5;
    let mut _y:u32=5;

    // y=x;
    let _z=10;//Z is i32

    /*P2*/
    let _v:u16=38_u8 as u16;

    /*P3*/
    assert_eq!(i8::MAX,127);
    assert_eq!(u8::MAX,255);

    /*P4*/
    let x4:u32=5;
    assert_eq!("u32".to_string(), type_of(&x4));

    /*P5*/
    let v51=251_u16 + 8_u16;
    let v52 = i16::checked_add(255,8).unwrap();
    println!("{},{}",v51,v52);

    /*P6*/
    let v6=1_024+0xff+0o77+0b1111_1111;
    assert!(v6==1597);

    /*P7*/
    let x7=1_000.000_1;//f64
    let _y7:f32 = 0.12;
    let _z7=0.01_f64;

    println!("{}",type_of(&x7));

    /*P8*/
    assert!(0.1_f32+0.2_f32==0.3_f32);
    

    /*P9*/
    let mut sum=0;
    for i in  -3..2{
        sum+=1;
    }

    assert!(sum== 5);

    for c in 'a'..='z'{
        println!("{}",c as u8);
    }

    /*P10*/
    use std::ops::{Range,RangeInclusive};
    assert_eq!((1..5), Range{start: 1, end: 5});
    assert_eq!((1..=5), RangeInclusive::new(1,5));


    /*P11*/
    assert!(1i32 + 2== 3);
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);
    
    println!("{}",9.6 / 3.0);
    assert!(9.6 / 3.2 != 3.0);

    assert!(24 % 5 == 4);

    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
}


fn type_of<T>(_: &T) ->String{
    format!("{}",std::any::type_name::<T>())
}
