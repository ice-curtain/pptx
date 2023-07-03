use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::str::from_utf8;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use crate::package::{doc_props, media, NOTES_SLIDE_CONTENT_TYPE, SLIDE_CONTENT_TYPE, SLIDE_LAYOUT_CONTENT_TYPE};
use crate::package::content_type::{ContentType, Override};
use crate::package::media::MediaPart;
use crate::schemas::drawing::main::{CtOfficeStyleSheet, CtTableStyleList};
use crate::schemas::presentation::main::{CtCommentAuthorList, CtHandoutMaster, CtNotesMaster, CtNotesSlide, CtPresentation, CtPresentationProperties, CtSlide, CtSlideLayout, CtSlideMaster, CtTagList, CtViewProperties};
use crate::schemas::standard::microsoft::presentation::CtAuthorList;

#[derive(Serialize, Deserialize, Debug)]
pub struct Part<T> {
    pub file_path: String,
    pub body: Option<Box<T>>,
    pub buf: Option<Vec<u8>>,
}

pub type ContentTypes = Part<ContentType>;
pub type Presentation = Part<CtPresentation>;
pub type Authors = Part<CtAuthorList>;
pub type TableStyles = Part<CtTableStyleList>;
pub type PresProps = Part<CtPresentationProperties>;
pub type CommentAuthors = Part<CtCommentAuthorList>;
pub type ViewProps = Part<CtViewProperties>;
pub type App = Part<doc_props::App>;
pub type Core = Part<doc_props::Core>;
pub type Thumbnail = Part<Vec<u8>>;
pub type Custom = Part<doc_props::Custom>;
pub type Slide = Part<CtSlide>;
pub type Theme = Part<CtOfficeStyleSheet>;
pub type NotesSlide = Part<CtNotesSlide>;
pub type NotesMaster = Part<CtNotesMaster>;
pub type SlideLayout = Part<CtSlideLayout>;
pub type SlideMaster = Part<CtSlideMaster>;
pub type Tag = Part<CtTagList>;
pub type UnSupportParts = Part<media::UnSupportPart>;
pub type HandOutMaster = Part<CtHandoutMaster>;
pub type Rels = Part<Relationships>;
pub type Media = Part<MediaPart>;


impl Rels {
    pub fn update_slide_target(&mut self, target: &str) {
        if self.body.is_none() {
            self.initial_body();
        }

        self.initial_body();
        for item in self.get_items() {
            if Relationship::RELATION_TYPE_SLIDE == item.r#type {
                item.target = target.to_string();
            }
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Relationships {
    #[serde(rename = "@xmlns")]
    pub xmlns: String,

    #[serde(rename = "Relationship")]
    pub relations: Vec<Relationship>,
}

impl Rels {
    pub fn get_max_number(&mut self) -> i32 {
        match self.body.as_mut() {
            None => {
                self.initial_body();
                self.body.as_ref().unwrap().get_max_number()
            }
            Some(relation_box) => {
                relation_box.get_max_number()
            }
        }
    }

    pub fn push(&mut self, relationship: Relationship) {
        self.body.as_mut().unwrap().relations.push(relationship)
    }

    pub fn get_items(&mut self) -> &mut Vec<Relationship> {
        if self.body.is_none() {
            self.initial_body();
        }
        let relations = &mut self.body.as_mut().unwrap().relations;
        return relations;
    }
}

impl Relationships {
    pub fn get_max_number(&self) -> i32 {
        let regex = Regex::new(r"(\d+)").unwrap();
        let mut max_number = 0;
        for relation in &self.relations {
            let id = &relation.id;
            for capture in regex.captures_iter(id) {
                let value: i32 = capture.get(0).unwrap().as_str().parse().unwrap();
                if value > max_number {
                    max_number = value;
                }
            }
        }
        max_number
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationship {
    #[serde(rename = "@Id")]
    pub id: String,

    #[serde(rename = "@Type")]
    pub r#type: String,

    #[serde(rename = "@Target")]
    pub target: String,
}

impl Relationship {
    pub const RELATION_TYPE_SLIDE: &'static str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide";
    pub const RELATION_TYPE_IMAGE: &'static str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
    pub const RELATION_TYPE_NOTES_SLIDE: &'static str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide";
    pub const RELATION_TYPE_SLIDE_LAYOUT: &'static str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout";
}


impl<T> Display for Part<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = self.buf.as_ref().unwrap().as_slice();
        f.write_str(&String::from_utf8_lossy(x))
    }
}


impl<T: DeserializeOwned> Part<T> {
    pub fn initial_body(&mut self) {
        let buffer = self.buf.as_ref().unwrap().as_slice();
        let str = from_utf8(buffer).unwrap();
        let result = quick_xml::de::from_str(str);
        self.body = Some(result.unwrap());
    }


    pub fn new_without_body(file_path: &str, buf: Option<Vec<u8>>) -> Part<T> {
        Part {
            file_path: file_path.to_string(),
            body: None,
            buf,
        }
    }

    pub fn new(file_path: &str, body: Option<Box<T>>, buf: Option<Vec<u8>>) -> Part<T> {
        Part {
            file_path: file_path.to_string(),
            body,
            buf,
        }
    }
}


impl ContentTypes {
    pub fn add_slide(&mut self, path: &str) {
        let item = Override {
            content_type: SLIDE_CONTENT_TYPE.to_string(),
            part_name: path.to_string(),
        };
        match self.body.as_mut() {
            None => {
                self.initial_body();
                self.body.as_mut().unwrap().overrides.push(item);
            }
            Some(relation_box) => {
                relation_box.overrides.push(item);
            }
        }
    }

    pub fn add_slide_layout(&mut self, path: &str) {
        let item = Override {
            content_type: SLIDE_LAYOUT_CONTENT_TYPE.to_string(),
            part_name: path.to_string(),
        };
        match self.body.as_mut() {
            None => {
                self.initial_body();
                self.body.as_mut().unwrap().overrides.push(item);
            }
            Some(relation_box) => {
                relation_box.overrides.push(item);
            }
        }
    }

    pub fn add_notes_slide(&mut self, path: &str) {
        let item = Override {
            content_type: NOTES_SLIDE_CONTENT_TYPE.to_string(),
            part_name: path.to_string(),
        };
        match self.body.as_mut() {
            None => {
                self.initial_body();
                self.body.as_mut().unwrap().overrides.push(item);
            }
            Some(relation_box) => {
                relation_box.overrides.push(item);
            }
        }
    }
}

