
use snake_game::cool_module::{Person, PersonId, Log, Animal};



fn main() {
    Person::some_function();

    let _person =  Person {
       name: "Filip".to_string(),
       email: "filip.jerga@hotbox.com".to_string(),
       age: 30,
       id: PersonId::Passport(643),
    };

    let _person_2 =  Person {
        name: "John".to_string(),
        email: "john.snow@hotbox.com".to_string(),
        age: 35,
        id: PersonId::Passport(345),
     };
    let mut person = Person::new();
    let person_2 = Person::from(
        String::from("John"),
        String::from("Snow"),
        35,
        PersonId::Passport(234),
    );

  

    person_2.display_info();
    person_2.alert_something();

    // log_info(person_2);
    log_info_2(&person_2);
    log_info_3(&person_2);


}


fn log_info(val: impl Log) {
    val.display_info();
}

fn log_info_2(val: &dyn Log) {
    val.display_info();
}

fn log_info_3(val: &impl Log) {
    val.display_info();
}

// fn print_message (mut a: String ) -> String {
//     a.push_str(" Yo Yo yo Yo!");
//     a
// }

// fn extend_age (mut a: u32) {
//     a += 100;
//     println!("{}", a);
// }