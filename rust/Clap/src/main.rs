use clap::{command, Arg, ArgGroup, Command};

fn main() {
    let match_result = command!()
        .about("about me is here")
        .subcommand(
            Command::new("register-person")
            .arg(
                Arg::new("firstname")
                    .short('f')
                    .long("first-name")
                    .aliases(["fname, firstname"])
                    //.required(true)
                    .help("The persons first name")
                    //.conflicts_with("lastname")
                    

            )  
            .arg(
                Arg::new("lastname")
                    .short('l')
                    .long("last-name")
                    .aliases(["lname"])
            )
        )
        .subcommand(
            Command::new("pet")
                .arg(
                    Arg::new("petname")
                    .short('n')
                    .long("pet-name")
                    .required(true)
                )   
        )
        .arg(
            Arg::new("fluffy")
            .short('y')
            .long("fluffy")
            //.required(true)
        )
    .get_matches();

    //println!("{}", match_result.get_one::<String>("fluffy").unwrap());
    let pet_args = match_result.subcommand_matches("register-person");
    println!("does petnames: {}", pet_args.unwrap().get_one::<String>("firstname").unwrap());
}
