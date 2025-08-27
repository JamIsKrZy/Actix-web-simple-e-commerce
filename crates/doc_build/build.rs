use std::fs;
use std::path::Path;

use app::ApiDoc;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

fn main() {
    // 1. Generate OpenAPI JSON
    let open_api = ApiDoc::openapi()
        .to_pretty_json()
        .expect("Failed to generate OpenAPI JSON");

    // Target output directory: ../../doc
    let out_dir = Path::new("../../doc");

    // Ensure target directory exists
    fs::create_dir_all(out_dir).expect("Failed to create ../../doc directory");

    // Write OpenAPI JSON to ../../doc/doc.json
    let json_path = out_dir.join("doc.json");
    fs::write(&json_path, &open_api).expect("Failed to write doc.json");

    // 2. Generate static HTML with RapiDoc pointing to doc.json
    let rapidoc_html = RapiDoc::new("./doc.json") // relative path inside ../../doc
        .to_html();

    // 3. Save HTML as ../../doc/index.html
    let index_path = out_dir.join("index.html");
    fs::write(&index_path, rapidoc_html).expect("Failed to write index.html");

    println!(
        "✅ Documentation generated at {}/index.html",
        out_dir.display()
    );
}
