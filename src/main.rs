mod cli;
mod templates;

#[paw::main]
fn main(args: cli::Args) -> Result<(), std::io::Error> {
    for component in args.component.iter() {
        templates::write_all(&component, &args)?;
    }
    Ok(())
}
