use std::fmt::Debug;
use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// Comment 1
pub struct ClArgs {
    /// set output path
    #[argh(option, short = 'o')]
    output: String,
}