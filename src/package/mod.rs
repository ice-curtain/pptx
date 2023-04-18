use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use zip::read::ZipFile;
use zip::ZipArchive;

use crate::package::content_type::{CONTENT_TYPE_FILE_NAME, ContentType};
use crate::package::PartEnum::{App, Authors, CommentAuthors, Core, NotesMaster, NotesSlide, PresentationMain, PresProps, Slide, SlideLayout, SlideMaster, TableStyles, Tags, Theme, ViewProps};
use crate::package::parts::{ContentTypes, Presentation};

pub mod parts;
pub mod content_type;


pub const PRESENTATION_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml";
pub const SLIDE_MASTER_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml";
pub const SLIDE_LAYOUT_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml";
pub const SLIDE_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.slide+xml";
pub const NOTES_MASTER_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml";
pub const COMMENT_AUTHORS: &str = "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml";
pub const PRES_PROPS_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml";
pub const VIEW_PROPS_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml";
pub const THEME_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.theme+xml";
pub const TABLE_STYLES_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml";
pub const NOTES_SLIDE_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml";
pub const TAGS_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.tags+xml";
pub const AUTHORS_CONTENT_TYPE: &str = "application/vnd.ms-powerpoint.authors+xml";
pub const CORE_CONTENT_TYPE: &str = "application/vnd.openxmlformats-package.core-properties+xml";
pub const APP_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.extended-properties+xml";
pub const CUSTOM_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.custom-properties+xml";


#[derive(Serialize,Default)]
pub struct Package {
    content_type: Option<ContentTypes>,
    presentation: Option<Presentation>,
    authors: Option<parts::Authors>,
    comment_authors: Option<parts::CommentAuthors>,
    pres_props: Option<parts::PresProps>,
    view_props: Option<parts::ViewProps>,
    table_styles: Option<parts::TableStyles>,

}

impl From<ZipArchive<File>> for Package {
    fn from(mut zip: ZipArchive<File>) -> Self {
        let len = zip.len();
        let mut package = Package::default();


        let mut content_type_file = zip.by_name(CONTENT_TYPE_FILE_NAME).unwrap();
        let content_type: ContentType = read_to(content_type_file);
        for part in content_type.overrides.iter() {
            let part_enum = PartEnum::from_str(&part.content_type).unwrap();
            let mut file = zip.by_name(&part.part_name[1..]).unwrap();

            match part_enum {
                PresentationMain => { package.presentation = Some(Presentation::new(&part.part_name, read_to(file))); }
                // SlideMaster => { package.presentation = Some(Presentation::new(&part.part_name, read_to(file))); }
                // SlideLayout,
                // Slide,
                // NotesMaster,
                CommentAuthors => { package.comment_authors = Some(parts::CommentAuthors::new(&part.part_name, read_to(file))) }
                PresProps => { package.pres_props = Some(parts::PresProps::new(&part.part_name, read_to(file))) }
                ViewProps => { package.view_props = Some(parts::ViewProps::new(&part.part_name, read_to(file))) }
                // Theme,
                TableStyles => { package.table_styles = Some(parts::TableStyles::new(&part.part_name, read_to(file))) }
                // NotesSlide,
                // Tags,
                Authors => { package.authors = Some(parts::Authors::new(&part.part_name, read_to(file))) }
                // Core=>{package.comment_authors = Some(parts::CommentAuthors::new(&part.part_name,read_to(file)))},
                // App=>{package.comment_authors = Some(parts::CommentAuthors::new(&part.part_name,read_to(file)))},
                // Custom,
                _ => {}
            }
        }
        package.content_type = Some(ContentTypes::new(CONTENT_TYPE_FILE_NAME, content_type));

        package
    }
}


fn read_to<T: DeserializeOwned>(mut zip_file: ZipFile) -> T {
    let mut xml = String::new();
    zip_file.read_to_string(&mut xml);
    quick_xml::de::from_str(&xml).unwrap()
}


#[derive(Debug)]
pub enum PartEnum {
    PresentationMain,
    SlideMaster,
    SlideLayout,
    Slide,
    NotesMaster,
    CommentAuthors,
    PresProps,
    ViewProps,
    Theme,
    TableStyles,
    NotesSlide,
    Tags,
    Authors,
    Core,
    App,
    Custom,
}

impl FromStr for PartEnum {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            PRESENTATION_CONTENT_TYPE => { Ok(PresentationMain) }
            SLIDE_MASTER_CONTENT_TYPE => { Ok(SlideMaster) }
            SLIDE_LAYOUT_CONTENT_TYPE => { Ok(SlideLayout) }
            SLIDE_CONTENT_TYPE => { Ok(Slide) }
            NOTES_MASTER_CONTENT_TYPE => { Ok(NotesMaster) }
            COMMENT_AUTHORS => { Ok(CommentAuthors) }
            PRES_PROPS_CONTENT_TYPE => { Ok(PresProps) }
            VIEW_PROPS_CONTENT_TYPE => { Ok(ViewProps) }
            THEME_CONTENT_TYPE => { Ok(Theme) }
            TABLE_STYLES_CONTENT_TYPE => { Ok(TableStyles) }
            NOTES_SLIDE_CONTENT_TYPE => { Ok(NotesSlide) }
            TAGS_CONTENT_TYPE => { Ok(Tags) }
            AUTHORS_CONTENT_TYPE => { Ok(Authors) }
            CORE_CONTENT_TYPE => { Ok(Core) }
            APP_CONTENT_TYPE => { Ok(App) }
            CUSTOM_CONTENT_TYPE => { Ok(Core) }
            _ => { Err(()) }
        }
    }
}