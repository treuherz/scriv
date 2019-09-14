use std::path::PathBuf;
use std::fs::{OpenOptions, create_dir_all};
use std::io::{BufRead, BufReader, Write};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "scriv")]
struct Opt {
    file: Option<PathBuf>,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    #[structopt(name = "add")]
    Add {
        #[structopt(name = "TEXT")]
        text: String,
    },
    #[structopt(name = "list")]
    List,
}

fn main() -> Result<(), failure::Error> {
    let opt = Opt::from_args();
    let file = dirs::data_dir().map(|p| p.join("scriv/log").into_boxed_path()).unwrap();
    create_dir_all(file.parent().unwrap())?;

    match opt.cmd {
        Command::Add { text } => {
            let note = if text.ends_with("\n") {text} else {text + "\n"};
            let mut file = OpenOptions::new().append(true).create(true).open(file)?;
            file.write(note.as_ref())
                .and(Ok(()))
                .map_err(failure::Error::from)
        }
        Command::List => {
            BufReader::new(OpenOptions::new().read(true).open(file)?)
                .lines()
                .for_each(|f| println!("{}", f.unwrap()));
            Ok(())
        }
    }
}
