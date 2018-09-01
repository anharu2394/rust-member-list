extern crate colored;
use std::io;
use colored::*;

struct User {
  name: String,
  age: u32,
}

fn new_user(name: String, age: u32) -> User {
    User { name, age }
}

fn show(users: &Vec<User>) {
    println!("{}","This is users".green().bold());
    println!("{}","-------------".blue().bold());
    for (i,user) in users.into_iter().enumerate() {
        println!("id: {}",i);
        println!("name: {}",user.name);
        println!("age:{}",user.age);
        println!("{}","-------------".blue().bold());
    }
}

fn new(users: &mut Vec<User>) {
    println!("{}","Please type a name!".yellow().bold());
    let mut name = String::new();
    io::stdin().read_line(&mut name)
        .expect("Failed to read line");
    println!("{}","Please type a age".yellow().bold());
    let mut age = String::new();
    io::stdin().read_line(&mut age)
        .expect("Failed to read line");
    users.push(new_user(name.trim().to_string(),age.trim().parse::<u32>().unwrap()));
}

fn delete(users: &mut Vec<User>) {
    println!("{}","Which user do you delete?".white().bold());
    show(users);
    println!("{}","Please type the id which you want to delete".yellow().bold());
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let id: usize = input.trim().parse().unwrap();
    users.remove(id);
}

fn update(users: &mut Vec<User>) {
    println!("{}","Which user do you update?".white().bold());
    show(users);
    println!("{}","Please type the id which you want to update".yellow().bold());
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let id: usize = input.trim().parse().unwrap();
    println!("{}","Please edit the user".yellow().bold());
    println!("{}","Please type a name!".yellow().bold());
    println!("previous name: {}",users[id].name);
    let mut name = String::new();
    io::stdin().read_line(&mut name)
        .expect("Failed to read line");
    println!("{}","Please type a age".yellow().bold());
    println!("previous name: {}",users[id].age);
    let mut age = String::new();
    io::stdin().read_line(&mut age)
        .expect("Failed to read line");
    users.insert(id,new_user(name.trim().to_string(), age.trim().parse::<u32>().unwrap()));
}

fn main() {
    let mut users = Vec::new();
    println!("{}","Hello! This is a name list!\n".red().bold());
    loop {
        println!("{}","Please type a number\n".yellow().bold());
        println!("if you type 0: create new user\n\
                if you type 1: show users\n\
                if you type 2: delete user\n\
                if you type 3: update user\n\
                if you type 4: exit");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let input_number: i32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {println!("{}","Cannot parse");continue},
        };
        match input_number {
            0 => new(&mut users),
            1 => show(&users),
            2 => delete(&mut users),
            3 => update(&mut users),
            4 => {println!("{}","Good Bye!".red().bold());break},
            _ => println!("{}","this is not a number".red().bold())
        }
    }
}
