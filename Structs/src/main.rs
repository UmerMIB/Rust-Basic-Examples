/*Create a struct(named student) with 2 fields( roll no, name  ) , impl this struct with 1 method that show
details of struct, create 1 method to change the value  one of the field of struct , 1 associated function 
that return an instance of that structure . Use all these functions and methods in main function*/

struct Student {
roll_no: u32,
name: String,
}

impl Student {
fn show_details(&self) {
println!("The Student Roll no  is {}", self.roll_no);
println!("The Student name is {}", self.name);
}

fn mutate_and_show_details(&mut self, new_name: String) {
self.name = new_name;
println!("The new name for the Roll no {}, is {}", self.roll_no, self.name);
}

fn create_student(roll_no: u32, name: String) -> Student {
Student { roll_no: roll_no, name: name }
}
}

fn main() {
let student = Student { roll_no: 1, name: "Umer".to_string() };
student.show_details();

let student = &mut Student { roll_no: 1, name: "Umer".to_string() };
student.mutate_and_show_details("MiB".to_string());
student.show_details();

let student = Student::create_student(2, "Developer".to_string());
student.show_details();
}
