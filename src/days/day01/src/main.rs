use std::env;
use std::fs;

use day01::file_content_to_top_elf_calories;

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

    // let max_calories = file_content_to_max_elf_calories(file_content);
    // println!("max calorie: {}", max_calories);

    let elves = file_content_to_top_elf_calories(file_content);

    println!("top 3 total: {}", elves[0] + elves[1] + elves[2]);
}
