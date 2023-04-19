use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use zip::{ZipArchive, ZipWriter};
use zip::read::ZipFile;
use zip::write::FileOptions;

use crate::package::content_type::{CONTENT_TYPE_FILE_NAME, ContentType};
use crate::package::PartEnum::{App, Authors, CommentAuthors, Core, Custom, NotesMaster, NotesSlide, PresentationMain, PresProps, Slide, SlideLayout, SlideMaster, TableStyles, Tags, Theme, ViewProps};
use crate::package::parts::{ContentTypes, Presentation};

pub mod parts;
pub mod content_type;
pub mod doc_props;


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


#[derive(Serialize, Default)]
pub struct Package {
    content_type: Option<ContentTypes>,
    presentation: Option<Presentation>,
    authors: Option<parts::Authors>,
    comment_authors: Option<parts::CommentAuthors>,
    pres_props: Option<parts::PresProps>,
    view_props: Option<parts::ViewProps>,
    table_styles: Option<parts::TableStyles>,
    app: Option<parts::App>,
    core: Option<parts::Core>,
    custom: Option<parts::Custom>,
}


impl Package {
    pub fn save(self) {
        let mut writer = ZipWriter::new(File::create("1.zip").unwrap());
        match self.content_type {
            Some(content_type) => {
                writer.start_file(&content_type.file_path[1..], FileOptions::default());
                writer.write_all(quick_xml::se::to_string(&content_type.body).unwrap().as_ref());
            }
            None => {}
        }
        match self.presentation {
            Some(presentation) => {
                writer.start_file(&presentation.file_path[1..], FileOptions::default());
                writer.write_all(quick_xml::se::to_string(&presentation.body).unwrap().as_ref());
            }
            None => {}
        }
        match self.authors {
            Some(authors) => {
                writer.start_file(&authors.file_path[1..], FileOptions::default());
                writer.write_all(quick_xml::se::to_string(&authors.body).unwrap().as_ref());
            }
            None => {}
        }
        match self.comment_authors {
            Some(comment_authors) => {
                println!("{:?}", comment_authors);
                println!("{:?}", quick_xml::se::to_string(&comment_authors.body));
                writer.start_file(&comment_authors.file_path[1..], FileOptions::default());
                writer.write_all(quick_xml::se::to_string(&comment_authors.body).unwrap().as_ref());
            }
            None => {}
        }
        match self.pres_props {
            Some(pres_props) => {
                writer.start_file(&pres_props.file_path[1..], FileOptions::default());
                writer.write_all(quick_xml::se::to_string(&pres_props.body).unwrap().as_ref());
            }
            None => {}
        }
        match self.view_props {
            Some(view_props) => {
                writer.start_file(&view_props.file_path[1..], FileOptions::default());
                writer.write_all(quick_xml::se::to_string(&view_props.body).unwrap().as_ref());
            }
            None => {}
        }
        match self.table_styles {
            Some(table_styles) => {
                writer.start_file(&table_styles.file_path[1..], FileOptions::default());
                writer.write_all(quick_xml::se::to_string(&table_styles.body).unwrap().as_ref());
            }
            None => {}
        }

        match self.app {
            Some(app) => {
                writer.start_file(&app.file_path[1..], FileOptions::default());
                writer.write_all(quick_xml::se::to_string(&app.body).unwrap().as_ref());
            }
            None => {}
        }


        match self.core {
            Some(core) => {
                writer.start_file(&core.file_path[1..], FileOptions::default());
                writer.write_all(quick_xml::se::to_string(&core.body).unwrap().as_ref());
            }
            None => {}
        }

        match self.custom {
            Some(custom) => {
                writer.start_file(&custom.file_path[1..], FileOptions::default());
                writer.write_all(quick_xml::se::to_string(&custom.body).unwrap().as_ref());
            }
            None => {}
        }

        writer.finish();
    }
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
                Core=>{package.core = Some(parts::Core::new(&part.part_name,read_to(file)))},
                App=>{package.app = Some(parts::App::new(&part.part_name,read_to(file)))},
                Custom=>{package.custom = Some(parts::Custom::new(&part.part_name,read_to(file)))},
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
            CUSTOM_CONTENT_TYPE => { Ok(Custom) }
            _ => { Err(()) }
        }
    }
}