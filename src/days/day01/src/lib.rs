/// Convert elf inventory file to summary of each elf calories. Return a vec array of the top 3
///
/// String input represent calories inventory of each elf.
/// Each elfs inventory is separated by a blank line
///
/// # Note
/// if row contain NaN value will be replaced with 0, and error message will be printed
/// proper error handling should be propogated - see Propagating Errors in the book: <https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html?highlight=return%20result#propagating-errors>
///
/// # Example
/// ```
/// use day01::file_content_to_top_elf_calories;
///
/// let test_file_content = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000".to_string();
/// assert_eq!([24000, 11000, 10000], file_content_to_top_elf_calories(test_file_content));
/// ```
pub fn file_content_to_top_elf_calories(file_content: String) -> [u64; 3] {
    let mut elves: [u64; 3] = [0, 0, 0];
    let mut elf_calories: u64 = 0;
    file_content.lines().for_each(|s| {
        if s.trim().is_empty() {
            add_to_elves(elf_calories, &mut elves);
            elf_calories = 0
        } else {
            let val = match s.parse::<u64>() {
                Ok(v) => v,
                Err(error) => {
                    println!("error parsing {s}: {}", error.to_string());
                    0
                }
            };
            elf_calories += val;
        }
    });
    if elf_calories > 0 {
        add_to_elves(elf_calories, &mut elves);
    }

    elves
}

fn add_to_elves(elf_calories: u64, elves: &mut [u64; 3]) {
    if elf_calories > elves[0] {
        elves[2] = elves[1];
        elves[1] = elves[0];
        elves[0] = elf_calories;
    } else if elf_calories > elves[1] {
        elves[2] = elves[1];
        elves[1] = elf_calories;
    } else if elf_calories > elves[2] {
        elves[2] = elf_calories;
    }
}

/// Get max calorie and elf carried
///
/// String input represent calories inventory of each elf.
/// Each elfs inventory is separated by a blank line
///
/// # Example
/// ```
/// use day01::file_content_to_max_elf_calories;
///
/// let test_file_content = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000".to_string();
/// assert_eq!(24000, file_content_to_max_elf_calories(test_file_content));
/// ```
pub fn file_content_to_max_elf_calories(file_content: String) -> u64 {
    let mut elf_calories: u64 = 0;
    let mut max_calories: u64 = 0;
    file_content.lines().for_each(|s| {
        if s.trim().is_empty() {
            if elf_calories > max_calories {
                max_calories = elf_calories;
            }
            elf_calories = 0
        } else {
            let val = match s.parse::<u64>() {
                Ok(v) => v,
                Err(error) => {
                    println!("error parsing {s}: {}", error.to_string());
                    0
                }
            };
            elf_calories += val;
        }
    });
    if elf_calories > max_calories {
        max_calories = elf_calories;
    }
    max_calories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_elf_calories() {
        // this test is not needed it is already performed doc test
        let test_file_content =
            "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000".to_string();
        let result = file_content_to_top_elf_calories(test_file_content);
        assert_eq!(result, [24000, 11000, 10000]);
    }

    #[test]
    fn ut_elf_calories_with_nan() {
        let test_file_content =
            "1000\n2000\n3000\n\n4000\n\n5a00\n6000\n\n7000\n8000\n9000\n\n10000".to_string();
        let result = file_content_to_top_elf_calories(test_file_content);
        assert_eq!(result, [24000, 10000, 6000]);
    }

}
