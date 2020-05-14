extern crate feembox;

use std::process::exit;
use std::io::{stdout, stderr};


fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    if let Err(err) = result_main() {
        // err.print_error(&mut stderr());
        // err.exit_value()
        -1
    } else {
        0
    }
}

fn result_main() -> Result<(), ()/*feembox::Error*/> {
    let opts = feembox::Options::parse();
    eprintln!("{:#?}", opts);

    Ok(())
}
