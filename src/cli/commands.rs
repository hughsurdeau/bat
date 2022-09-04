use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {

    /// Adds text to a file
    Add {
        text: String,
    },

    /// Creates a new file with text
    /// TODO try and make this flagable
    Create {
        text: String,
    },
}
