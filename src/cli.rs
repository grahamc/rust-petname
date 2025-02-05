use std::path::PathBuf;

use clap::Parser;

/// Generate human readable random names.
#[derive(Parser)]
#[clap(
    name = "rust-petname",
    author,
    after_help = "Based on Dustin Kirkland's petname project <https://github.com/dustinkirkland/petname>."
)]
pub struct Cli {
    /// Number of words in name
    #[clap(short, long, value_name = "WORDS", default_value_t = 2)]
    pub words: u8,

    /// Separator between words
    #[clap(short, long, value_name = "SEP", default_value = "-")]
    pub separator: String,

    /// Use small words (0), medium words (1), or large words (2)
    #[clap(short, long, value_name = "COM", possible_values = &["0", "1", "2"], default_value_t = 0, hide_possible_values = true)]
    pub complexity: u8,

    /// Directory containing adjectives.txt, adverbs.txt, names.txt
    #[clap(short, long = "dir", value_name = "DIR", conflicts_with = "complexity")]
    pub directory: Option<PathBuf>,

    /// Generate multiple names; pass 0 to produce infinite names
    /// (--count=0 is deprecated; use --stream instead)
    #[clap(long, value_name = "COUNT", default_value_t = 1)]
    pub count: usize,

    /// Stream names continuously
    #[clap(long, conflicts_with = "count")]
    pub stream: bool,

    /// Do not generate the same name more than once
    #[clap(long)]
    pub non_repeating: bool,

    /// Maximum number of letters in each word; 0 for unlimited
    #[clap(short, long, value_name = "LETTERS", default_value_t = 0)]
    pub letters: usize,

    /// Generate names where each word begins with the same letter
    #[clap(short, long)]
    pub alliterate: bool,

    /// Generate names where each word begins with the given letter
    #[clap(short = 'A', long, value_name = "LETTER")]
    pub alliterate_with: Option<char>,

    // For compatibility with upstream.
    /// Alias; see --alliterate
    #[clap(short, long)]
    pub ubuntu: bool,
}
