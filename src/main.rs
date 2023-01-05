use std::collections::HashMap;
use std::io;


#[derive(Debug)]
pub struct Task {
    name: String,
    status: String
}

pub struct Tasks {
    data: HashMap<String, Task>
}

impl Tasks {
    fn new() -> Self{
        Self {
            data: HashMap::new()
        }
    }

    fn add(&mut self, task: Task) {
        self.data.insert(task.name.to_string(), task);
    }

    fn update(&mut self, name: &str, status: &str) -> bool {
        match self.data.get_mut(name) {
            Some(task) => {
                task.status = status.to_string();
                true
            }
            None => false
        }
    }

    fn remove(&mut self, name: &str) -> bool {
        self.data.remove(name).is_some()
    }

    fn get_all(&self) -> Vec<&Task>{
        self.data.values().collect()
    } 
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again")
    }

    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

mod task_menu {
    use crate::{ get_input, Task, Tasks };
    pub fn add_task(tasks: &mut Tasks) {
        println!("Please enter task name:");
        let name = match get_input() {
            Some(input) => input,
            None => return
        };

        let task = Task {
            name,
            status: "new".to_string()
        };

        tasks.add(task);
        println!("Task added successfully!");
    }

    pub fn update_task(tasks: &mut Tasks) {
        for task in tasks.get_all() {
            println!("{:?}", task);
        }
        println!("Please enter name:");
        let name = match get_input() {
            Some(input) => input,
            None => return
        };
        println!("Please enter status:");
        let status = match get_input() {
            Some(input) => input,
            None => return
        };

        if tasks.update(&name, &status) {
           println!("Task updated successfully!");
        } else {
            println!("Task not found!")
        }
    }

    pub fn remove_task(tasks: &mut Tasks) {
        for task in tasks.get_all() {
            println!("{:?}", task);
        }
        println!("Please enter task name to remove:");
        let name = match get_input() {
            Some(input) => input,
            None => return
        };

        if tasks.remove(&name) {
           println!("Task removed successfully!");
        } else {
            println!("Task not found!")
        }
    }

    pub fn view_tasks(tasks: &Tasks) {
        for task in tasks.get_all() {
            println!("{:?}", task)
        }
    }
}

enum MainMenu {
    Add,
    View,
    Remove,
    Update
}

impl MainMenu {
    fn get_option(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::Add),
            "2" => Some(Self::View),
            "3" => Some(Self::Remove),
            "4" => Some(Self::Update),
            _ => None
        }
    }
    fn show() {
        println!("");
        println!("__Tasks Management __");
        println!("1. Add Task");
        println!("2. View All");
        println!("3. Remove Task");
        println!("4. Update Task");
        println!("");
    }
}


fn task_management() -> Option<()>{
    let mut tasks = Tasks::new();
    loop {
        MainMenu::show();
        let input = get_input()?;
        match MainMenu::get_option(input.as_str()) {
            Some(MainMenu::Add) => task_menu::add_task(&mut tasks),
            Some(MainMenu::View) => task_menu::view_tasks(&tasks),
            Some(MainMenu::Remove) => task_menu::remove_task(&mut tasks),
            Some(MainMenu::Update) => task_menu::update_task(&mut tasks),
            None => break
        }
    }
    None
}
fn main() {
    task_management();
}
