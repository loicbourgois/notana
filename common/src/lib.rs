use crate::element::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::net::SocketAddr;
mod element;
#[cfg(test)]
mod test;
mod to_html;
mod to_md;
pub type Tasks = Vec<Task>;
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub created_by: SocketAddr,
    pub title: String,
}
pub fn read(path: &str) -> String {
    fs::read_to_string(path).expect("Should have been able to read the file")
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
        if *word != "" {
            return i;
        }
    }
    return words.len();
}
pub fn json_file_to_html_file(path_in: &str, path_out: &str) {
    write(path_out, &json_to_html(&read(path_in)));
}
pub fn json_to_html(json_str: &str) -> String {
    let page: Element = serde_json::from_str(json_str).unwrap();
    return page.to_html(0, 0);
}
pub fn json_to_md(json_str: &str) -> String {
    let page: Element = serde_json::from_str(json_str).unwrap();
    return page.to_md(0, 0);
}
pub fn md_to_json(md_str: &str) -> String {
    let mut page = Element {
        data: ElementData::Page(Page {
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
                    let childs = &mut (*parent).childs;
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
                    let childs = &mut (*parent).childs;
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
                    let childs = &mut (*parent).childs;
                    let l = childs.len() - 1;
                    let new_child_ptr: *mut Element = &mut childs[l];
                    parents_by_indent.insert(indent + 1, new_child_ptr);
                },
                _ => unsafe {
                    let indent = start_space_count / 4;
                    let parent = &mut (*parents_by_indent[&indent]);
                    parent.childs.push(Element {
                        data: ElementData::Text(Text {
                            txt: line[start_space_count + 0..].to_string(),
                        }),
                        childs: Vec::new(),
                    });
                    let childs = &mut (*parent).childs;
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
                        line = Some(words_2[1].replace("l", ""));
                        user = words_2[2].replace("@", "");
                    } else {
                        user = words_2[1].replace("@", "");
                    }
                    parent.childs.push(Element {
                        data: ElementData::Comment(Comment {
                            text: Vec::new(),
                            user: user,
                            line: line,
                        }),
                        childs: Vec::new(),
                    });
                    let childs = &mut (*parent).childs;
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
    let json_str = serde_json::to_string_pretty(&page).unwrap();
    return json_str;
}
