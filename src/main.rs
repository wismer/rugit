use std::env;


fn init() {
    unimplemented!();
}

fn main() {
    for argument in env::args() {
        println!("argument -> {}", argument);
        match argument.as_ref() {
           "init" => init(),
           "target/debug/rugit" => {},
           _ => panic!("Unknown command! -> {}", argument)
       } 
    }
}
