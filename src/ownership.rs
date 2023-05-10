fn main() {
    // ownership
    // - each value in rust has a variable called its owner.
    // - There can be only one owner at a time.
    // - when owner goes out of scope, the value is dropped.
    // copy and move

    let mut x: i32 = 32;
    let mut y: i32 = x;
    // here we "copy" the primitive type
    println!("x = {} and y = {}", x, y); 

    let s1: String = String::from("abc");
    let s2: &String = &s1;
    // let s1: String = String::from("abd");
    // non-primitive will be "moved"
    println!("s1 = {} and s2 = {}", s1, s2);

    let num1: Vec<i32> = vec![5,4,3,2];
    let num2: &Vec<i32> = &num1;

    println!("num1 = {:?} and num2 = {:?}", num1, num2);

    // cloning
    let num2: Vec<i32> = num1.clone();
    println!("num1 = {:?} and num2 = {:?}", num1, num2);

    {   
        // different scope
        let my_name: String = String::from("name");
    }

    println!("my_name = {}", my_name);

    let stack_num: i32 = 32;
    let mut heap_vec: Vec<i32> = vec![4,5,6];

    // As primitive type stored on stack memory, the value is copied and then passed to function.
    println!("original value of stack_num = {}", stack_num);
    stack_function(stack_num);
    println!("copied value of stack_num = {}", stack_num);

    // As non-primitive type is stored on heap memory, the value moves i.e changes ownership.
    println!("Original value of heap_vec = {:?}", heap_vec);
    heap_function(&mut heap_vec);
    println!("value after function call of heap_vec = {:?}", heap_vec);

}


fn stack_function(mut var: i32){
    var = 56;
    println!("value of copied variable = {}", var);
}

fn heap_function(var: &mut Vec<i32>){
    var.push(65);
    println!("value of vector inside the function = {:?}", var);
}