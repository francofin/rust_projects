mod list{

    pub struct Task{
        pub item: String,
    }


    pub mod items_completed{
        pub fn remove_task(){}
        pub fn move_back_todo(){}
    }
}


//file must be calles the same
mod things_todo;
use crate::things_todo::add_activity;
use crate::things_todo::update_activity;
use things_todo::items_potential;
use things_todo::items_potential::test::test;


fn add_task(){
    let task = list::Task{item: String::from("New Task")};
    add_activity(); 
    update_activity();
    items_potential.add_activity(); // from a folder module. This is a nested module from a folder. This is a sub module of the main module but in a sub directory
    test();
    list::items_completed.move_back_todo() // Relative Path
    crate::list::items_completed.remove_task() //Absolute path, start at root crate designated by crate keyword.
}



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
