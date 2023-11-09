use crate::json_file_to_html_file;
use crate::json_file_to_md_file;
use crate::md_file_to_json_file;
#[test]
fn md_to_json_to_md() {
    md_file_to_json_file("./src/example-base.md", "./src/example-generated.json");
    json_file_to_md_file("./src/example-generated.json", "./src/example-generated.md");
    json_file_to_html_file(
        "./src/example-generated.json",
        "./src/example-generated.html",
    );
}
