use file_access::AsFile;
use std::io::Result;

fn main() -> Result<()> {
    Ok({
        let cargo = "Cargo.toml".as_file().read_lines()?.len();
        let world = library_project::add(cargo, cargo);
        println!("Hello, {world}!");
    })
}
