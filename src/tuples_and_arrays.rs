fn main() {
    /*Tuples */
    let info: (i32, i64, &str) = (32, 100000_000, "Salary");
    println!("{:?}", info);
    println!("{}", info.1 > 54);
    println!("{}", info.2 < "54");

    // Deconstructing the tuple.
    let name: (&str, &str) = ("First", "Last");
    let (first_name, last_name) = name;
    println!("{} {}", first_name, last_name);

    // nesting tuples
    let nested_tuples: (i32, f64, (&str, i32, (&str, &str)), &str) = (4, 5.0, ("First", 8_00, ("Hello", "World")), "Salary");
    println!("{} {} {}", nested_tuples.1, nested_tuples.2.1, nested_tuples.2.2.1);
    println!("{:#?}", nested_tuples);

    // Arrays.
    let mut a: [i32; 6] = [1,2,3,4,5, 6];
    println!("{:#?}", a);
    println!("{}", a[0]);
    a[3] = 9;
    println!("{:?}", a);

    // Array with same element
    let zero_array: [i32; 10] = [0;10];
    println!("{:?}", zero_array);

    // slices
    let b = &a[2..4];
    println!("{:?}", b);

    let b = &a[2..=4];
    println!("{:?}", b);

    // functions
    println!("length of array is {}", a.len());
    println!("size of the array is {} bytes.", std::mem::size_of_val(&a));
    println!("{:?}", a.get(3));
    println!("{:?}", a.get(1000));

    let c: [[u8; 4]; 12] = [[1, 2, 3, 4]; 12];
    println!("{:?}", c);
    println!("{}", c[0][2]);
    println!("{}", std::mem::size_of_val(&c))
}