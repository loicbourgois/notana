use crate::import_organizations;
use crate::read;
use crate::Data;
use glob::glob;
fn diff() {
    for entry in glob(&format!("../examples/in/**/*.md")).expect("Failed to read glob pattern") {
        match entry {
            Ok(path_md) => {
                let path_in = path_md.clone().into_os_string().into_string().unwrap();
                let path_out = path_in.replace("../examples/in/", "../examples/out/");
                let str_in = read(&path_in);
                let str_out = read(&path_out);
                similar_asserts::assert_eq!(str_in, str_out);
            }
            Err(e) => println!("{e:?}"),
        }
    }
}
#[test]
fn import_export() {
    let data: Data = import_organizations(&[
        "../examples/in/tasty_bakery",
        "../examples/in/yummy_for_charity",
        "../examples/in/notana",
    ]);
    data.export("../examples/out");
    diff();
}
