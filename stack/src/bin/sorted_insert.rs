//Given a stack whose elements are sorted, write a function that will insert elements
//in sorted order. With highest at the top and lowest at the bottom

fn sorted_insert(mut stack: Vec<i32>, element: i32) -> Vec<i32> {
    let mut temp_stack: Vec<i32> = vec![];
    while stack.len() > 0 && stack[stack.len() - 1] > element {
        temp_stack.push(stack.pop().unwrap());
    }

    stack.push(element);

    while temp_stack.len() > 0 {
        stack.push(temp_stack.pop().unwrap());
    }
    
    return stack;
}

fn main() {
    let mut stack: Vec<i32> = vec![8, 16, 28, 36, 48];
    let element: i32 = 30;
    stack = sorted_insert(stack, element);
    println!("result stack: {:?}", stack)
}
