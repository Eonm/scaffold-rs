use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate clap;

extern crate chrono;
use chrono::prelude::*;

mod path_range;
mod path_parent;

static mut VERBOSITY: bool = false;

mod cli;

fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        ("scaffold", Some(matches)) => {
            unsafe {
                VERBOSITY = matches.is_present("verbosity");
            }

            let parsed_config = match matches.value_of("template") {
                Some(template) => {
                    let template_path = template;
                    let config = open_config(&Path::new(template_path));
                    let model = load_config(config);
                    model
                },
                None => {
                    let input_paths = matches.values_of("input").unwrap()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>();
                    let model = Model {
                        name    :   None,
                        notes   :   None,
                        author  :   None,
                        email   :   None,
                        licence :   None,
                        paths   :   input_paths
                    };
                    model
                }
            };

            if matches.is_present("display") {
                  println!("Printing template info");
                  println!("{}", parsed_config.display());
            } else if matches.is_present("dry_run") {
                dry_run(parsed_config)
            } else {
              scaffold(&parsed_config);
            }
        }
        _ => {}
    };
}

//-------------------------------------------------------------------------------------------------
// Path processing and analysis
//-------------------------------------------------------------------------------------------------

fn is_written_like_a_dir(path : &Path) -> bool {
    let path_str = path.to_str().expect("failed to get path name");
    if path_str.ends_with("/") || path_str.ends_with("\\") {
        true
    } else {
        false
    }
}

fn is_written_like_a_file (path : &Path) -> bool {
    !is_written_like_a_dir(path)
}

fn get_ancestors (path: &Path) -> Option<&Path> {
    let mut ancestors : Vec<&Path> = path.ancestors().collect();
    ancestors.pop();

    if ancestors.len() >= 2 {
        Some(ancestors[1])
    } else {
        None
    }
}

fn have_ancestors (path : &Path) -> bool {
    match get_ancestors(&path) {
        Some(_) => true,
        None => false
    }
}

//-------------------------------------------------------------------------------------------------
// Config processing
//-------------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
struct Model {
    name    :   Option<String>,
    notes   :   Option<String>,
    author  :   Option<String>,
    email   :   Option<String>,
    licence :   Option<String>,
    paths   :   Vec<String>
}

impl Model {
    pub fn get_dirs (&self) -> Vec<&Path> {
        let dir_path : Vec<&Path> = self.paths.iter()
        .filter_map(|path| {
            let p = Path::new(path);
            if is_written_like_a_dir(&p) {
                return Some(p)
            } else if have_ancestors(&p) {
                return Some(get_ancestors(&p).expect("failed to get path ancestors"))
            }
            None
        }).collect();
        dir_path
    }

    pub fn get_files (&self) -> Vec<&Path> {
        let files_paths : Vec<&Path> = self.paths.iter().filter(|path| {
            is_written_like_a_file(&Path::new(path))
        })
        .map(|path| {
            Path::new(path)
        }).collect();
        files_paths
    }

    pub fn display (self) -> String {
        format!("name\t:\t{}\nnotes\t:\t{}\nauthor\t:\t{}\nemail\t:\t{}\nlicence\t:\t{}\npaths\t:\t{:?}",
            &self.name.unwrap_or("undefined".to_string()),
            &self.notes.unwrap_or("undefined".to_string()),
            &self.author.unwrap_or("undefined".to_string()),
            &self.email.unwrap_or("undefined".to_string()),
            &self.licence.unwrap_or("undefined".to_string()),
            &self.paths
        )
    }
}

fn open_config (path : &Path) -> String {
    let mut file = match File::open(path) {
        Err(why) => {
            log(&format!("Can't open config file {}", why), LogType::Error);
            panic!("Can't open config file {}", why)
        },
        Ok(file) => file,
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(why) => log(&format!("Can't read config file {}", why), LogType::Error),
    }
    content
}

fn load_config (config : String) -> Model {
    let json_config : Model = match serde_json::from_str(&config) {
        Err(why) => {
            let error_mssg = format!("Can't load config file to json {}", why);
            log(&error_mssg, LogType::Error);
            panic!(error_mssg)
        },
        Ok(json_data) => json_data,
    };
    return json_config
}

//-------------------------------------------------------------------------------------------------
// Scaffolding
//-------------------------------------------------------------------------------------------------

fn scaffold (model : &Model) {
    log(&format!("Scaffolding with model {} created by {}", model.name.clone().unwrap_or("undefined".to_string()), model.author.clone().unwrap_or("undefined".to_string())), LogType::Info);

    //always crate dirs before files
    let generated_paths =  model.get_dirs().into_iter().flat_map(|path| {
        path_range::generate_paths(path.to_str().unwrap(),("[","]"))
    }).map(|path| path_parent::replace_with_parent_dir(path)).collect::<Vec<String>>();

    for dir in generated_paths {
        scaffold_dir(&Path::new(&dir))
    }

    let generated_files =  model.get_files().into_iter().flat_map(|path| {
        path_range::generate_paths(path.to_str().unwrap(),("[","]"))
    }).map(|path| path_parent::replace_with_parent_dir(path)).collect::<Vec<String>>();

    for file in generated_files {
        create_file(&Path::new(&file))
    }
}

fn dry_run (model : Model) {
    let generated_paths =  model.get_dirs().into_iter().flat_map(|path| {
        path_range::generate_paths(path.to_str().unwrap(),("[","]"))
    }).map(|path| path_parent::replace_with_parent_dir(path)).collect::<Vec<String>>();

    let generated_files =  model.get_files().into_iter().flat_map(|path| {
        path_range::generate_paths(path.to_str().unwrap(),("[","]"))
    }).map(|path| path_parent::replace_with_parent_dir(path)).collect::<Vec<String>>();

    for dir in generated_paths {
        println!("{}", dir);
    }

    for file in generated_files {
        println!("{}", file);
    }

}

fn scaffold_dir (path : &Path) {
        match fs::create_dir_all(path) {
        Err(why) => log(&format!("Can't create dir {:?} because => {}", path, why), LogType::Error),
        Ok(_) => log(&format!("Dir {:?} created", path), LogType::Info),
    }
}

fn create_file (path: &Path) {
let file = OpenOptions::new()
    .write(true)
    .create_new(true)
    .open(&path);

    match file {
        Err(why) => log(&format!("Can't create file {:?} because => {}", path, why), LogType::Error),
        Ok(_) => log(&format!("File {:?} created", path), LogType::Info),
    };
}

//-------------------------------------------------------------------------------------------------
// Logging errors and info
//-------------------------------------------------------------------------------------------------

enum LogType {
    Info,
    Error,
}

fn log(text: &String, mssg_type : LogType) {

    let log_message = match mssg_type {
        LogType::Error => format!("[error] {}", text),
        LogType::Info => format!("[info] {}", text),
    };

    unsafe {
        if VERBOSITY { println!("{}", log_message) }
    }

    update_log_file(log_message).expect("Failed to log message");
}

fn update_log_file(log_message : String) -> std::io::Result<()> {
    let now = Utc::now();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("scaffold_log.txt")
        .unwrap();
    let message_to_log = format!("{:?} : {}", now, log_message);
    writeln!(file,"{}",message_to_log);
    Ok(())
}

//-------------------------------------------------------------------------------------------------
// Tests
//-------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_have_ancestors() {
        assert_eq!(have_ancestors(Path::new("test_file.txt")), false);
        assert_eq!(have_ancestors(Path::new("/dir/test_file.txt")), true);
        assert_eq!(have_ancestors(Path::new("/")), false);
        assert_eq!(have_ancestors(Path::new("/dir1/dir2/")), true);
        assert_eq!(have_ancestors(Path::new("./")), false);
    }

    #[test]
    fn test_get_ancestors() {
        assert_eq!(get_ancestors(Path::new("test_file.txt")), None);
        assert_eq!(get_ancestors(Path::new("/dir/test_file.txt")), Some(Path::new("/dir")));
        assert_eq!(get_ancestors(Path::new("./dir/test_file.txt")), Some(Path::new("./dir")));
        assert_eq!(get_ancestors(Path::new("./")), None);
    }

    #[test]
    fn test_get_dirs() {
        let fake_model = Model {
            name    :   Some("fake_model".to_string()),
            notes   :   Some("".to_string()),
            author  :   Some("john doe".to_string()),
            email   :   Some("john.doe@mail.com".to_string()),
            licence :   Some("MIT".to_string()),
            paths   :   vec!["dir1/".to_string(), "/dir/file.txt".to_string(), "/dir1/dir2/".to_string(), "file.txt".to_string()],
        };

        let expected = vec![Path::new("dir1/"), Path::new("/dir/"), Path::new("/dir1/dir2/")];
        assert_eq!(fake_model.get_dirs(), expected)
    }

    #[test]
    fn test_get_files() {
        let fake_model = Model {
            name    :   Some("fake_model".to_string()),
            notes   :   Some("".to_string()),
            author  :   Some("john doe".to_string()),
            email   :   Some("john.doe@mail.com".to_string()),
            licence :   Some("MIT".to_string()),
            paths   :   vec!["dir1/".to_string(), "/dir/file.txt".to_string(), "/dir1/dir2/".to_string(), "file.txt".to_string()],
        };

        let expected = vec![Path::new("/dir/file.txt"), Path::new("file.txt")];
        assert_eq!(fake_model.get_files(), expected);
    }

    #[test]
    fn test_is_written_like_a_dir() {
        assert_eq!(is_written_like_a_dir(Path::new("/dir/file.txt")), false);
        assert_eq!(is_written_like_a_dir(Path::new("/dir/dir2/file.txt")), false);
        assert_eq!(is_written_like_a_dir(Path::new("\\dir\\")), true);
        assert_eq!(is_written_like_a_dir(Path::new("\\dir\\file.txt")), false);
        assert_eq!(is_written_like_a_dir(Path::new("\\dir\\dir2\\file.txt")), false);
        assert_eq!(is_written_like_a_dir(Path::new("\\")), true);
        assert_eq!(is_written_like_a_dir(Path::new("./")), true);
        assert_eq!(is_written_like_a_dir(Path::new("/")), true);
    }

    #[test]
    #[should_panic]
    fn should_panic_opening_bad_file() {
        open_config(&Path::new("0a82f0798dea87d1ef80140c5550768786c9c4dab6c65e392617320524688895.json"));
    }

    #[test]
    #[should_panic]
    fn test_should_panic_invalid_json() {
        let input_json = r#"
        {
            "name": "fake_model",
            "paths": [
                "./path_1/file_1",
                "./path_1/file_2",
                "./path_1/dir_1/",
                "./path_1/dir_2/"
            ],
            "licence": "",
            "author": "",
            "email": "",@
            "notes": ""
        }
        "#;
        load_config(input_json.to_string());
    }
}
