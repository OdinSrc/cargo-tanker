use std::{fs::{self, OpenOptions}, io::{Read, self}};
use std::io::prelude::*;


use super::ActionTrait;


pub struct RestApiAction{
    pub target_folder: String,
}


impl RestApiAction{

    /**
     * Create new object
     */
    pub fn new(target_folder: String,) -> Self{
        Self { target_folder }
    }
    
    /**
     * Setup modules folder
     */
    pub fn setup_module_folder(&self, module: &str){
        println!("Creating module folder {module}");
        let module_folder = &format!("{}/{module}", self.target_folder);
        fs::create_dir(module_folder).unwrap_or_default();

        let _ = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("{module_folder}/mod.rs"));
    }

    /**
     * Add modules to main.rs file
     */
    pub fn add_module_to_main(&self, modules: Vec<&str>){
        let file_path = &format!("{}/main.rs", self.target_folder);

        let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_path).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines_to_prepend = String::new();
        modules.into_iter()
        .for_each(|module | {
            let module_declartion = format!("mod {module};\n");
            if !contents.contains(&module_declartion){
                lines_to_prepend += &module_declartion;
            }
        });

        let new_contents = format!("{}{}", lines_to_prepend, contents);
        
        file.seek(io::SeekFrom::Start(0)).unwrap();
        file.write_all(new_contents.as_bytes()).unwrap();
    }
}

impl ActionTrait for RestApiAction {
    fn create_folder(&self) {
        // let target_folder = "./src";
        let modules: Vec<&str> = vec!["models", "repoistory", "routes"];
        println!("Creating REST API folder structure");

        modules.iter()
        .for_each(|module| {
            self.setup_module_folder(module);
        }); 

        self.add_module_to_main(modules);
    }
    
}
