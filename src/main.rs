use std::io::prelude::*;
use std::fs;
use std::fs::{File, DirEntry};
use std::path::Path;

use warp::{http::Uri, Filter};
use liquid;

#[tokio::main]
async fn main() {
    let compiler = construct_liquid_complier();
    
    let index = warp::path("index.html").and(warp::fs::file("index.html"));
    let index_redirect = warp::path::end().map(|| warp::redirect(Uri::from_static("/index.html")));

    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let routes = warp::get().and(index.or(index_redirect).or(hello));

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
