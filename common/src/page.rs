use crate::count_start_spaces;
use crate::my_uuid;
use crate::read_path_buf;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;
use uuid::Uuid;
trait Element: Debug {
    fn add_child(&mut self, child: Box<dyn Element>) -> *mut dyn Element;
    fn to_md(&self, depth: usize) -> String;
}
#[derive(Debug)]
pub struct Title {
    value: String,
    childs: Vec<Box<dyn Element>>,
}
impl Element for Title {
    fn add_child(&mut self, child: Box<dyn Element>) -> *mut dyn Element {
        self.childs.push(child);
        let l = self.childs.len() - 1;
        &mut *self.childs[l]
    }

    fn to_md(&self, depth: usize) -> String {
        let mut strs = Vec::new();
        strs.push(String::new());
        strs.push(format!("{} {}", "#".repeat(depth + 1), self.value));
        for c in &self.childs {
            strs.push(c.to_md(depth + 1));
        }
        strs.join("\n")
    }
}
#[derive(Serialize, Debug)]
pub struct Page {
    #[serde(with = "my_uuid")]
    pub id: Uuid,
    pub title: String,
    pub path: String,
    #[serde(skip_serializing)]
    pub childs: Vec<Box<dyn Element>>,
}
impl Element for Page {
    fn add_child(&mut self, child: Box<dyn Element>) -> *mut dyn Element {
        self.childs.push(child);
        let l = self.childs.len() - 1;
        &mut *self.childs[l]
    }

    fn to_md(&self, depth: usize) -> String {
        let mut strs = Vec::new();
        strs.push(format!("# {}", self.title));
        for c in &self.childs {
            strs.push(c.to_md(depth + 1));
        }
        strs.push(String::new());
        strs.join("\n")
    }
}
impl Page {
    pub fn from_path(path_long: PathBuf, path_short: String) -> Page {
        let page_md_str = read_path_buf(path_long);
        let mut title = None;
        let mut parents: HashMap<usize, *mut dyn Element> = HashMap::new();
        let mut page = Page {
            id: Uuid::new_v4(),
            path: path_short,
            title: String::new(),
            childs: Vec::new(),
        };
        parents.insert(0, &mut page);
        let mut last_parent_level = 0;
        for (_, line) in page_md_str
            .split('\n')
            .collect::<Vec<&str>>()
            .iter()
            .enumerate()
        {
            let words: Vec<&str> = line.split(' ').collect::<Vec<&str>>();
            let start_space_count = count_start_spaces(&words).min(line.len());
            let words_2 = &words[start_space_count..];
            match words_2[0] {
                "#" => {
                    let new_title = line[2..].to_string();
                    match title {
                        None => {
                            title = Some(new_title);
                        }
                        Some(value) => {
                            panic!("Title already set: old={:?} new={}", value, new_title);
                        }
                    }
                }
                "##" => {
                    last_parent_level = 1;
                    let par = *(parents.get_mut(&0).unwrap());
                    unsafe {
                        let child = (*par).add_child(Box::new(Title {
                            value: line[3..].to_string(),
                            childs: Vec::new(),
                        }));
                        parents.insert(1, child);
                    }
                }
                "###" => {
                    last_parent_level = 2;
                    let par = *(parents.get_mut(&1).unwrap());
                    unsafe {
                        let child = (*par).add_child(Box::new(Title {
                            value: line[4..].to_string(),
                            childs: Vec::new(),
                        }));
                        parents.insert(2, child);
                    }
                }
                _ => {}
            }
        }
        page.title = title.unwrap();
        println!("------");
        println!("{}", page.to_md(0));
        page
    }
}
