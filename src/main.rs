use std::{fs, io};
use std::fs::File;

fn main() -> io::Result<()> {
    // println!("Hello, world!");

    let mut entries = fs::read_dir("./spektrum/")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    // The entries have now been sorted by their path.
    for entry in entries {
        println!("Processing {:?}", entry);

        let file = File::open(entry)?;
        let mut rdr = csv::Reader::from_reader(file);
        for result in rdr.records() {
            let record = result?;
            println!("{:?}", record);
        }
    }

    Ok(())
}