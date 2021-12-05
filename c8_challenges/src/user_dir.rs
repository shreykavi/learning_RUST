// Using a hash map and vectors, create a text interface to allow a user to add 
// employee names to a department in a company. For example, “Add Sally to Engineering” 
// or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

use std::io;
use std::collections::HashMap;
use wildmatch::WildMatch;

struct Company {
    directory: HashMap<String, Vec<String>>
}

impl Company {
    fn new() -> Company{
        Company {
            directory: HashMap::new()
        }
    }
    fn add_person(& mut self, employee_name: &str, department: &str) {
        self.directory
            .entry(department.to_string())
            .or_insert(Vec::new())
            .push(employee_name.to_string());
    }

    fn list_department(& self, department: &str){
        match self.directory.get(department) {
            Some(i) => {
                let mut array = i.clone();
                // Sort to be alphabetic
                array.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
                println!("{:?}", array);
            },
            None => println!("This department doesnt exist"),
            _ => (),
        }
    }
    fn list_company(&self){
        for (department, value) in &self.directory {
            println!("Department: {}", department);
            Company::list_department(&self, department);
        };
    }
}

pub fn run(){
    let mut company = Company::new();

    loop {
        println!("\nPlease enter a command of the following formats for the command you would like to run:");
        println!(" - add (employee_name) to (department)");
        println!(" - list company");
        println!(" - list (department)");

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        command = command.trim().to_lowercase();
            
        if command == "list company"{
            println!("Heres the whole company:");
            company.list_company();
        } else if WildMatch::new("list *").matches(&command) {
            let split_command: Vec<&str> = command.split_whitespace().collect();
            if split_command.len() != 2{
                println!("Whats that after the department??");
                continue
            };

            println!("Heres all the employees in {}", split_command[1]);
            company.list_department(split_command[1]);
        } else if WildMatch::new("add * to *").matches(&command) {
            let split_command: Vec<&str> = command.split_whitespace().collect();
            if split_command.len() != 4{
                println!("Whats this rubbish ?!?");
                continue
            };

            let employee_name = split_command[1];
            let department = split_command[3];

            println!("Adding employee: {}, department: {}", employee_name, department);
            company.add_person(employee_name, department);
        } else {
            println!("Unknown command..");
        }
    }
}
    