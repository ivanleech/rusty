fn stack_fn(mut stack_num: i32){
    stack_num = 6;
    println!("The value of stack_num in function is: {}", stack_num);
}

fn heap_fn(var: &mut Vec<i32>){
    var.push(4);
    println!("The value of heap_string in function is: {:?}", var);
}

fn main(){
    let stack_num: i32 = 5;
    let mut heap_vec: Vec<i32> = vec![1, 2, 3];

    stack_fn(stack_num);
    heap_fn(&mut heap_vec);
    
    println!("The value of stack_num in main is: {}", stack_num);
    println!("The value of heap_vec in main is: {:?}", heap_vec);
}