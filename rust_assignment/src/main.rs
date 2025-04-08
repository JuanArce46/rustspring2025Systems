struct Student {
    major: String,
}

fn update_majors(students: &mut [Student], behavior: fn(&mut Student)) {
    for student in students.iter_mut() {
        behavior(student);
    }
}

fn assign_cs(student: &mut Student) {
    student.major = "Computer Science".to_string();
}

fn assign_biology(student: &mut Student) {
    student.major = "Biology".to_string();
}

fn print_majors(students: &[Student]) {
    print!("Majors: ");
    for (i, student) in students.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", student.major);
    }
    println!();
}

fn main() {
    let mut students = vec![
        Student { major: String::new() },
        Student { major: String::new() },
        Student { major: String::new() },
    ];

    print!("Original ");
    print_majors(&students);

    update_majors(&mut students, assign_cs);
    print!("After assigning CS: ");
    print_majors(&students);

    update_majors(&mut students, assign_biology);
    print!("After assigning Biology: ");
    print_majors(&students);
}
