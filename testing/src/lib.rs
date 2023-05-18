pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn failing_func(){
    panic!("Test Failed");
}

//We can use the #[ignore] to ignore certain tests. The #[should_panic ] helps catch panics so will assert and pass if a panic is thrown. 
// We can run a subset of test using cargo test call_simple_add for example
//for a subset of test we can call cargo test it for example to call test starting with it. 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn call_simple_add(){
        assert!(simple_add());
    }

    #[test]
    fn it_works_nq() {
        let result = add(2, 2);
        assert_ne!(result, 5);
    }
}

pub fn simple_add() -> bool{
    if 2+2 ==4{
        true
    } else{
        false
    }
}