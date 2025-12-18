use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut name = "World".to_string();
    let mut upper = false;
    let mut repeat = 1;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--upper" => {
                upper = true;
            }
            "--repeat" => {
                if i + 1 < args.len() {
                    if let Ok(num) = args[i + 1].parse::<usize>() {
                        repeat = num;
                        i += 1;
                    }
                }
            }
            arg if !arg.starts_with('-') => {
                name = arg.to_string();
            }
            _ => {}
        }
        i += 1;
    }

    let mut message = format!("Hello, {}!", name);
    if upper {
        message = message.to_uppercase();
    }

    for _ in 0..repeat {
        println!("{}", message);
    }
}
