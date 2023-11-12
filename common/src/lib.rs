use crate::element::*;
use glob::glob;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;
mod element;
use std::path::PathBuf;
mod page;
#[cfg(test)]
mod test;
mod to_html;
mod to_md;
use page::Page;
// pub struct NetworkTask {
//     pub created_by: SocketAddr,
//     pub title: String,
// }
pub fn read(path: &str) -> String {
    fs::read_to_string(path).expect("Should have been able to read the file")
}
pub fn read_path_buf(path: PathBuf) -> String {
    read(&path.into_os_string().into_string().unwrap())
}
pub fn write(path: &str, content: &str) {
    write!(File::create(path).unwrap(), "{content}").unwrap();
}
pub fn md_file_to_json_file(path_in: &str, path_out: &str) {
    write(path_out, &md_to_json(&read(path_in)));
}
pub fn json_file_to_md_file(path_in: &str, path_out: &str) {
    write(path_out, &json_to_md(&read(path_in)));
}
pub fn count_start_spaces(words: &Vec<&str>) -> usize {
    for (i, word) in words.iter().enumerate() {
        if !word.is_empty() {
            return i;
        }
    }
    words.len()
}
pub fn json_file_to_html_file(path_in: &str, path_out: &str) {
    write(path_out, &json_to_html(&read(path_in)));
}
pub fn json_to_html(json_str: &str) -> String {
    let page: Element = serde_json::from_str(json_str).unwrap();
    page.to_html(0, 0)
}
pub fn json_to_md(json_str: &str) -> String {
    let page: Element = serde_json::from_str(json_str).unwrap();
    page.to_md(0, 0)
}
pub fn md_to_page(md_str: &str) -> Element {
    let mut page = Element {
        data: ElementData::Page(PageElement {
            title: None,
            comments: None,
        }),
        childs: Vec::new(),
    };
    let mut parents: HashMap<usize, *mut Element> = HashMap::new();
    let mut parents_by_indent: HashMap<usize, *mut Element> = HashMap::new();
    let page_pointer: *mut Element = &mut page;
    parents.insert(0, page_pointer);
    parents_by_indent.insert(0, page_pointer);
    let mut last_comment_parent_id = 0;
    let mut mode = "page";
    let mut parent_comment_by_indent: HashMap<usize, *mut Element> = HashMap::new();
    for (_, line) in md_str.split('\n').collect::<Vec<&str>>().iter().enumerate() {
        let words: Vec<&str> = line.split(' ').collect::<Vec<&str>>();
        let start_space_count = count_start_spaces(&words).min(line.len());
        let words_2 = &words[start_space_count..];
        match *line {
            "# Comments" => {
                if let ElementData::Page(ref mut page_data) = page.data {
                    match page_data.comments {
                        Some(_) => {
                            panic!("comments already set");
                        }
                        None => {
                            page_data.comments = Some(Box::new(Element {
                                data: ElementData::CommentSection,
                                childs: Vec::new(),
                            }));
                            let ptr: *mut Element =
                                &mut *page_data.comments.as_mut().unwrap().as_mut();
                            parent_comment_by_indent.insert(0, ptr);
                        }
                    }
                }
                mode = "comments";
                continue;
            }
            _ => {}
        }
        match mode {
            "page" => match words_2[0] {
                "#" => {
                    if let ElementData::Page(ref mut page_data) = page.data {
                        match page_data.title {
                            Some(_) => {
                                panic!("title already set");
                            }
                            None => {
                                page_data.title = Some(line[2..].to_string());
                            }
                        }
                    }
                }
                "##" => unsafe {
                    let parent = &mut (*parents[&0]);
                    parent.childs.push(Element {
                        data: ElementData::Title(Title {
                            txt: line[start_space_count + 3..].to_string(),
                        }),
                        childs: Vec::new(),
                    });
                    let childs = &mut parent.childs;
                    let l = childs.len() - 1;
                    let new_child_ptr: *mut Element = &mut childs[l];
                    parents.insert(1, new_child_ptr);
                    parents_by_indent.insert(0, new_child_ptr);
                },
                "###" => unsafe {
                    let parent = &mut (*parents[&1]);
                    parent.childs.push(Element {
                        data: ElementData::Title(Title {
                            txt: line[start_space_count + 4..].to_string(),
                        }),
                        childs: Vec::new(),
                    });
                    let childs = &mut parent.childs;
                    let l = childs.len() - 1;
                    let new_child_ptr: *mut Element = &mut childs[l];
                    parents.insert(2, new_child_ptr);
                    parents_by_indent.insert(0, new_child_ptr);
                },
                "" => {}
                "-" => unsafe {
                    let indent = start_space_count / 4;
                    let parent = &mut (*parents_by_indent[&indent]);
                    parent.childs.push(Element {
                        data: ElementData::ListElement(ListElement {
                            txt: line[start_space_count + 2..].to_string(),
                        }),
                        childs: Vec::new(),
                    });
                    let childs = &mut parent.childs;
                    let l = childs.len() - 1;
                    let new_child_ptr: *mut Element = &mut childs[l];
                    parents_by_indent.insert(indent + 1, new_child_ptr);
                },
                _ => unsafe {
                    let indent = start_space_count / 4;
                    let parent = &mut (*parents_by_indent[&indent]);
                    parent.childs.push(Element {
                        data: ElementData::Text(Text {
                            txt: line[start_space_count..].to_string(),
                        }),
                        childs: Vec::new(),
                    });
                    let childs = &mut parent.childs;
                    let l = childs.len() - 1;
                    let new_child_ptr: *mut Element = &mut childs[l];
                    parents_by_indent.insert(indent + 1, new_child_ptr);
                },
            },
            "comments" => match words_2[0] {
                "-" => unsafe {
                    let indent = start_space_count / 2;
                    let parent = &mut (*parent_comment_by_indent[&indent]);
                    let mut line = None;
                    let user;
                    if indent == 0 {
                        line = Some(words_2[1].replace('l', ""));
                        user = words_2[2].replace('@', "");
                    } else {
                        user = words_2[1].replace('@', "");
                    }
                    parent.childs.push(Element {
                        data: ElementData::Comment(Comment {
                            text: Vec::new(),
                            user,
                            line,
                        }),
                        childs: Vec::new(),
                    });
                    let childs = &mut parent.childs;
                    let l = childs.len() - 1;
                    let new_child_ptr: *mut Element = &mut childs[l];
                    last_comment_parent_id = indent + 1;
                    parent_comment_by_indent.insert(last_comment_parent_id, new_child_ptr);
                },
                _ => unsafe {
                    let parent = &mut (*parent_comment_by_indent[&last_comment_parent_id]);
                    if let ElementData::Comment(ref mut comment) = parent.data {
                        comment.text.push(line[start_space_count..].to_string());
                    }
                },
            },
            _ => {}
        }
    }
    page
}
pub fn md_to_json(md_str: &str) -> String {
    serde_json::to_string_pretty(&md_to_page(md_str)).unwrap()
}
pub enum TaskStatus {
    New,
    Doing,
    Todo,
    Backlog,
    Done,
    WontDo,
    Duplicate(Uuid),
}
pub struct Task {
    title: String,
    description: String,
    lead: Option<Uuid>,
    status: TaskStatus,
    completion: Option<f32>,
}
pub struct Organization {
    id: Uuid,
    id_txt: String,
    name: String,
    pages: HashMap<Uuid, Page>,
    tasks: HashMap<u128, Task>,
}
mod my_uuid {
    use serde::Serialize;
    use serde::Serializer;
    use uuid::Uuid;
    pub fn serialize<S>(val: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        val.to_string().serialize(serializer)
    }
    // pub fn deserialize<'de, D>(deserializer: D) -> Result<UUID, D::Error>
    // where
    //     D: Deserializer<'de>,
    // {
    //     let val: &str = Deserialize::deserialize(deserializer)?;
    //     UUID::from_str(val).map_err(D::Error::custom)
    // }
}
#[derive(Debug, Serialize, Clone)]
pub struct OrganizationSettings {
    #[serde(with = "my_uuid")]
    id: Uuid,
    name: String,
    id_txt: String,
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
                    let page = Page::from_path(path_md, path_md_short);
                    org.pages.insert(page.id, page);
                }
                Err(e) => println!("{:?}", e),
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
        for (_, page) in &self.pages {
            write(
                &format!("{path}/{}.json", page.path),
                &serde_json::to_string_pretty(&page).unwrap(),
            );
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
pub struct Data {
    organizations: HashMap<Uuid, Organization>,
}
impl Data {
    pub fn export(&self, path: &str) {
        for (_, org) in &self.organizations {
            org.export(path);
        }
    }
}
pub fn import_organizations(paths: &[&str]) -> Data {
    let mut data = Data {
        organizations: HashMap::new(),
    };
    for path in paths {
        let org = Organization::new(path);
        data.organizations.insert(org.id, org);
    }
    data
}
