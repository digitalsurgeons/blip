use std::fs;
use std::path::Path;

#[derive(structopt::StructOpt)]
struct Args {
    /// Component to create
    #[structopt(name = "COMPONENT")]
    component: String,

    /// Tells Blip to not create these files
    #[structopt(short = "i", long = "ignore")]
    ignore: Vec<String>,

    // Extension for markup
    #[structopt(short = "m", long = "markup-extension", default_value = ".twig")]
    markupExtension: String,

    // Extension for styles
    #[structopt(short = "s", long = "styles-extension", default_value = ".css")]
    stylesExtension: String,

    // Extension for scripts
    #[structopt(short = "x", long = "scripts-extension", default_value = ".js")]
    scriptsExtension: String,
}

#[paw::main]
fn main(args: Args) -> Result<(), std::io::Error> {
    fs::create_dir(&args.component)?;

    for ignore in args.ignore.iter() {
        match ignore.as_ref() {
            "markup" => println!("ignore the markup"),
            "styles" => println!("ignore the styles"),
            "scripts" => println!("ignore the scripts"),
            _ => {
                println!("Error: Valid options for --ignore are 'markup', 'styles', and 'scripts'")
            }
        }
    }

    let markup_file = format!(
        "
<section class='{}'>
    <!-- markup goes here-->
</section>
    ",
        &args.component
    );

    // let path = Path::new(".");
    // path.join(&args.component)
    //     .join("index")
    //     .join(&args.markupExtension);
    // println!("{:?}", path);
    // fs::write(path, markup_file)?;

    Ok(())
}
