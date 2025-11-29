use std::io::{self, Write};

fn main() -> io::Result<()> {
    let student_name = vec!["Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunle Gold","Blanca Edemoh",];

    let matric_number = vec!["ACC10211111","ECO10110101","CSC10328828","EEE11020202","MEE10202001",];

     let department = vec!["Accounting","Economics","Computer","Electrical","Mechanical",];

     let level = vec!["300","100","200","200","100",];
   
    let mut file = std::fs::File::create("pau_smis.txt").expect("create failed");


file.write_all("Variations Of Students Information\n".as_bytes()).expect("write failed");

    writeln!(file, "\nStudent_name:").expect("Invaild input");
    for information in student_name {
        writeln!(file, "- {}", information).expect("Failed to process");
    }

    writeln!(file, "\nMatric_number:").expect("Invaild input");
    for information in matric_number {
        writeln!(file, "- {}", information).expect("Failed to process");
    }

    writeln!(file, "\nDepartment:").expect("Invaild input");
    for information in department {
        writeln!(file, "- {}", information).expect("Failed to process");
    }
    writeln!(file, "\nLevel:").expect("Invaild input");
    for information in level {
        writeln!(file, "- {}", information).expect("Failed to process");
    }

    println!("File 'pau_smis.txt' created successfully."); Ok(())
   }  
