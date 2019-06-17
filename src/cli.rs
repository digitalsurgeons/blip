#[derive(structopt::StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Args {
    /// Component to create
    #[structopt(name = "COMPONENT")]
    pub component: Vec<String>,

    /// Tells Blip to not create these files
    #[structopt(short = "i", long = "ignore")]
    pub ignore: Vec<String>,

    /// Extension for markup
    #[structopt(short = "m", long = "markup-extension", default_value = ".twig")]
    pub markup_extension: String,

    /// Extension for styles
    #[structopt(short = "s", long = "styles-extension", default_value = ".pcss")]
    pub styles_extension: String,

    /// Extension for scripts
    #[structopt(short = "x", long = "scripts-extension", default_value = ".js")]
    pub scripts_extension: String,

    /// Generate React templates
    #[structopt(short = "r", long = "react")]
    pub react: bool,
}