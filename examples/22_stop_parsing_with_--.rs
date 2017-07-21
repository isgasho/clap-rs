extern crate clap;

use clap::{App, Arg};

/// myprog -f -p=bob -- sloppy slop slop
fn main() {

    let matches = App::new("myprog")
        .arg(Arg::new("eff").short("f"))
        .arg(Arg::new("pea").short("p").set(ArgSettings::TakesValue))
        .arg(Arg::new("slop").multiple(true))
        .get_matches();


    println!("-f used: {:?}", matches.is_present("eff"));
    println!("-p's value: {:?}", matches.value_of("pea"));
    println!("'slops' values: {:?}",
             matches
                 .values_of("slop")
                 .map(|vals| vals.collect::<Vec<_>>()));

    // Continued program logic goes here...
}
