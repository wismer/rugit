use std::env;
use std::fs;
use std::path::PathBuf;
// If the path to the repository is not provided, 
// then Jit should use the current directory as the place to create the repository.


fn init(directory: Option<String>) {
    println!("ATTEMPTING WITH {:?}", directory);
    let result = match directory.as_ref() {
        Some(path) => {
            let mut path_buf: PathBuf = PathBuf::new();
            path_buf.push(path);
            // create the root if it does not exist yet
            if !path_buf.is_dir() {
                let _ = fs::create_dir(path_buf.as_path());
            }

            // create the main directory for rugit
            path_buf.push(".rugit");
            let _ = fs::create_dir(path_buf.as_path());

            // create the objects subdirectory in .rugit
            path_buf.push("objects");
            let _ = fs::create_dir(path_buf.as_path());

            // remove `objects` from the path
            path_buf.pop();

            // finally, add refs subdirectory and create it
            path_buf.push("refs");
            fs::create_dir(path_buf.as_path())
        },
        None => {
            let cwd = env::current_dir();
            match cwd {
                Ok(mut p) => {
                    p.push(".rugit");
                    fs::create_dir(p.as_path())
                },
                Err(_) => panic!("sdasd"),
            }
            // create_directory(cwd.as_path())
        }
    };

    match result {
        Ok(_) => print!("created!"),
        Err(_) => panic!("not created!")
    }
}

fn main() {
    let mut arguments = env::args();
    let command = match arguments.nth(1) {
        Some(arg) => arg,
        None => panic!("No command given!")
    };

    match command.as_ref() {
        "init" => init(arguments.nth(0)),
        _ => panic!("Unknown Command! -> {}", command)
    }




    // for argument in env::args() {
    //     println!("argument -> {}", argument);
    //     match argument.as_ref() {
    //        "init" => init(),
    //        "target/debug/rugit" => {},
    //        _ => panic!("Unknown command! -> {}", argument)
    //    }
    // }
}
