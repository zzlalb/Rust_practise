fn main() {
    let x=5;
    let y: i32=6;
    println!("x is equal to {}, y is equal to {}",x,y);

    /*Q2*/
    let mut a=1;
    a+=2;
    println!("x={}",a);

    /*Q3*/
    let x:i32 =10;
    {
        let y:i32=5;
        println!("The value of x is {}, the value of y is {}",x,y);
    }
    println!("The value of x is {}, the value of y is {}",x,y);

    /*Q4*/
    define_x();
    println!("{}, world",x);

    /*Q5*/
    let x5:i32=5;
    {
        let x5=12;
        assert_eq!(x5,12);
    }

    assert_eq!(x5,5);

    let x5=42;
    println!("{}",x5);

    /*Q6*/
    // let mut x6:i32=1;
    // x6=7;
    // let mut x6=x6;
    // x6+=3;

    // let  y=4;
    // let y="I can also be bound to text";


    let _x7=1;

    /*Q8*/
    let(mut x8,y8)=(1,2);
    x8+=2;

    assert_eq!(x8,3);
    assert_eq!(y8,2);


    /*Q9*/
    let (x9, y9);
    (x9,..)=(3,4);
    [.., y9]=[1,2];
    
    assert_eq!([x9,y9],[3,2]);
    
}

fn define_x(){
    let _x="hello";
}
