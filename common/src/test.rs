use crate::import_organizations;
use crate::json_file_to_html_file;
use crate::json_file_to_md_file;
use crate::md_file_to_json_file;
use crate::Data;
// #[test]
// fn md_to_json_to_md_and_html() {
//     md_file_to_json_file("./src/example-base.md", "./src/example-generated.json");
//     json_file_to_md_file("./src/example-generated.json", "./src/example-generated.md");
//     json_file_to_html_file(
//         "./src/example-generated.json",
//         "./src/example-generated.html",
//     );
// }
#[test]
fn import_export() {
    let data: Data = import_organizations(&[
        "../examples/in/tasty_bakery",
        "../examples/in/yummy_for_charity",
    ]);
    data.export("../examples/out");
}
