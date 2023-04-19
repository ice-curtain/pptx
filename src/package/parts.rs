use serde::{Deserialize, Serialize};

use crate::package::content_type::{ContentType};
use crate::package::doc_props;
use crate::schemas::drawing::main::CtTableStyleList;
use crate::schemas::presentation::main::{
    CtCommentAuthorList, CtPresentation, CtPresentationProperties, CtViewProperties,
};
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
pub type Custom = Part<doc_props::Custom>;




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


