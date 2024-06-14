fn sort_stack(mut stack: Vec<i32>) -> Vec<i32> {
    let mut temp_stack: Vec<i32> = vec![];
    let mut popped: i32;
    while stack.len() > 0 {
        popped = stack.pop().unwrap(); 
        while temp_stack.len() > 0 && *temp_stack.last().unwrap() < popped {
            stack.push(temp_stack.pop().unwrap());
        }                      
        temp_stack.push(popped);  
    }
    
    while temp_stack.len() > 0 {
        stack.push(temp_stack.pop().unwrap());
    }

    return stack;
}

fn main() {
    let mut stack: Vec<i32> = vec![26, 60, 36, 106, 6, 76];
    stack = sort_stack(stack);
    println!("result stack: {:?}", stack)
}