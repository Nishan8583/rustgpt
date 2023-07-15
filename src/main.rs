use clap::{Command,Arg, ArgAction};
use rustgpt::{*};

fn main() {

    let oai = parse_yaml();
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
            println!("{}",oai.get_chat_gpt_message(message));
        },
        None => {},
    }

    // If they want to go interactive
    let value = matches.get_flag("interactive");
    if value {
        oai.start_interactive();
    } 
}

