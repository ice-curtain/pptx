use serde::{Deserialize, Serialize};

use crate::package::content_type::{ContentType};
use crate::package::{doc_props, media};
use crate::schemas::drawing::main::{CtOfficeStyleSheet, CtTableStyleList};
use crate::schemas::presentation::main::{CtCommentAuthorList, CtNotesMaster, CtNotesSlide, CtPresentation, CtPresentationProperties, CtSlide, CtSlideLayout, CtSlideMaster, CtTagList, CtViewProperties};
use crate::schemas::standard::microsoft::presentation::CtAuthorList;

#[derive(Serialize, Deserialize, Debug)]
pub struct Part<T> {
    pub file_path: String,
    pub body: T,
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
pub type Media = Part<media::Media>;




impl<T> Part<T> {
    pub fn new(file_path: &str, body: T) -> Part<T> {
        Part {
            file_path: file_path.to_string(),
            body,
        }
    }

    pub fn save(self){

    }
}


