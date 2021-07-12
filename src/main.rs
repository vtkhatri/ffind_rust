use std::env;
use std::process;
use std::io;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    // making the command
    let cmd_args_result = make_command(args);
    let cmd_args = match cmd_args_result {
        Err(e) => {
            println!("Error making command: {}", e);
            process::exit(1);
        },
        Ok(cmd_args) => cmd_args,
    };
    
    // executing the command
    let exec_status_result = process::Command::new("find")
                                              .args(cmd_args)
                                              .stdout(process::Stdio::inherit())
                                              .stderr(process::Stdio::inherit())
                                              .status();

    // proper error code propogation
    // spaghettified because Command::status() returns - Ok(Exitstatus(Exitstatus(code)))
    // we need to unwrap highest Ok then unwrap the Exitstatus to get to the return code of
    // command executed
    process::exit(match exec_status_result {
        Ok(exec_out) => match exec_out.code() {
            Some(code) => code,
            None       => {
                println!("Process terminated by signal");
                130 // because when I press ctrl-c and echo $? right after, it prints 130
            },
        },
        Err(e) => {
            println!("Error executing command: {}", e);
            2
        },
    });
}

fn print_usage() {
    println!("Usage: ffind [-fdri] [-e=maxdepth] [--debug --help] [expression] [path]");
}

fn make_command(args_in: Vec<String>) -> Result<Vec<String>, io::Error> {
    let mut args_out: Vec<String> = Vec::new();

    let _sorted_args = sort_args(args_in)?;

    Err(io::Error::new(io::ErrorKind::Other, "test"))
    //Ok(args_out)
}

#[derive(Debug)]
struct SortedArgs {
    short_args: Vec<String>,
    long_args: Vec<String>,
    file_name: String,
    path: String,
    exec_args: String,
}

fn sort_args(args_in: Vec<String>) -> Result<SortedArgs, io::Error> {
    let mut short_args: Vec<String> = Vec::new();
    let mut long_args: Vec<String> = Vec::new();
    let mut file_name = String::from("");
    let mut path = String::from("");
    let mut exec_args = String::from("");

    for (arg_no, arg) in args_in.iter().enumerate() {
        let re = Regex::new(r"^(?P<flag_marker>--?)(?P<flags>[^=]+)(?P<max_depth>.*?)$").unwrap();

        // let regex_arg: Vec<String> = re.find_iter(&arg).filter_map(|cap| cap.as_str().parse().ok()).collect();
        // println!("regex_arg={:?}", regex_arg)
        // Borked : this is giving only cap[0], the full string, it's not collecting the regex matches

        if arg_no != 0 {
            println!("{:?}", re.captures(&arg).unwrap().name("max_depth").unwrap());
            match re.captures(&arg) {
                Some(caps) => {
                    if caps.name("flag_marker").unwrap().as_str() == "--" {
                        long_args.push(get_long_args(caps.name("flags").unwrap().as_str())?.join(" "));
                    } else {
                        short_args.push(get_short_args(caps.name("flags").unwrap().as_str())?.join(" "));
                        if caps.name("max_depth").unwrap().end() != caps.name("max_depth").unwrap().start() {
                            short_args.push(get_short_args(caps.name("max_depth").unwrap().as_str())?.join(" "));
                        }
                    }
                }
                None => (),
            };
        }
    }

    let mut sorted_args = SortedArgs {
        short_args: short_args,
        long_args: long_args,
        file_name: file_name,
        path: path,
        exec_args: exec_args,
    };

    Ok(sorted_args)
}

fn get_short_args(args_in: &str) -> Result<Vec<String>, io::Error> {
    let mut ret_short_args: Vec<String> = Vec::new();
    println!("{:?}", args_in);
    return Ok(ret_short_args);
}

fn get_long_args(args_in: &str) -> Result<Vec<String>, io::Error> {
    let mut ret_long_args: Vec<String> = Vec::new();
    println!("{:?}", args_in);
    return Ok(ret_long_args);
}
