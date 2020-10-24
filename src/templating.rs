use std::collections::HashMap;
use std::borrow::Cow;
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

    pub fn render_file(&self, template_path: &std::path::Path/*, globals: &Object*/) -> String {
        let globals = object!({"empty": "empty?"});

        let file_metadata = match Templator::get_file_contents(template_path) {
            Ok(c) => Templator::get_file_metadata(&c),
            Err(e) => return format!("Could not get file metadata with error: {}", e).to_string(),
        };

        let output = match self.process_metadata(&file_metadata, &globals) {
            Ok(s) => s,
            Err(e) => return format!("Failed to process metadata with error: {}", e).to_string()
        };

        output
    }

    fn process_metadata(&self, file_metadata: &HashMap<String, String>, globals: &Object) -> Result<String, String> {
        let mut globals = Templator::merge_globals(&file_metadata, globals);

        let template = self.parser.parse(file_metadata.get("content").unwrap()).unwrap();
        let output = template.render(&globals).unwrap();

        let output = match file_metadata.get(&"layout".to_string()) {
            Some(s) => {
                let layout_metadata = match self.layout_collection.get(s) {
                    Some(metadata) => metadata,
                    None => return Err(format!("Layout {} could not be found.", s)),
                };

                Templator::add_content_to_global(&mut globals, output);

                self.process_metadata(&layout_metadata, &globals).unwrap()
            },
            None => output,
        };

        Ok(output)
    }

    fn merge_globals(file_metadata: &HashMap<String, String>, globals: &Object) -> Object {
        let mut new_globals = globals.clone();

        for (key, value) in file_metadata.iter() {
            let k = model::scalar!(key).to_kstr().into_owned();
            let v = model::Value::from(model::value!(value).as_scalar().unwrap().into_owned());

            new_globals.insert(k, v);
        }

        new_globals
    }

    fn add_content_to_global(globals: &mut Object, content: String) {
        let k = model::scalar!("Content").to_kstr().into_owned();
        let v = model::Value::from(model::value!(content).as_scalar().unwrap().into_owned());

        globals.insert(k, v);
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
                    let file_name = match Templator::path_to_file_name(&path) {
                        Some(s) => s,
                        None => continue,
                    };

                    match Templator::get_file_contents(&path) {
                        Ok(contents) => {
                            compiler.add(file_name, contents);
                            ()
                        },
                        Err(e) => println!("{}", e),
                    }
                }
            }
        }

        Ok(())
    }

    fn path_to_file_name(path: &std::path::Path) -> Option<Cow<'_, str>> {
        match path.file_name() {
            Some(os) => Some(os.to_string_lossy()),
            None => None,
        }
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
    fn merge_globals_empty_metadata_empty_globals() {
        let metadata = HashMap::<String, String>::new();
        let globals = object!({});
        let expected = object!({});

        let actual = Templator::merge_globals(&metadata, &globals);
        assert_eq!(expected, actual);
    }

    #[test]
    fn merge_globals_empty_globals() {
        let mut metadata = HashMap::<String, String>::new();
        metadata.insert("metadata".to_string(), "test".to_string());
        let globals = object!({});
        let expected = object!({ "metadata": "test" });

        let actual = Templator::merge_globals(&metadata, &globals);
        assert_eq!(expected, actual);
    }

    #[test]
    fn merge_globals_empty_metadata() {
        let metadata = HashMap::<String, String>::new();
        let globals = object!({ "global": "test" });
        let expected = object!({ "global": "test" });

        let actual = Templator::merge_globals(&metadata, &globals);
        assert_eq!(expected, actual);
    }

    #[test]
    fn merge_globals_nonoverlaping_metadata_and_globals() {
        let mut metadata = HashMap::<String, String>::new();
        metadata.insert("metadata".to_string(), "test".to_string());

        let globals = object!({ "global": "test" });
        let expected = object!({ "metadata": "test", "global": "test" });

        let actual = Templator::merge_globals(&metadata, &globals);
        assert_eq!(expected, actual);
    }

    #[test]
    fn merge_globals_overlaping_metadata_and_globals() {
        let mut metadata = HashMap::<String, String>::new();
        metadata.insert("metadata".to_string(), "test".to_string());
        metadata.insert("overlap".to_string(), "metadata".to_string());

        let globals = object!({ "global": "test", "overlap": "globals" });
        let expected = object!({ "global": "test", "metadata": "test", "overlap": "metadata" });

        let actual = Templator::merge_globals(&metadata, &globals);
        assert_eq!(expected, actual);
    }

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

    #[test]
    fn get_file_metadata_valid_file() {
        let mut expected_hashmap = HashMap::new();
        expected_hashmap.insert("testing".to_string(), "values".to_string());
        expected_hashmap.insert("content".to_string(), "File contents go\nhere.".to_string());

        let file_content = "---\ntesting: values\n---\nFile contents go\nhere.".to_string();
        let actual_hashmap = Templator::get_file_metadata(&file_content);

        assert_eq!(expected_hashmap, actual_hashmap);
    }

    #[test]
    fn get_file_metadata_valid_file_with_empty_frontmatter() {
        let mut expected_hashmap = HashMap::new();
        expected_hashmap.insert("content".to_string(), "File contents go\nhere.".to_string());

        let file_content = "---\n---\nFile contents go\nhere.".to_string();
        let actual_hashmap = Templator::get_file_metadata(&file_content);

        assert_eq!(expected_hashmap, actual_hashmap);
    }

    #[test]
    fn get_file_metadata_valid_file_with_no_frontmatter() {
        let mut expected_hashmap = HashMap::new();
        expected_hashmap.insert("content".to_string(), "File contents go\nhere.".to_string());

        let file_content = "File contents go\nhere.".to_string();
        let actual_hashmap = Templator::get_file_metadata(&file_content);

        assert_eq!(expected_hashmap, actual_hashmap);
    }

    #[test]
    fn get_file_metadata_valid_file_with_empty_content() {
        let mut expected_hashmap = HashMap::new();
        expected_hashmap.insert("testing".to_string(), "values".to_string());
        expected_hashmap.insert("content".to_string(), "".to_string());

        let file_content = "---\ntesting: values\n---\n".to_string();
        let actual_hashmap = Templator::get_file_metadata(&file_content);

        assert_eq!(expected_hashmap, actual_hashmap);
    }

    #[test]
    fn get_file_metadata_valid_file_with_empty_content_no_trailing_newline() {
        let mut expected_hashmap = HashMap::new();
        expected_hashmap.insert("testing".to_string(), "values".to_string());
        expected_hashmap.insert("content".to_string(), "".to_string());

        let file_content = "---\ntesting: values\n---".to_string();
        let actual_hashmap = Templator::get_file_metadata(&file_content);

        assert_eq!(expected_hashmap, actual_hashmap);
    }

    #[test]
    fn get_file_metadata_empty_file() {
        let mut expected_hashmap = HashMap::new();
        expected_hashmap.insert("content".to_string(), "".to_string());

        let file_content = "".to_string();
        let actual_hashmap = Templator::get_file_metadata(&file_content);

        assert_eq!(expected_hashmap, actual_hashmap);
    }

    #[test]
    fn get_file_metadata_valid_file_with_three_frontmatter_seperators() {
        let mut expected_hashmap = HashMap::new();
        expected_hashmap.insert("testing".to_string(), "values".to_string());
        expected_hashmap.insert("content".to_string(), "File contents go\n---\nhere.".to_string());

        let file_content = "---\ntesting: values\n---\nFile contents go\n---\nhere.".to_string();
        let actual_hashmap = Templator::get_file_metadata(&file_content);

        assert_eq!(expected_hashmap, actual_hashmap);
    }

    #[test]
    fn get_file_metadata_invalid_file_with_one_frontmatter_seperator_at_start() {
        let mut expected_hashmap = HashMap::new();
        expected_hashmap.insert("content".to_string(), "---\nFile contents go\nhere.".to_string());

        let file_content = "---\nFile contents go\nhere.".to_string();
        let actual_hashmap = Templator::get_file_metadata(&file_content);

        assert_eq!(expected_hashmap, actual_hashmap);
    }

    #[test]
    fn get_file_metadata_valid_file_with_one_frontmatter_seperator_in_middle() {
        let mut expected_hashmap = HashMap::new();
        expected_hashmap.insert("content".to_string(), "File contents go\n---\nhere.".to_string());

        let file_content = "File contents go\n---\nhere.".to_string();
        let actual_hashmap = Templator::get_file_metadata(&file_content);

        assert_eq!(expected_hashmap, actual_hashmap);
    }

    #[test]
    fn read_file_contents_valid_file() {
        let expected = "This file is valid.\n";

        let path = Path::new("./test/files/basic/valid.txt");
        let result = Templator::get_file_contents(&path);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn read_file_contents_nonexistant_file() {
        let path = Path::new("./this/is/not/a/valid/path.txt");
        let result = Templator::get_file_contents(&path);
        assert!(result.is_err());
        assert!(result.expect_err("Error did not contain expected substring.").contains("Failed to open file"));
    }
}
