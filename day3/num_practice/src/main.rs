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



}

fn type_of<T>(_: &T) ->String{
    format!("{}",std::any::type_name::<T>())
}
