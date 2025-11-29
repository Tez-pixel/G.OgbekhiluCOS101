use std::io::{self, Write};

fn main() -> io::Result<()> {
    let name_of_commisioner = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye",];

    let ministry = vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum",];

     let geopolitical_zone = vec!["South West","North East","South South","South West","South East",];

   
    let mut file = std::fs::File::create("efcc.txt").expect("create failed");


file.write_all("Convicted Commisioners\n".as_bytes()).expect("write failed");

    writeln!(file, "\nName_of_commisioner:").expect("Invaild input");
    for conviction in name_of_commisioner {
        writeln!(file, "- {}", conviction).expect("Failed to process");
    }

    writeln!(file, "\nMinistry:").expect("Invaild input");
    for conviction in ministry {
        writeln!(file, "- {}", conviction).expect("Failed to process");
    }

    writeln!(file, "\nGeopolitical_zone:").expect("Invaild input");
    for conviction in geopolitical_zone {
        writeln!(file, "- {}", conviction).expect("Failed to process");
    }
    

    println!("File 'efcc.txt' created successfully."); Ok(())
   }  
