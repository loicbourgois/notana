use crate::count_start_spaces;
use crate::my_uuid;
use crate::read_path_buf;
use crate::task::Task;
use crate::task::TaskStatus;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;
use uuid::Uuid;
#[derive(Serialize, Debug)]
pub struct Page {
    #[serde(with = "my_uuid")]
    pub id: Uuid,
    pub title: String,
    pub path: String,
    #[serde(skip_serializing)]
    childs: Vec<Box<dyn Element>>,
    #[serde(skip_serializing)]
    pub comments: Option<String>,
}
#[derive(Debug)]
pub struct TaskReference {
    id: Uuid,
    childs: Vec<Box<dyn Element>>,
}
#[derive(Debug)]
pub struct ListItem {
    txt: String,
    childs: Vec<Box<dyn Element>>,
}
#[derive(Debug)]
pub struct CodeBlock {
    content: Vec<String>,
    childs: Vec<Box<dyn Element>>,
}
#[derive(Debug)]
pub struct Title {
    value: String,
    childs: Vec<Box<dyn Element>>,
}
#[derive(Debug)]
pub struct Line {
    value: String,
    childs: Vec<Box<dyn Element>>,
}
pub trait Element: Debug {
    fn add_child(&mut self, child: Box<dyn Element>) -> *mut dyn Element;
    fn to_md(&self, depth: usize, indent: usize, tasks: &HashMap<Uuid, Task>) -> String;
}
impl Element for TaskReference {
    fn add_child(&mut self, child: Box<dyn Element>) -> *mut dyn Element {
        self.childs.push(child);
        let l = self.childs.len() - 1;
        &mut *self.childs[l]
    }

    fn to_md(&self, depth: usize, indent: usize, tasks: &HashMap<Uuid, Task>) -> String {
        let mut strs: std::vec::Vec<String> = Vec::new();
        let task = tasks.get(&self.id).unwrap();
        let status_str = match task.status {
            TaskStatus::Done => "x",
            TaskStatus::New => " ",
        };
        strs.push(format!(
            "{}- [{status_str}] {}",
            "    ".repeat(indent),
            task.title
        ));
        for c in &self.childs {
            strs.push(c.to_md(depth + 1, indent + 1, tasks));
        }
        strs.join("\n")
    }
}
impl Element for CodeBlock {
    fn add_child(&mut self, child: Box<dyn Element>) -> *mut dyn Element {
        self.childs.push(child);
        let l = self.childs.len() - 1;
        &mut *self.childs[l]
    }

    fn to_md(&self, _depth: usize, _indent: usize, _tasks: &HashMap<Uuid, Task>) -> String {
        // TODO: handle childs
        self.content.join("\n")
    }
}
impl Element for Line {
    fn add_child(&mut self, child: Box<dyn Element>) -> *mut dyn Element {
        self.childs.push(child);
        let l = self.childs.len() - 1;
        &mut *self.childs[l]
    }

    fn to_md(&self, depth: usize, indent: usize, tasks: &HashMap<Uuid, Task>) -> String {
        let mut strs: std::vec::Vec<String> = Vec::new();
        strs.push(format!("{}{}", "    ".repeat(indent), self.value));
        for c in &self.childs {
            strs.push(c.to_md(depth + 1, indent + 1, tasks));
        }
        strs.join("\n")
    }
}
impl Element for ListItem {
    fn add_child(&mut self, child: Box<dyn Element>) -> *mut dyn Element {
        self.childs.push(child);
        let l = self.childs.len() - 1;
        &mut *self.childs[l]
    }

    fn to_md(&self, depth: usize, indent: usize, tasks: &HashMap<Uuid, Task>) -> String {
        let mut strs: std::vec::Vec<String> = Vec::new();
        strs.push(format!("{}- {}", "    ".repeat(indent), self.txt));
        for c in &self.childs {
            strs.push(c.to_md(depth + 1, indent + 1, tasks));
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

    fn to_md(&self, depth: usize, _indent: usize, tasks: &HashMap<Uuid, Task>) -> String {
        let mut strs = Vec::new();
        strs.push(String::new());
        strs.push(format!("{} {}", "#".repeat(depth + 1), self.value));
        for c in &self.childs {
            strs.push(c.to_md(depth + 1, 0, tasks));
        }
        strs.join("\n")
    }
}
impl Element for Page {
    fn add_child(&mut self, child: Box<dyn Element>) -> *mut dyn Element {
        self.childs.push(child);
        let l = self.childs.len() - 1;
        &mut *self.childs[l]
    }

    fn to_md(&self, depth: usize, _indent: usize, tasks: &HashMap<Uuid, Task>) -> String {
        let mut strs = Vec::new();
        strs.push(format!("# {}", self.title));
        for c in &self.childs {
            strs.push(c.to_md(depth + 1, 0, tasks));
        }
        strs.push(String::new());
        match &self.comments {
            Some(comments) => {
                strs.push("## Comments".to_string());
                strs.push(comments.to_string());
            }
            None => {}
        }
        strs.join("\n")
    }
}
enum CodeBlockState {
    Open,
    Closed,
}
impl Page {
    pub fn from_path(
        path_long: PathBuf,
        path_short: String,
        tasks: &mut HashMap<Uuid, Task>,
    ) -> Page {
        let page_md_str_full = read_path_buf(path_long);
        let page_md_str_split = page_md_str_full.split("## Comments\n").collect::<Vec<_>>();
        let page_md_str = page_md_str_split[0];
        let mut title = None;
        let mut parents_by_level: HashMap<usize, *mut dyn Element> = HashMap::new();
        let mut parents_by_indent: HashMap<usize, *mut dyn Element> = HashMap::new();
        let mut page = Page {
            id: Uuid::new_v4(),
            path: path_short,
            title: String::new(),
            childs: Vec::new(),
            comments: None,
        };
        if page_md_str_split.len() == 2 {
            page.comments = Some(page_md_str_split[1].to_string());
        }
        parents_by_level.insert(0, &mut page);
        parents_by_indent.insert(0, &mut page);
        let mut code_block_state = CodeBlockState::Closed;
        let mut code_block_content = Vec::new();
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
                    let status = if line_2.starts_with("- [ ]") {
                        Some(TaskStatus::New)
                    } else if line_2.starts_with("- [x]") {
                        Some(TaskStatus::Done)
                    } else {
                        unsafe {
                            let child = (*parent).add_child(Box::new(ListItem {
                                childs: Vec::new(),
                                txt: line_2[2..].to_string(),
                            }));
                            parents_by_indent.insert(indent + 1, child);
                        }
                        None
                    };
                    match status {
                        Some(status) => unsafe {
                            let task = Task {
                                id: Uuid::new_v4(),
                                subtasks: Vec::new(),
                                status,
                                title: line_2[6..].to_string(),
                            };
                            let child = (*parent).add_child(Box::new(TaskReference {
                                childs: Vec::new(),
                                id: task.id,
                            }));
                            tasks.insert(task.id, task);
                            parents_by_indent.insert(indent + 1, child);
                        },
                        _ => {}
                    }
                }
                "```mermaid" => {
                    code_block_content.push((*line).to_string());
                    match code_block_state {
                        CodeBlockState::Closed => code_block_state = CodeBlockState::Open,
                        CodeBlockState::Open => panic!("invalid"),
                    }
                }
                "```" => {
                    code_block_content.push((*line).to_string());
                    match code_block_state {
                        CodeBlockState::Closed => code_block_state = CodeBlockState::Open,
                        CodeBlockState::Open => unsafe {
                            code_block_state = CodeBlockState::Closed;
                            let parent = *(parents_by_indent.get_mut(&indent).unwrap());
                            let child = (*parent).add_child(Box::new(CodeBlock {
                                childs: Vec::new(),
                                content: code_block_content.clone(),
                            }));
                            parents_by_indent.insert(indent + 1, child);
                            code_block_content.clear();
                        },
                    }
                }
                _ => match code_block_state {
                    CodeBlockState::Open => code_block_content.push((*line).to_string()),
                    CodeBlockState::Closed => {
                        if line.is_empty() {
                        } else {
                            unsafe {
                                let parent = *(parents_by_indent.get_mut(&indent).unwrap());
                                let child = (*parent).add_child(Box::new(Line {
                                    childs: Vec::new(),
                                    value: (*line_2).to_string(),
                                }));
                                parents_by_indent.insert(indent + 1, child);
                            }
                        }
                    }
                },
            }
        }
        page.title = title.unwrap();
        page
    }
}
