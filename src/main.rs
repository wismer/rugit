use std::env;
use std::fs;
use std::path::{PathBuf, Path};
extern crate sha1;


mod rugit;
// If the path to the repository is not provided, 
// then Jit should use the current directory as the place to create the repository.
use rugit::workspace::Workspace;

fn commit() {
    let workspace = Workspace {
        path: String::from("/Users/Matt/rugut")
    };

    let result = workspace.process_file(&Path::new("/Users/Matt/rugut/vice-v2.css"));
    match result {
        Ok(hash) => println!("HASH {}", hash),
        Err(_) => panic!("something went wrong")
    }
}

fn init(directory: Option<String>) {
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
                    let _ = fs::create_dir(p.as_path());

                    p.push("objects");
                    let _ = fs::create_dir(p.as_path());

                    p.pop();
                    let _ = fs::create_dir(p.as_path());

                    p.push("refs");
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
    let mut arguments = std::env::args();
    let command = match arguments.nth(1) {
        Some(arg) => arg,
        None => panic!("No command given!")
    };

    match command.as_ref() {
        "init" => init(arguments.nth(0)),
        "commit" => commit(),
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
