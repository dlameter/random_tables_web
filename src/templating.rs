use std::collections::HashMap;
use std::io::prelude::*;
use std::fs;
use std::fs::{File};
use std::path::Path;

use liquid;
use liquid::*;

pub struct Templator {
    layout_collection: HashMap<String, HashMap<String, String>>,
    parser: liquid::Parser,
}

impl Templator {
    pub fn new(layout_dir: String, include_dir: String) -> Templator {
        let layout_collection = Templator::create_layout_collection(&layout_dir).unwrap();
        let parser = Templator::construct_liquid_parser(&include_dir);

        Templator {
            layout_collection,
            parser,
        }
    }

    pub fn render_file(&self, template_path: &std::path::Path/*, globals, &Object*/) -> impl warp::Reply {
        let globals = object!({"empty": "empty?"});

        let template = self.parser.parse_file(template_path)
            .unwrap();

        let output = template.render(&globals).unwrap();
        println!("Test liquid templating:\n{}", output);

        warp::reply::html(output)
    }

    fn construct_liquid_parser(include_dir: &String) -> liquid::Parser {
        ParserBuilder::with_stdlib()
            .partials(Templator::construct_liquid_complier(include_dir))
            .build()
            .unwrap()
    }

    fn construct_liquid_complier(include_dir: &String) -> liquid::partials::EagerCompiler::<liquid::partials::InMemorySource> {
        let mut compiler = liquid::partials::EagerCompiler::<liquid::partials::InMemorySource>::empty();

        match Templator::add_files_to_compiler(&mut compiler, include_dir) {
            Ok(_) => (),
            Err(_) => print!("Failed to add files to compiler.\n"),
        }

        compiler
    }

    fn add_files_to_compiler(compiler: &mut liquid::partials::EagerCompiler::<liquid::partials::InMemorySource>, include_dir: &String) -> std::io::Result<()> {
        let dir = Path::new(include_dir);

        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    let path_str = match path.to_str() {
                        Some(s) => s,
                        None => "[error: could not get str representation of path]",
                    };

                    match Templator::get_file_contents(&path) {
                        Ok(contents) => {
                            compiler.add(path_str, contents);
                            ()
                        },
                        Err(e) => println!("{}", e),
                    }
                }
            }
        }

        Ok(())
    }

    fn create_layout_collection(layout_dir: &String) -> std::io::Result<HashMap<String, HashMap<String, String>>> {
        let mut layouts = HashMap::new();

        let dir = Path::new(layout_dir);

        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();

                match Templator::add_file_to_layout_collection(&mut layouts, &path) {
                    Ok(_) => (),
                    Err(e) => println!("Failed to add layout file to layout collection. Error text: {}", e),
                }
            }
        }

        Ok(layouts)
    }

    fn add_file_to_layout_collection(layouts: &mut HashMap<String, HashMap<String,String>>, file_path: &std::path::Path) -> Result<(), String> {
        let file_stem: String = match file_path.file_stem().unwrap().to_os_string().into_string() {
            Ok(s) => s,
            Err(os) => return Err(format!("File stem is not a valid Unicode string. Lossy result is: {}", os.to_string_lossy())),
        };

        let contents = match Templator::get_file_contents(file_path) {
            Ok(c) => c,
            Err(e) => return Err(format!("Failed to add file to layout collection due to: {}", e)),
        };

        layouts.insert(file_stem, Templator::get_file_metadata(&contents));
        
        Ok(())
    }

    fn get_file_metadata(file_string: &String) -> HashMap<String, String> {
        if file_string.starts_with("---\n") {
            // Split on --- twice
            let split: Vec<&str> = file_string.splitn(3, "---\n").collect();

            // Confirm it was twice
            if split.len() != 3 {
                let mut metadata = HashMap::new();
                metadata.insert("content".to_string(), file_string.clone());

                metadata
            }
            else {
                // Process first split portion for metadata
                let mut metadata = Templator::process_frontmatter(split[1].to_string().clone());

                // Insert last bit as content
                metadata.insert("content".to_string(), split[2].to_string().clone());

                metadata
            }
        }
        else {
            let mut metadata = HashMap::new();
            metadata.insert("content".to_string(), file_string.clone());

            metadata
        }
    }

    fn process_frontmatter(metadata_string: String) -> HashMap<String, String> {
        let mut metadata = HashMap::new();

        for line in metadata_string.split("\n").collect::<Vec<&str>>() {
            let line = line.to_string().trim().to_string();

            let split: Vec<&str> = line.splitn(2, ":").collect();

            let key = split[0].trim();
            if key.is_empty() { continue; }

            if split.len() > 1 {
                let value = split[1].trim();
                metadata.insert(key.to_string(), value.to_string());
            }
            else {
                metadata.insert(key.to_string(), "".to_string());
            }
        }

        metadata
    }

    fn get_file_contents(path: &std::path::Path) -> Result<String, String> {
        let path_str = match path.to_str() {
            Some(s) => s,
            None => "[error: could not get str representation of path]",
        };

        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Err(format!("Failed to open file: {}", path_str)),
        };

        let mut contents = String::new();

        match file.read_to_string(&mut contents) {
            Ok(_) => {
                Ok(contents)
            },
            Err(_) => Err(format!("Failed to read the contents of file: {}", path_str)),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_frontmatter() {
        let mut expected_hashmap = HashMap::new();
        expected_hashmap.insert("testing".to_string(), "values".to_string());
        expected_hashmap.insert("layout".to_string(), "base".to_string());

        let file_content = "testing: values\nlayout: base\n".to_string();
        let actual_hashmap = Templator::process_frontmatter(file_content);

        assert_eq!(expected_hashmap, actual_hashmap);
    }

    #[test]
    fn valid_frontmatter_with_blank_line() {
        let mut expected_hashmap = HashMap::new();
        expected_hashmap.insert("testing".to_string(), "values".to_string());
        expected_hashmap.insert("layout".to_string(), "base".to_string());

        let file_content = "testing: values\n\nlayout: base\n".to_string();
        let actual_hashmap = Templator::process_frontmatter(file_content);

        assert_eq!(expected_hashmap, actual_hashmap);
    }

    #[test]
    fn empty_frontmatter() {
        let expected_hashmap = HashMap::new();

        let file_content = "".to_string();
        let actual_hashmap = Templator::process_frontmatter(file_content);

        assert_eq!(expected_hashmap, actual_hashmap);
    }

    #[test]
    fn blank_line_frontmatter() {
        let expected_hashmap = HashMap::new();

        let file_content = "\n".to_string();
        let actual_hashmap = Templator::process_frontmatter(file_content);

        assert_eq!(expected_hashmap, actual_hashmap);
    }
}
