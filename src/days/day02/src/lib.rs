/// code shape you selected
/// 
/// The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ShapeCode {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
    Invalid = 0,
}

/// Result value for round outcome - **outcome of the round**
/// * 0 if you lost
/// * 3 if the round was a draw
/// * 6 if you won
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum RoundResultsCode {
    Won = 6,
    Draw = 3,
    Lost = 0,
}

/// calculates the score for the outcome of the round.
///
/// ## Day 2: Rock Paper Scissors
/// exact wording from aoc day 2:
/// * "score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won)."
/// * "Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock"
///
///
/// # Example
/// ```
/// use day02::{outcome_of_the_round_result, RoundResultsCode, ShapeCode};
/// use std::matches;
///
/// let _result = outcome_of_the_round_result(ShapeCode::Rock, ShapeCode::Scissors);
/// assert!(matches!( RoundResultsCode::Won, _result));
/// ```
pub fn outcome_of_the_round_result(
    my_play: ShapeCode,
    opponent_play: ShapeCode,
) -> RoundResultsCode {
    // Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
    // 1 for Rock, 2 for Paper, and 3 for Scissors
    let round_outcome = match [my_play, opponent_play] {
        [ShapeCode::Rock, ShapeCode::Scissors]
        | [ShapeCode::Scissors, ShapeCode::Paper]
        | [ShapeCode::Paper, ShapeCode::Rock] => RoundResultsCode::Won,
        [ShapeCode::Rock, ShapeCode::Rock]
        | [ShapeCode::Paper, ShapeCode::Paper]
        | [ShapeCode::Scissors, ShapeCode::Scissors] => RoundResultsCode::Draw,
        _ => RoundResultsCode::Lost,
    };
    round_outcome
}

/// calculates the score for the outcome of the round.
///
/// ## Day 2: Rock Paper Scissors
/// exact wording from aoc day 2:
/// * Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.
///
///
/// # Example
/// ```
/// use day02::{decode_opponents_play, RoundResultsCode, ShapeCode};
/// use std::matches;
///
/// let _result = decode_opponents_play("A");
/// assert!(matches!( ShapeCode::Rock, _result));
/// ```
pub fn decode_opponents_play(play: &str) -> ShapeCode {
    // A for Rock, B for Paper, and C for Scissors
    // 1 for Rock, 2 for Paper, and 3 for Scissors
    let opponent_play = match play {
        "A" => ShapeCode::Rock,
        "B" => ShapeCode::Paper,
        "C" => ShapeCode::Scissors,
        _ => ShapeCode::Invalid,
    };
    opponent_play
}

/// calculates the score for the outcome of the round.
///
/// ## Day 2: Rock Paper Scissors
/// exact wording from aoc day 2:
/// * The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.
///
/// # Example
/// ```
/// use day02::{decode_my_play, RoundResultsCode, ShapeCode};
/// use std::matches;
///
/// let _result = decode_my_play("X");
/// assert!(matches!( ShapeCode::Rock, _result));
/// ```
pub fn decode_my_play(play: &str) -> ShapeCode {
    // X for Rock, Y for Paper, and Z for Scissors
    // 1 for Rock, 2 for Paper, and 3 for Scissors
    let my_play = match play {
        "X" => ShapeCode::Rock,
        "Y" => ShapeCode::Paper,
        "Z" => ShapeCode::Scissors,
        _ => ShapeCode::Invalid,
    };
    my_play
}

/// calculate score based on "The second column, you reason, must be what you should play in response:
/// * X for Rock,
/// * Y for Paper,
/// * and Z for Scissors.
///
/// # Example
/// ```
/// use day02::get_total_score_given;
///
/// let test_file_content = "A Y\nB X\nC Z".to_string();
/// assert_eq!(15, get_total_score_given(test_file_content));
/// ```
pub fn get_total_score_given(file_content: String) -> u32 {
    let mut total_score: u32 = 0;
    file_content.lines().for_each(|s| {
        let split = s.trim().split(" ");

        let vec = split.collect::<Vec<&str>>();

        let my_play = decode_my_play(&vec[1]);
        let my_shape_score = my_play as u32; //using copy trait

        let opponent_play = decode_opponents_play(&vec[0]);

        let round_outcome = outcome_of_the_round_result(my_play, opponent_play);

        total_score += my_shape_score + round_outcome as u32;
    });

    total_score
}

/// Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
pub fn play_to_win(opponent_play: ShapeCode) -> ShapeCode {
    let play = match opponent_play {
        ShapeCode::Rock => ShapeCode::Paper,
        ShapeCode::Paper => ShapeCode::Scissors,
        ShapeCode::Scissors => ShapeCode::Rock,
        _ => ShapeCode::Invalid,
    };
    play
}

/// Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
pub fn play_to_lose(opponent_play: ShapeCode) -> ShapeCode {
    let play = match opponent_play {
        ShapeCode::Rock => ShapeCode::Scissors,
        ShapeCode::Paper => ShapeCode::Rock,
        ShapeCode::Scissors => ShapeCode::Paper,
        _ => ShapeCode::Invalid,
    };
    play
}

/// given the second column states the game should win, lose, or draw. Return the shape code needed to meet this need.
///
/// ## Day 2: Rock Paper Scissors
/// exact wording from aoc day 2:
/// *  X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win.
///
/// # Example
/// ```
/// use day02::{decode_win_lose_draw_play, ShapeCode};
/// use std::matches;
///
/// let _result = decode_win_lose_draw_play(ShapeCode::Rock, "X");
/// assert!(matches!( ShapeCode::Scissors, _result));
/// ```
pub fn decode_win_lose_draw_play(opponent_play: ShapeCode, encrypted_play: &str) -> ShapeCode {
    // X for Rock, Y for Paper, and Z for Scissors
    // 1 for Rock, 2 for Paper, and 3 for Scissors
    let my_play = match encrypted_play {
        "X" => play_to_lose(opponent_play),
        "Y" => opponent_play,
        "Z" => play_to_win(opponent_play),
        _ => play_to_lose(opponent_play),
    };
    my_play
}

/// Calculate the score based on second column specifying outcome
/// 
/// ## Part Two
/// The Elf finishes helping with the tent and sneaks back over to you.
/// Anyway, the second column says how the round needs to end:
/// * X means you need to lose,
/// * Y means you need to end the round in a draw,
/// * and Z means you need to win.
/// # Example
/// ```
/// use day02::get_total_score_by_desired_result;
///
/// let test_file_content = "A Y\nB X\nC Z".to_string();
/// assert_eq!(12, get_total_score_by_desired_result(test_file_content));
/// ```
pub fn get_total_score_by_desired_result(file_content: String) -> u32 {
    let mut total_score: u32 = 0;
    file_content.lines().for_each(|s| {
        let split = s.trim().split(" ");

        let vec = split.collect::<Vec<&str>>();

        let opponent_play = decode_opponents_play(&vec[0]);
        // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win.
        let my_play = decode_win_lose_draw_play(opponent_play, vec[1]);
        let my_shape_score = my_play as u32; //using copy trait

        let round_outcome = outcome_of_the_round_result(my_play, opponent_play);

        total_score += my_shape_score + round_outcome as u32;
    });

    total_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_outcome_of_the_round_win() {
        let _result = outcome_of_the_round_result(ShapeCode::Rock, ShapeCode::Scissors);
        assert!(matches!(RoundResultsCode::Won, _result));
    }

    #[test]
    fn ut_outcome_of_the_round_lost() {
        let _result = outcome_of_the_round_result(ShapeCode::Scissors, ShapeCode::Rock);
        assert!(matches!(RoundResultsCode::Won, _result));
    }

    #[test]
    fn ut_outcome_of_the_round_draw() {
        let _result = outcome_of_the_round_result(ShapeCode::Paper, ShapeCode::Paper);
        assert!(matches!(RoundResultsCode::Won, _result));
    }
}
