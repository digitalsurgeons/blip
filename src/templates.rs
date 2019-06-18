use std::fs;
use std::path::Path;

static MARKUP_FILE: &str = include_str!("../templates/markup.txt");
static SCRIPTS_FILE: &str = include_str!("../templates/scripts.txt");
static STYLES_FILE: &str = include_str!("../templates/styles.txt");
static EMOTION_FILE: &str = include_str!("../templates/emotion.txt");
static VUE_FILE: &str = include_str!("../templates/vue.txt");

pub fn write_all(component: &str, args: &super::cli::Args) -> Result<(), std::io::Error> {
    fs::create_dir(&component)?;

    if args.react {
        let index_path = format!("./{}/index.js", &component);
        fs::File::create(index_path)?;

        let emotion_path = format!("./{}/styles.js", &component);
        let path = Path::new(&emotion_path);
        fs::write(path, EMOTION_FILE)?;
    } else if args.vue {
        let vue_file = generate(VUE_FILE, &component);
        let vue_path = format!("./{}/index.vue", &component);
        let path = Path::new(&vue_path);
        fs::write(path, vue_file)?;
    } else {
        if !args.ignore.contains(&"markup".to_string()) {
            let markup_file = generate(MARKUP_FILE, &component);
            let markup_path = format!("./{}/index{}", &component, &args.markup_extension);
            let path = Path::new(&markup_path);
            fs::write(path, markup_file)?;
        }

        if !args.ignore.contains(&"styles".to_string()) {
            let styles_file = generate(STYLES_FILE, &component);
            let styles_path = format!("./{}/styles{}", &component, &args.styles_extension);
            let path = Path::new(&styles_path);
            fs::write(path, styles_file)?;
        }

        if !args.ignore.contains(&"scripts".to_string()) {
            let scripts_file = generate(SCRIPTS_FILE, &component);
            let scripts_path = format!("./{}/index{}", &component, &args.scripts_extension);
            let path = Path::new(&scripts_path);
            fs::write(path, scripts_file)?;
        }
    }

    Ok(())
}

fn generate(template: &str, component: &str) -> String {
    str::replace(template, "{{Component}}", &component)
}
