use crate::my_uuid;
use crate::page::Element;
use crate::task::Task;
use crate::write;
use crate::Page;
use glob::glob;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use uuid::Uuid;
pub struct Organization {
    pub id: Uuid,
    pub id_txt: String,
    pub name: String,
    pub pages: HashMap<Uuid, Page>,
    pub tasks: HashMap<Uuid, Task>,
}
#[derive(Debug, Serialize, Clone)]
pub struct OrganizationSettings {
    #[serde(with = "my_uuid")]
    pub id: Uuid,
    pub name: String,
    pub id_txt: String,
}
impl Organization {
    pub fn new(path: &str) -> Organization {
        let path_splitted: Vec<_> = path.split('/').collect();
        let n = path_splitted.len() - 1;
        let mut org = Organization {
            name: path_splitted[n].to_string(),
            id_txt: path_splitted[n].to_string(),
            pages: HashMap::new(),
            tasks: HashMap::new(),
            id: Uuid::new_v4(),
        };
        for entry in glob(&format!("{path}/**/*.md")).expect("Failed to read glob pattern") {
            match entry {
                Ok(path_md) => {
                    let path_long = path_md.clone().into_os_string().into_string().unwrap();
                    let path_md_short = path_long.replace(path, "").replace(".md", "");
                    let page = Page::from_path(path_md, path_md_short, &mut org.tasks);
                    org.pages.insert(page.id, page);
                }
                Err(e) => println!("{e:?}"),
            }
        }
        org
    }

    pub fn export(&self, path: &str) {
        fs::create_dir_all(&format!("{path}/{}/settings/", self.name)).unwrap();
        write(
            &format!("{path}/{}/settings/organization.json", self.name),
            &serde_json::to_string_pretty(&self.settings()).unwrap(),
        );
        for page in self.pages.values() {
            write(
                &format!("{path}/{}/{}.json", self.name, page.path),
                &serde_json::to_string_pretty(&page).unwrap(),
            );
            let md_str = page.to_md(0, 0, &self.tasks);
            write(&format!("{path}/{}/{}.md", self.name, page.path), &md_str);
            // println!("------");
            // println!("{md_str}");
        }
    }

    pub fn settings(&self) -> OrganizationSettings {
        OrganizationSettings {
            id: self.id,
            name: self.name.clone(),
            id_txt: self.id_txt.clone(),
        }
    }
}
