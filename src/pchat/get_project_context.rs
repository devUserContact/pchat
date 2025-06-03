use std::fs;
use std::fs::File;
use std::path::Path;
use walkdir::WalkDir;

pub fn check_if_rust_project() -> String {
    let cargo_toml_path = Path::new("./Cargo.toml");
    let mut project_context: String = String::from("");
    match File::open(cargo_toml_path) {
        Ok(_file) => {
            let cargo_toml_cont: Result<std::string::String, std::io::Error> =
                fs::read_to_string(cargo_toml_path);
            match cargo_toml_cont {
                Ok(toml_cont) => {
                    project_context.push_str(format!("{:?}", cargo_toml_path).as_str());
                    project_context.push_str(format!("```rust\n{}```\n", toml_cont).as_str());
                    project_context.push_str(get_rust_project().as_str());
                }
                Err(e) => println!("error attempting to read `Cargo.toml`: {}", e),
            }
        }
        Err(e) => {
            println!(
                "Could not file `Cargo.toml` in current directory. Move to the root directory of a rust project: {}",
                e
            );
        }
    };
    return project_context;
}
pub fn get_rust_project() -> String {
    let rust_project_path = Path::new("./src/");
    let mut project_context: String = String::from("");
    for entry in WalkDir::new(rust_project_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let metadata = fs::metadata(entry.path());
        match metadata {
            Ok(md) => {
                let file_type = md.file_type();
                if file_type.is_file() {
                    let rs_file_cont: Result<std::string::String, std::io::Error> =
                        fs::read_to_string(entry.path());
                    match rs_file_cont {
                        Ok(rs_cont) => {
                            project_context
                                .push_str(format!("{}\n", entry.path().display()).as_str());
                            project_context.push_str(format!("```rust\n{}```\n", rs_cont).as_str());
                        }
                        Err(e) => println!(
                            "error attempting to read `{}`: {}",
                            entry.path().display(),
                            e
                        ),
                    }
                }
            }
            Err(e) => println!("There was an error attempting to get metadata {}", e),
        }
    }
    return project_context;
}
