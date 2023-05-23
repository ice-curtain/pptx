use std::fs::File;
use std::io::{Read, Write};
use std::ops::Index;
use std::str::FromStr;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use zip::{ZipArchive, ZipWriter};
use zip::read::ZipFile;
use zip::write::FileOptions;

use crate::package::content_type::{CONTENT_TYPE_FILE_NAME, ContentType};
use crate::package::media::{MEDIA_DIR, UnSupportPart};
use crate::package::PartEnum::{App, Authors, CommentAuthors, Core, Custom, HandoutMaster, NotesMaster, NotesSlide, PresentationMain, PresProps, Slide, SlideLayout, SlideMaster, TableStyles, Tags, Theme, ViewProps};
use crate::package::parts::{ContentTypes, Part, Presentation, Thumbnail};

pub mod parts;
pub mod content_type;
pub mod doc_props;
pub mod media;


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
pub const HANDOUT_MASTER_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml";


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
    pub slides: Option<Vec<parts::Slide>>,
    themes: Option<Vec<parts::Theme>>,
    notes_slides: Option<Vec<parts::NotesSlide>>,
    notes_masters: Option<Vec<parts::NotesMaster>>,
    slide_layouts: Option<Vec<parts::SlideLayout>>,
    slide_masters: Option<Vec<parts::SlideMaster>>,
    tags: Option<Vec<parts::Tag>>,
    unsupport_parts: Option<Vec<parts::UnSupportParts>>,
    handout_masters: Option<Vec<parts::HandOutMaster>>,
}

pub trait Save {
    fn save(self, writer: &mut ZipWriter<File>);
}

impl<T: Serialize> Save for Option<Part<T>> {
    fn save(self, writer: &mut ZipWriter<File>) {
        match self {
            Some(mut part) => {
                writer.start_file(&part.file_path, FileOptions::default());
                if part.body.is_some() {
                    writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());
                    part.buf = Some(quick_xml::se::to_string(&part.body).unwrap().into_bytes())
                }
                writer.write_all(part.buf.unwrap().as_slice());
            }
            None => {}
        }
    }
}

impl<T: Serialize> Save for Option<Vec<Part<T>>> {
    fn save(self, writer: &mut ZipWriter<File>) {
        match self {
            Some(parts) => {
                for mut part in parts.into_iter() {
                    writer.start_file(&part.file_path, FileOptions::default());
                    if part.body.is_some() {
                        writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());
                        part.buf = Some(quick_xml::se::to_string(&part.body).unwrap().into_bytes())
                    }
                    writer.write_all(part.buf.unwrap().as_slice());
                }
            }
            None => {}
        }
    }
}


impl Package {
    pub fn save(self, file_path: &str) {
        let mut writer = ZipWriter::new(File::create(file_path).unwrap());
        self.content_type.save(&mut writer);
        self.presentation.save(&mut writer);
        self.authors.save(&mut writer);
        self.comment_authors.save(&mut writer);
        self.pres_props.save(&mut writer);
        self.view_props.save(&mut writer);
        self.table_styles.save(&mut writer);
        self.app.save(&mut writer);
        self.core.save(&mut writer);
        self.custom.save(&mut writer);
        self.slides.save(&mut writer);
        self.themes.save(&mut writer);
        self.notes_slides.save(&mut writer);
        self.notes_masters.save(&mut writer);
        self.slide_layouts.save(&mut writer);
        self.slide_masters.save(&mut writer);
        self.tags.save(&mut writer);
        self.handout_masters.save(&mut writer);
        self.unsupport_parts.save(&mut writer);
        writer.flush();
        writer.finish();
    }
}

impl From<ZipArchive<File>> for Package {
    fn from(mut zip: ZipArchive<File>) -> Self {
        let mut file_names = zip.file_names().map(|item| { item.to_string() }).collect::<Vec<String>>();

        let mut package = Package::default();

        let mut content_type_file = zip.by_name(CONTENT_TYPE_FILE_NAME).unwrap();


        let mut content_type: ContentType = read_to(content_type_file);

        for part in content_type.overrides.iter() {
            let part_enum = PartEnum::from_str(&part.content_type);
            match part_enum {
                Ok(part_enum) => {
                    let file_path = &part.part_name[1..];
                    let is_exist = file_names.contains(&file_path.to_string());
                    if is_exist {
                        let index = file_names.iter().position(|r| r == file_path).unwrap();
                        file_names.remove(index);
                    }
                    let mut file = zip.by_name(file_path).unwrap();
                    match part_enum {
                        PresentationMain => {
                            package.presentation = Some(Presentation::new_without_body(file_path, Some(read_to_vec(file))));
                        }
                        SlideMaster => {
                            if package.slide_masters.is_none() {
                                package.slide_masters = Some(Vec::new())
                            }
                            let slide_masters = package.slide_masters.as_mut().unwrap();
                            let slide_master = parts::SlideMaster::new_without_body(file_path, Some(read_to_vec(file)));
                            slide_masters.push(slide_master);
                        }
                        SlideLayout => {
                            if package.slide_layouts.is_none() {
                                package.slide_layouts = Some(Vec::new())
                            }
                            let slide_layouts = package.slide_layouts.as_mut().unwrap();
                            let slide_layout = parts::SlideLayout::new_without_body(file_path, Some(read_to_vec(file)));
                            slide_layouts.push(slide_layout);
                        }
                        Slide => {
                            if package.slides.is_none() {
                                package.slides = Some(Vec::new())
                            }
                            let slides = package.slides.as_mut().unwrap();
                            let slide = parts::Slide::new_without_body(file_path, Some(read_to_vec(file)));
                            slides.push(slide);
                        }
                        NotesMaster => {
                            if package.notes_masters.is_none() {
                                package.notes_masters = Some(Vec::new())
                            }
                            let notes_masters = package.notes_masters.as_mut().unwrap();
                            let notes_master = parts::NotesMaster::new_without_body(file_path, Some(read_to_vec(file)));
                            notes_masters.push(notes_master);
                        }
                        CommentAuthors => { package.comment_authors = Some(parts::CommentAuthors::new_without_body(file_path, Some(read_to_vec(file)))) }
                        PresProps => { package.pres_props = Some(parts::PresProps::new_without_body(file_path, Some(read_to_vec(file)))) }
                        ViewProps => { package.view_props = Some(parts::ViewProps::new_without_body(file_path, Some(read_to_vec(file)))) }
                        Theme => {
                            if package.themes.is_none() {
                                package.themes = Some(Vec::new())
                            }
                            let themes = package.themes.as_mut().unwrap();
                            let theme = parts::Theme::new_without_body(file_path, Some(read_to_vec(file)));
                            themes.push(theme);
                        }
                        TableStyles => { package.table_styles = Some(parts::TableStyles::new_without_body(file_path, Some(read_to_vec(file)))) }
                        NotesSlide => {
                            if package.notes_slides.is_none() {
                                package.notes_slides = Some(Vec::new())
                            }
                            let notes_slides = package.notes_slides.as_mut().unwrap();
                            let notes_slide = parts::NotesSlide::new_without_body(file_path, Some(read_to_vec(file)));
                            notes_slides.push(notes_slide);
                        }
                        Tags => {
                            if package.tags.is_none() {
                                package.tags = Some(Vec::new())
                            }
                            let tags = package.tags.as_mut().unwrap();
                            let tag = parts::Tag::new_without_body(file_path, Some(read_to_vec(file)));
                            tags.push(tag);
                        }
                        Authors => { package.authors = Some(parts::Authors::new_without_body(file_path, Some(read_to_vec(file)))) }
                        Core => { package.core = Some(parts::Core::new_without_body(file_path, Some(read_to_vec(file)))) }
                        App => { package.app = Some(parts::App::new_without_body(file_path, Some(read_to_vec(file)))) }
                        Custom => { package.custom = Some(parts::Custom::new_without_body(file_path, Some(read_to_vec(file)))) }
                        HandoutMaster => {
                            if package.handout_masters.is_none() {
                                package.handout_masters = Some(Vec::new())
                            }
                            let tags = package.handout_masters.as_mut().unwrap();
                            let tag = parts::HandOutMaster::new_without_body(file_path, Some(read_to_vec(file)));
                            tags.push(tag);
                        }
                        _ => { panic!("unknown part") }
                    }
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        package.content_type = Some(ContentTypes::new(CONTENT_TYPE_FILE_NAME, Some(Box::new(content_type)), None));
        let index = file_names.iter().position(|r| r == CONTENT_TYPE_FILE_NAME).unwrap();
        file_names.remove(index);
        for file_name in file_names {
            let mut file = zip.by_name(&file_name).unwrap();
            if package.unsupport_parts.is_none() {
                package.unsupport_parts = Some(Vec::new())
            }
            let medias = package.unsupport_parts.as_mut().unwrap();
            let media = parts::UnSupportParts::new_without_body(&file_name, Some(read_to_vec(file)));
            medias.push(media);
        }


        package
    }
}

fn read_to_vec(mut zipFile: ZipFile) -> Vec<u8> {
    let mut result = vec![];
    zipFile.read_to_end(&mut result);
    result
}


fn read_to<T: DeserializeOwned>(mut zip_file: ZipFile) -> T {
    let start = Instant::now();
    let name = zip_file.name().to_string();

    let mut xml = String::new();
    zip_file.read_to_string(&mut xml);
    let read_duration = start.elapsed();
    let result = quick_xml::de::from_str(&xml);
    let duration = start.elapsed();
    println!("Time elapsed in read {}() is: {:?},read to string is {:?}", name, duration, read_duration);
    match result {
        Ok(x) => { x }
        Err(e) => {
            println!("{}", xml);
            panic!("{}", e);
        }
    }
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
    HandoutMaster,
}

impl FromStr for PartEnum {
    type Err = String;

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
            HANDOUT_MASTER_CONTENT_TYPE => { Ok(HandoutMaster) }
            _ => { Err(format!("unsupport content type:{}", s)) }
        }
    }
}