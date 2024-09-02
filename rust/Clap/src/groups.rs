use clap::{command, Arg, ArgGroup, Command};

fn main() {
    let match_result = command!()
        .about("about me is here")
        .group(ArgGroup::new("person")
            .arg("firstname")
            .arg("lastname")
            .conflicts_with("pet")
        )
        .group(ArgGroup::new("pet")
            .arg("petname")
            )
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
    .arg(
        Arg::new("petname")
        .short('n')
        .long("pet-name")
        //.required(true)
    )
    .get_matches();
}
