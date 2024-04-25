
fn new_stack(maxsize: usize) -> Vec<u32>{
    let vec: Vec<u32> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let popped_val: Option<u32> = stack.pop();
    println!("Popped value: {:?}", popped_val);
    popped_val
}

fn push(stack: &mut Vec<u32>, item: u32, maxsize: usize){
    if stack.len() == maxsize{
        println!("Cannot add more")
    } else {
        stack.push(item);
        println!("Stack {:?}", stack);
    }
}

fn size(stack: &Vec<u32>) -> usize{
    stack.len()
}

fn input() -> u32 {
    let mut n: String = String::new();
    std::io::stdin().read_line(&mut n).expect("failed to read input");
    let n = n.trim().parse().expect("invalid input");
    n
}

fn main(){
    println!("Enter the size of the stack");
    let size_stack: u32 = input();

    let mut stack = new_stack(size_stack as usize);

    loop {
        println!("1. Push");
        println!("2. Pop");
        println!("3. Size");
        println!("4. Exit");

        let choice: u32 = input();

        match choice {
            1 => {
                println!("Enter the value to push");
                let item: u32 = input();
                push(&mut stack, item, size_stack as usize);
            }
            2 => {
                pop(&mut stack);
            }
            3 => {
                println!("Size of the stack: {:?}", size(&stack));
            }
            4 => {
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }


    }
}
