pub mod cool_module {

    // mod top_level {
    //     fn hi_there() {
    //         println!("Hi There!");
    //     }

    //     mod low_level {
    //         fn hello_world() {
    //             println!("Hello World");
    //         }
            
    //     }

    // }
    
    pub trait Log {
        fn display_info(&self);
        fn alert_something(&self) {
            println!("Alert something");
        }
    }
    pub struct Animal(pub String);

    #[derive(Debug)]
    pub enum PersonId {
        Passport(u32),
        IdentityCard(u32, u32, u32),
    }


    #[derive(Debug)]
    pub struct Person {
        pub name: String,
        pub email: String,
        pub age: u32,
        pub id: PersonId,
    }

    impl Log for Animal {
        fn display_info(&self) {
            println!("I'm an animal");
        }
    }

    impl Log for Person {
        fn display_info(&self) {
            println!("{} {} {}", self.name, self.email, self.age);
        }

        fn alert_something(&self) {
            println!("Override alert-something");
        }
    }

    impl Person {

        pub fn some_function() {
            println!("some_function");
        }

        pub fn new() -> Person {
            Person {
                name: "Default".to_string(),
                age: 0,
                email: "Default".to_string(),
                id: PersonId::Passport(123),
            }
        }

        // method
        // first parameter is always self, which represents the instance of the struct the
        // method is being called on
        // Within an impl block, the type Self is an alias for the current type
        pub fn display_age(&self) {
        println!("Current Age: {}", self.age);
        }

        pub fn from(name: String, email: String, age: u32, id: PersonId) -> Person {
            Person {
                name,
                email,
                age,
                id,
            }
        }

        pub fn change_age(&mut self, new_age: u32) {
        self.age = new_age;
        }
    }

} 