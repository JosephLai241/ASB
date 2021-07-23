//! Command-line interface.

use structopt::StructOpt;

/// Defining all flags used in this program.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "A Simple Blockchain", 
    about = "Run a simple blockchain implementation"
)]
pub struct Args {
    /// Set the difficulty level.
    #[structopt(
        short = "d", 
        long = "difficulty", 
        default_value = "5"
    )]
    pub difficulty: usize,

    /// Set the total number of blocks in the blockchain.
    #[structopt(
        short = "t", 
        long = "total", 
        default_value = "5"
    )]
    pub total_blocks: i32
}

/// Check argument values. Panics if `difficulty` >= 64 or `total_blocks` < 2.
fn check_args() -> Args {
    let args = Args::from_args();
    
    if args.difficulty >= 64 {
        panic!("Difficulty cannot be greater than or equal to 64.");
    } else if args.total_blocks < 2 {
        panic!(
            format!(
                "Cannot provide value of {}. Genesis block is the first block.", 
                args.total_blocks
            )
        );
    }

    args
}

/// Get args.
pub fn get_args() -> Args {
    check_args()
}

#[cfg(test)]
mod test_cli {
    use super::*;
    
    use assert_cmd::Command;

    #[test]
    fn test_invalid_arg() {
        Command::cargo_bin("asb")
            .unwrap()
            .arg("-q")
            .assert()
            .failure();
    }

    #[test]
    fn test_check_args_invalid_difficulty_arg() {
        Command::cargo_bin("asb")
            .unwrap()
            .args(&["-d", "64"])
            .assert()
            .failure();
    }

    #[test]
    fn test_check_args_invalid_total_blocks_arg() {
        Command::cargo_bin("asb")
            .unwrap()
            .args(&["-t", "1"])
            .assert()
            .failure();
    }

    #[test]
    fn test_get_args() {
        let args = get_args();

        assert_eq!(args.difficulty, 5);
        assert_eq!(args.total_blocks, 5);
    }
}
