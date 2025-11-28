use std::io::{self, Write};

fn main() -> io::Result<()> {
    let  lager = vec!["Star","Heineken","Gulder","Goldberg","Desperados","33 Export",];

    let  stout = vec!["Legend","Turbo King","Williams",];

     let  non_alcoholic = vec!["Maltina","Amstel Malta","Malta Gold","Fayrouz",];
   
    let mut file = std::fs::File::create("drinks.txt").expect("create failed");


file.write_all("Categories Of Drinks\n".as_bytes()).expect("write failed");

    writeln!(file, "\nLager:")?;
    for drink in lager {
        writeln!(file, "- {}", drink)?;
    }

    writeln!(file, "\nStout:")?;
    for drink in stout {
        writeln!(file, "- {}", drink)?;
    }

    writeln!(file, "\nNon-Alcoholic:")?;
    for drink in non_alcoholic {
        writeln!(file, "- {}", drink)?;
    }

    println!("File 'drinks.txt' created successfully."); Ok(())
   }  
