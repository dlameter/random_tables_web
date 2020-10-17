use std::path::Path;
use std::sync::Arc;

use warp::{http::Uri, Filter};
use liquid;
use liquid::*;

mod templating;

#[tokio::main]
async fn main() {
    let compiler = templating::construct_liquid_complier();

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
    /*
    let test_split: Vec<&str> = test_str.splitn(3, "---\n").collect();
    println!("Testing splitn on:\n{}\nResult of splitn:\n{}\n{}\n{}", test_str, test_split[0], test_split[1], test_split[2]);
    */

    let test_metadata = templating::get_file_metadata(&test_str);
    println!("Extracted metadata:");
    for (key, value) in &test_metadata {
        println!("{} = {}", key, value);
    }

    let layouts = templating::create_layout_collection().unwrap();
    for (layout_name, layout_metadata) in layouts.iter() {
        println!("Metadata of layout {}:", layout_name);
        for (key, value) in layout_metadata {
            println!("{} = {}", key, value);
        }
    }

    // end Testing code

    
    // Actual program stuff
    let template_file = move |file_path| templating::render_file(parser.clone(), file_path);

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
