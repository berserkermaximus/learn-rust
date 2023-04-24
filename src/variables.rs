fn variables() {
    println!("Hello, world!");
    println!("{}", std::i64::MAX);
    println!("{}", std::u64::MAX);
    println!("{}", std::i64::MIN);
    println!("{}", std::u64::MIN);
    println!("{}", std::f64::MAX);
    println!("{}", std::f32::MAX);
    println!("{}", std::f64::MIN);
    println!("{}", std::f32::MIN);


    // pretty print
    println!("{:#?}", (32,54,65));

    // type-casting
    let x:i64 = 15;
    println!("{}", x + 1);
    let z:i32 = x as i32 + 18.4 as i32;
    println!("{}", z);

    //shawdowing
    let x:i64 = 15;
    println!("{}", x + 1);
    let x:i64 = 23;
    println!("{}", x + 1);

        // scope shadowing
    let mut s: i32 = 60;
    {
        s = 3244;
        println!("value of s in inner scope is {}", s);
    }

    println!("value of s in outer scope is {}", s);

    // constants
    const PI: f32 = 3.14;
    println!("the circumference of circle with radius {} is {}.", 5, 2 as f32 * PI * 5 as f32)

}
