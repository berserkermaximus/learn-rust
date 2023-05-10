fn strings() {

    // fixed strings
    let some_string = "Hello world";
    println!("{}", some_string);

    // variable length string

    let mut mutable_string: String = String::from("Hello Universe");
    println!("{}", mutable_string);

        // adding to string
    mutable_string.push('s');
    println!("{}", mutable_string);

        //popping form string
    mutable_string.pop();
    println!("{}", mutable_string);

        //pushing a whole string
    mutable_string.push_str(" and the whole worlds.");
    println!("{}", mutable_string);

        //string methods
    println!("is string empty: {}", mutable_string.is_empty());
    println!("length of the string: {}", mutable_string.len());
    println!("byte the string consumes: {}", mutable_string.capacity());
    println!("if it contains the word 'use': {}", mutable_string.contains("use"));

    mutable_string.push_str("     ");
    println!("length os string before trim: {}", mutable_string.len());
    println!("length os string after trim: {}", mutable_string.trim().len());


    let i: i32 = 32;
    let i = i.to_string();
    mutable_string.push_str(&i);
    println!("{}", mutable_string);

    let s3 = format!("both strings concatenated are {} {}", some_string, mutable_string);
    println!("{}", s3);
    let s3 = format!("both strings concatenated are {} {}", s3, s3);
    println!("{}", s3);


}