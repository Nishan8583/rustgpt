use std::io;
use std::io::Write;

use clap::{Command,Arg, ArgAction};
use rustgpt::{*};

fn main() {

    // getting command line arguements, I used builder pattern
    // derive pattern seems wierd to me
    let matches  = Command::new("program")
        .arg(
            Arg::new("interactive")
           .short('i')
           .long("interactive")
           .action(ArgAction::SetTrue)
           .help("for interactive mdoe")
        )
        .arg(
            Arg::new("message")
           .short('m')
           .long("message")
           .help("message to send")
        )
        .get_matches();

        // If user wants to send message
    match matches.get_one::<String>("message") {
        Some(v) => { 
            let message = v.clone();
            let messages = vec![
                Message{
                    role: "assistant".to_string(),
                    content: message,
                }
            ];
            println!("{}",get_chat_gpt_message(messages, "<API_KEY>"));
        },
        None => {},
    }

    // If they want to go interactive
    let value = matches.get_flag("interactive");
    if value {
        loop{
            print!("(you)>>> ");
            io::stdout().flush().unwrap();
            let mut value = String::new();
            io::stdin().read_line(&mut value).unwrap();
            let value = value.replace("\n", "").replace("\r", "");
            if value == "exit" || value =="quit" {
                return;
            }
            let messages = vec![
                Message{
                    role: "assistant".to_string(),
                    content: value,
                }
            ];
            
            print!("(gpt)>>> {}",get_chat_gpt_message(messages, "<API_KEY"));
            println!("");
            io::stdout().flush().unwrap();
        }
    } 
}