use std::io;
use std::process::Command;
mod  user;
mod display_top;
mod fitness;

static mut  CURRENT_USER:String= String::new();
static mut  CURRENT_USER_SCORE:i32=0;
static mut CURRENT_USER_RANK:String= String::new();
fn main() {
    //Command na vstup do appky
    let mut step = String::new();
loop{
    loop {
        let output = Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear terminal");

            if !output.success() {
                eprintln!("Failed to clear terminal");
            }

        println!("1: Login\n2: Register\n3: Exit\n\nEnter command: ");
        step.clear();
        io::stdin().read_line(&mut step).expect("Failed to read command.");

        if step.trim().is_empty(){
            println!("Failed to read input.");
        }
        else if step.trim()=="3"{
            let output = Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear terminal");

            if !output.success() {
                eprintln!("Failed to clear terminal");
            }

            return;
        }
        else if step.trim()=="2"{
            let output = Command::new("cmd")
                    .args(&["/C", "cls"])
                    .status()
                    .expect("Failed to clear terminal");

                    if !output.success() {
                        eprintln!("Failed to clear terminal");
                    }
             unsafe {
                CURRENT_USER = user::register();
             }
            break;
        }
        else if step.trim()=="1"{
            let output = Command::new("cmd")
                    .args(&["/C", "cls"])
                    .status()
                    .expect("Failed to clear terminal");

                    if !output.success() {
                        eprintln!("Failed to clear terminal");
                    }
                    unsafe{
                        CURRENT_USER = user::login();
                    }
             
            break;
        }
        else {
            println!("Invalid input!");
        }
    }

    //Command pre pohyb v appke
    let mut app_command = String::new();

    loop{
        unsafe{
        CURRENT_USER_SCORE = user::get_score(&CURRENT_USER.trim());
        CURRENT_USER_RANK = user::get_rank(CURRENT_USER_SCORE); 
        }           
        let output = Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear terminal");

            if !output.success() {
                eprintln!("Failed to clear terminal");
            }

        println!("What would you like to do?
        1: Check your profile       2: Rate
        3: Inspect fitness          4: Display best fitness centres
        5: Add a fitness centre     6: Diplay best Raters
        7: Exit                     e: log out");
        println!("Enter command:");
        
        app_command.clear();
        io::stdin().read_line(&mut app_command).expect("Failed to read command.");
        
        if app_command.trim().is_empty(){
            println!("Failed to read input.");
        }
        else if app_command.trim()=="1"{
            let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                        if !output.success() {
                              eprintln!("Failed to clear terminal");
                        }
            unsafe{
            println!("Name: {}Score: {}\nRank: {}",CURRENT_USER, CURRENT_USER_SCORE, CURRENT_USER_RANK);
            }
            let mut back = String::new();

            loop {
                println!("\nPress 'e' to go back");
                back.clear();
                io::stdin().read_line(&mut back).expect("Failed to read command.");
        
                if step.trim().is_empty(){
                    println!("Failed to read input.");
                }
                else if back.trim()=="e"{
                    break;
                }

                let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                         if !output.success() {
                              eprintln!("Failed to clear terminal");
                                     }
                unsafe{
                println!("Name: {}Score: {}",CURRENT_USER, CURRENT_USER_SCORE);
                }
            }
        }
        else if app_command.trim()=="2"{
            let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                        if !output.success() {
                              eprintln!("Failed to clear terminal");
                        }
            
            fitness::rate_fittnes();
            unsafe{
            user::score_update(CURRENT_USER.trim().to_string().clone());
            }
               }
        else if app_command.trim()=="6"{
            let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                        if !output.success() {
                              eprintln!("Failed to clear terminal");
                        }
            display_top::top_rates();
            let mut back = String::new();

            loop {
                println!("\nPress 'e' to go back");
                back.clear();
                io::stdin().read_line(&mut back).expect("Failed to read command.");
        
                if step.trim().is_empty(){
                    println!("Failed to read input.");
                }
                else if back.trim()=="e"{
                    break;
                }

                let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                        if !output.success() {
                              eprintln!("Failed to clear terminal");
                        }
                display_top::top_rates();
            }
        }
        else if app_command.trim()=="4"{
            let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                        if !output.success() {
                              eprintln!("Failed to clear terminal");
                        }
            display_top::top_fitness();
            let mut back = String::new();

            loop {
                println!("\nPress 'e' to go back");
                back.clear();
                io::stdin().read_line(&mut back).expect("Failed to read command.");
        
                if step.trim().is_empty(){
                    println!("Failed to read input.");
                }
                else if back.trim()=="e"{
                    break;
                }

                let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                        if !output.success() {
                              eprintln!("Failed to clear terminal");
                        }
                display_top::top_fitness();
            }
            
        }
        else if app_command.trim()=="5"{
            let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                        if !output.success() {
                              eprintln!("Failed to clear terminal");
                        }
            fitness::add_fit();
            let mut back = String::new();

            loop {
                println!("\nPress 'e' to go back");
                back.clear();
                io::stdin().read_line(&mut back).expect("Failed to read command.");
        
                if step.trim().is_empty(){
                    println!("Failed to read input.");
                }
                else if back.trim()=="e"{
                    break;
                }
            }
            
        }
        else if app_command.trim()=="3"{
            let output = Command::new("cmd")
                         .args(&["/C", "cls"])
                         .status()
                         .expect("Failed to clear terminal");

                        if !output.success() {
                              eprintln!("Failed to clear terminal");
                        }
            unsafe{
            fitness::inspection(&CURRENT_USER);
            }
            
        }
        else if app_command.trim()=="7"{
            let output = Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear terminal");

            if !output.success() {
                eprintln!("Failed to clear terminal");
            }

            return;
        }
        else if app_command.trim()=="e" {
            
            break;
        }
        else{
            print!(" ");
        }
    }
}
}
