use std::collections::HashMap;
use std::io::prelude::*;
use std::fs;
use std::fs::{File};
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

use warp::{http::Uri, Filter};
use liquid;
use liquid::*;

#[tokio::main]
async fn main() {
    let compiler = construct_liquid_complier();

    // Build parser
    let parser = ParserBuilder::with_stdlib()
        .partials(compiler)
        .build()
        .unwrap();
    let parser = Arc::new(parser);

    // TODO: Remove this testing code
    let globals = object!({"empty": "empty?"});
    let template = parser.parse("{% include './_includes/test.html' %}\ntesting templating.")
        .unwrap();

    let output = template.render(&globals).unwrap();
    println!("Test liquid templating:\n{}", output);

    let template = parser.parse("{% include './_includes/test.html' %}")
        .unwrap();

    let output = template.render(&globals).unwrap();
    println!("Test nested templating:\n{}", output);

    let test_str = "---\nlayout: base\ntitle: 'a page'\n---\n<p>This is the content of the webpage!</p>".to_string();
    let test_split: Vec<&str> = test_str.splitn(3, "---\n").collect();
    println!("Testing splitn on:\n{}\nResult of splitn:\n{}\n{}\n{}", test_str, test_split[0], test_split[1], test_split[2]);

    let test_metadata = process_file_frontmatter(test_split[1].to_string());
    println!("Extracted metadata:");
    for (key, value) in &test_metadata {
        println!("{} = {}", key, value);
    }

    // end Testing code

    
    // Actual program stuff
    let template_file = move |file_path| render_file(parser.clone(), file_path);

    let string = "This is a test string to place into a filter";
    
    let index = warp::path("index.html").and(warp::fs::file("index.html"));
    let index_redirect = warp::path::end().map(|| warp::redirect(Uri::from_static("/index.html")));

    let test = warp::path("test")
        .map(|| Path::new("index.html"))
        .map(template_file);

    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let routes = warp::get().and(index.or(index_redirect).or(hello).or(test));

    warp::serve(routes)
        .run(([127,0,0,1], 3030))
        .await;
}

fn construct_liquid_complier() -> liquid::partials::EagerCompiler::<liquid::partials::InMemorySource> {
    let mut compiler = liquid::partials::EagerCompiler::<liquid::partials::InMemorySource>::empty();

    match add_files_to_compiler(&mut compiler) {
        Ok(_) => (),
        Err(_) => print!("Failed to add files to compiler.\n"),
    }

    compiler
}

fn add_files_to_compiler(compiler: &mut liquid::partials::EagerCompiler::<liquid::partials::InMemorySource>) -> std::io::Result<()> {
    let dir = Path::new("./_includes/");

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let path_str = match path.to_str() {
                    Some(s) => s,
                    None => "[error: could not get str representation of path]",
                };

                match get_file_contents(&path) {
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

/*
fn create_layout_collection() -> std::io::Result<HashMap<String, String>> {
    let mut layouts = HashMap::new();

    let dir = Path::new("./_layouts/");

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            match add_file_to_layout_collection(&mut layouts, &path) {
                Ok(_) =>,
                Err(_) => println!("Failed to add layout file to layout collection."),
            }
        }
    }

    layouts
}

fn add_file_to_layout_collection(layouts: &mut HashMap<String, String>, file_path: &std::path::Path) -> Result<(), String> {
    let mut contents = match get_file_contents(file_path) {
        Ok(c) => c,
        Err(e) => return Err(format!("Failed to add file to layout collection due to: {}", e)),
    }
}

fn get_file_metadata(file_string: String) -> HashMap<String, String> {
    if file_string.starts_with("---\n") {
        // Split on --- twice
        let split: Vec<String> = file_string.splitn(3, "---\n").collect();

        // Confirm it was twice
        if split.len() != 3 {
            let metadata = HashMap::new();
            metadata.insert("content".to_string(), file_string.clone());
        }

        // Process first split portion for metadata
        let metadata = process_file_metadata(split[1].clone());

        // Insert last bit as content
        metadata.insert("content".to_string(), split[2].clone());

        metadata
    }
    else {
        let mut metadata = HashMap::new();
        metadata.insert("content".to_string(), file_string.clone());

        metadata
    }
}
*/

fn process_file_frontmatter(metadata_string: String) -> HashMap<String, String> {
    let mut metadata = HashMap::new();

    for line in metadata_string.split("\n").collect::<Vec<&str>>() {
        let line = line.to_string().trim().to_string();

        let split: Vec<&str> = line.splitn(2, ":").collect();

        let key = split[0].trim();
        if key.is_empty() { break; }

        if (split.len() > 1) {
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

fn render_file(liquid_parser: Arc<liquid::Parser>, template_path: &std::path::Path/*, globals, &Object*/) -> impl warp::Reply {
    let globals = object!({"empty": "empty?"});

    let template = liquid_parser.parse_file(template_path)
        .unwrap();

    let output = template.render(&globals).unwrap();
    println!("Test liquid templating:\n{}", output);

    warp::reply::html(output)
}
