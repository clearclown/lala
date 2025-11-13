use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "lala")]
#[command(about = "A text editor with GUI", long_about = None)]
struct Args {
    /// Path to open (file or directory)
    #[arg(default_value = ".")]
    path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Convert to absolute path
    let path = if args.path.is_relative() {
        std::env::current_dir()?.join(&args.path)
    } else {
        args.path
    };

    // Launch GUI
    gui_base::run(path)?;

    Ok(())
}
