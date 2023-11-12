use crate::count_start_spaces;
use crate::my_uuid;
use crate::read_path_buf;
// use crate::Task;
// use crate::TaskStatus;
#[derive(Debug)]
pub enum TaskStatus {
    New,
    //     Doing,
    //     Todo,
    //     Backlog,
    Done,
    //     WontDo,
    //     Duplicate(Uuid),
}
#[derive(Debug)]
pub struct Task {
    title: String,
    childs: Vec<Box<dyn Element>>,
    // description: String,
    // lead: Option<Uuid>,
    status: TaskStatus,
    // completion: Option<f32>,
}
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;
use uuid::Uuid;
pub trait Element: Debug {
    fn add_child(&mut self, child: Box<dyn Element>) -> *mut dyn Element;
    fn to_md(&self, depth: usize, indent: usize) -> String;
}
#[derive(Debug)]
pub struct Title {
    value: String,
    childs: Vec<Box<dyn Element>>,
}
impl Element for Task {
    fn add_child(&mut self, child: Box<dyn Element>) -> *mut dyn Element {
        self.childs.push(child);
        let l = self.childs.len() - 1;
        &mut *self.childs[l]
    }

    fn to_md(&self, depth: usize, indent: usize) -> String {
        let mut strs = Vec::new();
        let status_str = match self.status {
            TaskStatus::Done => "x",
            TaskStatus::New => " ",
        };
        strs.push(format!(
            "{}- [{status_str}] {}",
            "    ".repeat(indent),
            self.title
        ));
        for c in &self.childs {
            strs.push(c.to_md(depth + 1, indent + 1));
        }
        strs.join("\n")
    }
}
impl Element for Title {
    fn add_child(&mut self, child: Box<dyn Element>) -> *mut dyn Element {
        self.childs.push(child);
        let l = self.childs.len() - 1;
        &mut *self.childs[l]
    }

    fn to_md(&self, depth: usize, _indent: usize) -> String {
        let mut strs = Vec::new();
        strs.push(String::new());
        strs.push(format!("{} {}", "#".repeat(depth + 1), self.value));
        for c in &self.childs {
            strs.push(c.to_md(depth + 1, 0));
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
    childs: Vec<Box<dyn Element>>,
}
impl Element for Page {
    fn add_child(&mut self, child: Box<dyn Element>) -> *mut dyn Element {
        self.childs.push(child);
        let l = self.childs.len() - 1;
        &mut *self.childs[l]
    }

    fn to_md(&self, depth: usize, _indent: usize) -> String {
        let mut strs = Vec::new();
        strs.push(format!("# {}", self.title));
        for c in &self.childs {
            strs.push(c.to_md(depth + 1, 0));
        }
        strs.push(String::new());
        strs.join("\n")
    }
}
impl Page {
    pub fn from_path(path_long: PathBuf, path_short: String) -> Page {
        let page_md_str = read_path_buf(path_long);
        let mut title = None;
        let mut parents_by_level: HashMap<usize, *mut dyn Element> = HashMap::new();
        let mut parents_by_indent: HashMap<usize, *mut dyn Element> = HashMap::new();
        let mut page = Page {
            id: Uuid::new_v4(),
            path: path_short,
            title: String::new(),
            childs: Vec::new(),
        };
        parents_by_level.insert(0, &mut page);
        parents_by_indent.insert(0, &mut page);
        // let _last_parent_level = 0;
        for (_, line) in page_md_str
            .split('\n')
            .collect::<Vec<&str>>()
            .iter()
            .enumerate()
        {
            let words: Vec<&str> = line.split(' ').collect::<Vec<&str>>();
            let start_space_count = count_start_spaces(&words).min(line.len());
            let indent = start_space_count / 4;
            let words_2 = &words[start_space_count..];
            let line_2 = &words[start_space_count..].join(" ");
            match words_2[0] {
                "#" => {
                    let new_title = line[2..].to_string();
                    match title {
                        None => {
                            title = Some(new_title);
                        }
                        Some(value) => {
                            panic!("Title already set: old={value:?} new={new_title}");
                        }
                    }
                }
                "##" => {
                    // last_parent_level = 1;
                    let parent = *(parents_by_level.get_mut(&0).unwrap());
                    unsafe {
                        let child = (*parent).add_child(Box::new(Title {
                            value: line[3..].to_string(),
                            childs: Vec::new(),
                        }));
                        parents_by_level.insert(1, child);
                        parents_by_indent.insert(0, child);
                    }
                }
                "###" => {
                    // last_parent_level = 2;
                    let parent = *(parents_by_level.get_mut(&1).unwrap());
                    unsafe {
                        let child = (*parent).add_child(Box::new(Title {
                            value: line[4..].to_string(),
                            childs: Vec::new(),
                        }));
                        parents_by_level.insert(2, child);
                        parents_by_indent.insert(0, child);
                    }
                }
                "-" => {
                    let parent = *(parents_by_indent.get_mut(&indent).unwrap());
                    if line_2.starts_with("- [ ]") {
                        unsafe {
                            let child = (*parent).add_child(Box::new(Task {
                                childs: Vec::new(),
                                status: TaskStatus::New,
                                title: line_2[6..].to_string(),
                            }));
                            parents_by_indent.insert(indent + 1, child);
                        }
                    }
                    if line_2.starts_with("- [x]") {
                        unsafe {
                            let child = (*parent).add_child(Box::new(Task {
                                childs: Vec::new(),
                                status: TaskStatus::Done,
                                title: line_2[6..].to_string(),
                            }));
                            parents_by_indent.insert(indent + 1, child);
                        }
                    }
                }
                _ => {}
            }
        }
        page.title = title.unwrap();
        println!("------");
        println!("{}", page.to_md(0, 0));
        page
    }
}
