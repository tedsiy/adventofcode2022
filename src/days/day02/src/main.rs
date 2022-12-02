use std::env;
use std::fs;

use day02::get_total_score_by_desired_result;

fn main() {
    // https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        panic!("missing filename")
    }
    let file_path = &args[1];
    println!("Input file {}", file_path);

    // open file as string
    let file_result = fs::read_to_string(file_path);

    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html?highlight=result#recoverable-errors-with-result
    let file_content = match file_result {
        Ok(file) => file,
        Err(error) => {
            panic!(
                "Problem opening the file: {}\nerror: {:?}",
                file_path, error
            )
        }
    };

    // let total_score = get_total_score_given(file_content);

    let total_score = get_total_score_by_desired_result(file_content);

    println!("{}", total_score);
}
