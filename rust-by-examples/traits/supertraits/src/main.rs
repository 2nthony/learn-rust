trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

struct StudentData {
    name: String,
    university: String,
    fav_language: String,
    git_username: String,
}

// TODO: how
// impl dyn CompSciStudent for StudentData {
//     fn git_username(&self) -> String {
//         "rustacean".to_string()
//     }

//     fn name(&self) -> String {
//         "Rustacean".to_string()
//     }
// }

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {} and my git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn main() {}
