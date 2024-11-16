use std::collections::HashMap;

use crate::syntax::{Request, Target};

pub struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn new() -> Self {
        Company {
            departments: HashMap::new(),
        }
    }
    pub fn handle(&mut self, req: Request) {
        match req {
            Request::Add { subject, to } => {
                let subject = match subject {
                    Target::All => return println!("! subject field cannot be ALL"),
                    Target::As(d) => d,
                };

                let to = match to {
                    Target::All => return println!("! department field cannot be ALL"),
                    Target::As(d) => d,
                };

                let to_dep = self.departments.entry(to.clone()).or_insert(Vec::new());
                to_dep.push(subject.clone());

                println!("{subject} has been added to {to}");
            }
            Request::Remove { subject, from } => {
                let from = match from {
                    Target::All => return println!("! department field cannot be ALL"),
                    Target::As(d) => d,
                };

                let to_dep = self.departments.entry(from.clone()).or_insert(Vec::new());

                match subject {
                    Target::All => match self.departments.remove(&from) {
                        Some(_) => println!("Everyone has been removed from {from}"),
                        None => println!("! no such key"),
                    },
                    Target::As(subject) => {
                        to_dep.retain(|s| s != &subject);
                        println!("{subject} has been removed from {from}");
                    }
                };
            }
            Request::Show { subject } => match subject {
                Target::All => println!("{:#?}", self.departments),
                Target::As(dep) => match self.departments.get(&dep) {
                    None => println!("! no such department"),
                    Some(data) => {
                        let mut data = data.clone();
                        data.sort();
                        println!("{dep} = {data:#?}")
                    }
                },
            },
        }
    }
}
