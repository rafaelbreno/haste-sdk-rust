pub mod models {
    #[derive(PartialEq,Debug)]
    pub struct Arcade {
        pub id: String,
        pub name: String,
        pub description: String,
    }

    pub fn new_arcade(id: String) -> Arcade {
       Arcade {
           id, 
           name: String::from(""), 
           description: String::from(""),
       } 
    }

    pub enum HasteEnvironment {
        Production,
        NonProduction,
    }
}
