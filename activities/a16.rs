// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {

    let student1 = Student {
        name: "Pedro de la Vega".to_owned(),
        locker: None
    };

    let student2 = Student {
        name: "Guybrush Threepwood".to_owned(),
        locker: Some(1234),
    };

    let students = vec![student1, student2];

    for student in students {
        match student.locker {
            Some(number) => println!("Your locker number is: {number}"),
            None => println!("You don't have a locker assigned yet"),
        }
    };

}
