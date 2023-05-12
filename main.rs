//This is a basic ToDo list Command Line Application in RUST
//We will make three states: To Do, In progress and Done
use std::collections::HashMap;
use std::io::Read;

struct ToDo{
    map: HashMap<String, String>,
}

fn insert(&mut self, key: String){
    self.map.insert(key, String::from("To Do"));
}

fn start(&mut self, key: String) -> Option<()> {
    match self.map.get_mut(key){
        Some(v) => Some(*v = String::from("In Progress")), None => None,
    }
}

fn done(&mut self, key: &String) -> Option(){
    match self.map.get_mut(key){
        Some(v) => Some(*v = String::from("Done")),
        None => None,
    }
}

fn save(self) -> Result<(), std::io::Error>{
    let mut content = String::new();
    for (k,v) in self.map{
        let record = format!(" {} : {} \n", k ,v);
        content.push_str(&record)
    }
    std::fs::write("toDo.db", content)
}

    impl ToDo {
        fn new() -> Result<ToDo, std::io::Error>{
            .write(true)
            .create(true)
            .read(true)
            .open("toDo.db")?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let map: HashMap<String, String> = content
            .lines()
            .map(|line| line.split(" : ").collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k,v)| (String::from(k), String::from(v)))
            .collect();
        ok(ToDo {map})
        }
    }


fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        panic!("Too Less Arguments passed (or Missing Arguments)...Please pass more arguments");
    }
    let action = args[1].clone();
    let task = args[2].clone();
    let mut todo = ToDo::new().expect("Error in this ToDo list creating process");
    if action == "add" {
        ToDo.insert(task);
        match ToDo.save(){
            Ok(_) => println!(" '{}' not present in the ToDo List"),
            Err(e) => println!("Error: {}", e),
        }
    }
    else if action == "start"{
        match ToDo.start(&task){
            None => println!("'{}' not present in ToDo list", task),
            Some(_) => match ToDo.save(){
                Ok(_) => println!("Task Started"),
                Err(e) => println!("Error: {}", e),
            },
        }
    }
    else if action == "done"{
        match ToDo.done(&task){
            None => println!("'{}' not present in ToDo List", task),
            Some(_) => match ToDo.save(){
                Ok(_) => println!("Task Donde Successfully"),
                Err(e) => println!("Error: {}", e),
            },           
        }
    }
}
