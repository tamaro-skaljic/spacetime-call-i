use clap::{arg, command, value_parser};
use std::{
    collections::VecDeque,
    io::{Write, stderr, stdout},
};

fn main() -> Result<(), String> {
    let arg_matches = command!()
        .arg(
            arg!(
                -s --server <server> "The name of the server which hosts the database."
            )
            .required(false)
            .default_value("local")
            .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(
                -d --database <database> "The name or identity of the database."
            )
            .required(true)
            .value_parser(value_parser!(String)),
        )
        .get_matches();

    let server = arg_matches.get_one::<String>("server").unwrap().clone();

    let database = arg_matches
        .get_one::<String>("database")
        .expect("You must provide the name or identity of a database.")
        .clone();

    loop {
        let line = readline()?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        match respond(&server, &database, line) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                write!(std::io::stdout(), "{err}").map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}

fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "$ ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}

fn respond(server: &String, database: &String, line: &str) -> Result<bool, String> {
    let mut args: VecDeque<String> = shlex::split(line).ok_or("error: Invalid quoting")?.into();

    let reducer = args
        .pop_front()
        .expect("You need to supply a reducer name! (or quit)");

    if reducer.eq(&"quit") || reducer.eq(&"exit") {
        write!(std::io::stdout(), "Exiting ...").map_err(|e| e.to_string())?;
        std::io::stdout().flush().map_err(|e| e.to_string())?;
        return Ok(true);
    }
    if reducer.eq(&"clear") || reducer.eq(&"clean") || reducer.eq(&"cls") {
        print!("{}[2J", 27 as char);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        return Ok(false);
    }

    spacetime_call(&server, &database, &reducer, args);

    Ok(false)
}

fn spacetime_call(
    server: &String,
    database: &String,
    reducer_name: &String,
    arguments: VecDeque<String>,
) {
    println!("spacetime call --server {server} {database} {reducer_name} {arguments:?}");
    let output = std::process::Command::new("spacetime")
        .arg("call")
        .arg("--server")
        .arg(server)
        .arg(database)
        .arg(reducer_name)
        .args(arguments)
        .output()
        .unwrap();
    let _ = stdout().write_all(&output.stdout);
    let _ = stderr().write_all(&output.stderr);
}
