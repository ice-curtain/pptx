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
use crate::package::media::{Media, MEDIA_DIR};
use crate::package::PartEnum::{App, Authors, CommentAuthors, Core, Custom, HandoutMaster, NotesMaster, NotesSlide, PresentationMain, PresProps, Slide, SlideLayout, SlideMaster, TableStyles, Tags, Theme, ViewProps};
use crate::package::parts::{ContentTypes, Presentation, Thumbnail};

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
    slides: Option<Vec<parts::Slide>>,
    themes: Option<Vec<parts::Theme>>,
    notes_slides: Option<Vec<parts::NotesSlide>>,
    notes_masters: Option<Vec<parts::NotesMaster>>,
    slide_layouts: Option<Vec<parts::SlideLayout>>,
    slide_masters: Option<Vec<parts::SlideMaster>>,
    tags: Option<Vec<parts::Tag>>,
    medias: Option<Vec<parts::Media>>,
    handout_masters:Option<Vec<parts::HandOutMaster>>
}


impl Package {
    pub fn save(self) {
        let mut writer = ZipWriter::new(File::create("/Users/kevin/temps/3.pptx").unwrap());
        match self.content_type {
            Some(content_type) => {
                writer.start_file(&content_type.file_path, FileOptions::default());
                writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());
                writer.write_all(quick_xml::se::to_string(&content_type.body).unwrap().as_bytes());
            }
            None => {}
        }
        match self.presentation {
            Some(presentation) => {
                let x = quick_xml::se::to_string(&presentation.body).unwrap();
                writer.start_file(&presentation.file_path, FileOptions::default());
                writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());

                writer.write_all(quick_xml::se::to_string(&presentation.body).unwrap().as_bytes());
            }
            None => {}
        }
        match self.authors {
            Some(authors) => {
                let x = quick_xml::se::to_string(&authors.body).unwrap();
                writer.start_file(&authors.file_path, FileOptions::default());
                writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());
                writer.write_all(quick_xml::se::to_string(&authors.body).unwrap().as_bytes());
            }
            None => {}
        }

        match self.comment_authors {
            Some(comment_authors) => {
                writer.start_file(&comment_authors.file_path, FileOptions::default());
                writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());
                writer.write_all(quick_xml::se::to_string(&comment_authors.body).unwrap().as_ref());
            }
            None => {}
        }
        match self.pres_props {
            Some(pres_props) => {
                writer.start_file(&pres_props.file_path, FileOptions::default());
                writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());
                writer.write_all(quick_xml::se::to_string(&pres_props.body).unwrap().as_ref());
            }
            None => {}
        }
        match self.view_props {
            Some(view_props) => {
                writer.start_file(&view_props.file_path, FileOptions::default());
                writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());
                writer.write_all(quick_xml::se::to_string(&view_props.body).unwrap().as_ref());
            }
            None => {}
        }
        match self.table_styles {
            Some(table_styles) => {
                writer.start_file(&table_styles.file_path, FileOptions::default());
                writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());
                writer.write_all(quick_xml::se::to_string(&table_styles.body).unwrap().as_ref());
            }
            None => {}
        }

        match self.app {
            Some(app) => {
                writer.start_file(&app.file_path, FileOptions::default());
                writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());
                writer.write_all(quick_xml::se::to_string(&app.body).unwrap().as_ref());
            }
            None => {}
        }


        match self.core {
            Some(core) => {
                writer.start_file(&core.file_path, FileOptions::default());
                writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());
                writer.write_all(quick_xml::se::to_string(&core.body).unwrap().as_ref());
            }
            None => {}
        }

        match self.custom {
            Some(custom) => {
                writer.start_file(&custom.file_path, FileOptions::default());
                writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());
                writer.write_all(quick_xml::se::to_string(&custom.body).unwrap().as_ref());
            }
            None => {}
        }

        match self.slides {
            Some(slides) => {
                for slide in slides {
                    writer.start_file(&slide.file_path, FileOptions::default());
                    writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());
                    writer.write_all(quick_xml::se::to_string(&slide.body).unwrap().as_ref());
                }
            }
            None => {}
        }

        match self.themes {
            Some(themes) => {
                for theme in themes {
                    writer.start_file(&theme.file_path, FileOptions::default());
                    writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());
                    writer.write_all(quick_xml::se::to_string(&theme.body).unwrap().as_ref());
                }
            }
            None => {}
        }
        match self.notes_slides {
            Some(notes_slides) => {
                for notes_slide in notes_slides {
                    writer.start_file(&notes_slide.file_path, FileOptions::default());
                    writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());
                    writer.write_all(quick_xml::se::to_string(&notes_slide.body).unwrap().as_ref());
                }
            }
            None => {}
        }
        match self.notes_masters {
            Some(notes_masters) => {
                for notes_master in notes_masters {
                    writer.start_file(&notes_master.file_path, FileOptions::default());
                    writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());
                    writer.write_all(quick_xml::se::to_string(&notes_master.body).unwrap().as_ref());
                }
            }
            None => {}
        }
        match self.slide_layouts {
            Some(slide_layouts) => {
                for slide_layout in slide_layouts {
                    writer.start_file(&slide_layout.file_path, FileOptions::default());
                    writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());
                    writer.write_all(quick_xml::se::to_string(&slide_layout.body).unwrap().as_ref());
                }
            }
            None => {}
        }
        match self.slide_masters {
            Some(slide_masters) => {
                for slide_master in slide_masters {
                    writer.start_file(&slide_master.file_path, FileOptions::default());
                    writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());
                    writer.write_all(quick_xml::se::to_string(&slide_master.body).unwrap().as_ref());
                }
            }
            None => {}
        }
        match self.tags {
            Some(tags) => {
                for tag in tags {
                    writer.start_file(&tag.file_path, FileOptions::default());
                    writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());
                    writer.write_all(quick_xml::se::to_string(&tag.body).unwrap().as_ref());
                }
            }
            None => {}
        }
        match self.handout_masters {
            Some(handout_masters) => {
                for handout in handout_masters {
                    writer.start_file(&handout.file_path, FileOptions::default());
                    writer.write(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.as_bytes());
                    writer.write_all(quick_xml::se::to_string(&handout.body).unwrap().as_ref());
                }
            }
            None => {}
        }



        match self.medias {
            Some(medias) => {
                for media in medias {
                    writer.start_file(&media.file_path, FileOptions::default());
                    writer.write_all(media.body.buf.as_ref());
                }
            }
            None => {}
        }
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
            println!("{}",&part.content_type);
            let part_enum = PartEnum::from_str(&part.content_type).unwrap();

            let file_path = &part.part_name[1..];
            let is_exist = file_names.contains(&file_path.to_string());
            if is_exist {
                let index = file_names.iter().position(|r| r == file_path).unwrap();
                file_names.remove(index);
            }

            let mut file = zip.by_name(file_path).unwrap();
            match part_enum {
                PresentationMain => { package.presentation = Some(Presentation::new(file_path, read_to(file))); }
                SlideMaster => {
                    if package.slide_masters.is_none() {
                        package.slide_masters = Some(Vec::new())
                    }
                    let slide_masters = package.slide_masters.as_mut().unwrap();
                    let slide_master = parts::SlideMaster::new(file_path, read_to(file));
                    slide_masters.push(slide_master);
                }
                SlideLayout => {
                    if package.slide_layouts.is_none() {
                        package.slide_layouts = Some(Vec::new())
                    }
                    let slide_layouts = package.slide_layouts.as_mut().unwrap();
                    let slide_layout = parts::SlideLayout::new(file_path, read_to(file));
                    slide_layouts.push(slide_layout);
                }
                Slide => {
                    if package.slides.is_none() {
                        package.slides = Some(Vec::new())
                    }
                    let slides = package.slides.as_mut().unwrap();
                    let slide = parts::Slide::new(file_path, read_to(file));
                    slides.push(slide);
                }
                NotesMaster => {
                    if package.notes_masters.is_none() {
                        package.notes_masters = Some(Vec::new())
                    }
                    let notes_masters = package.notes_masters.as_mut().unwrap();
                    let notes_master = parts::NotesMaster::new(file_path, read_to(file));
                    notes_masters.push(notes_master);
                }
                CommentAuthors => { package.comment_authors = Some(parts::CommentAuthors::new(file_path, read_to(file))) }
                PresProps => { package.pres_props = Some(parts::PresProps::new(file_path, read_to(file))) }
                ViewProps => { package.view_props = Some(parts::ViewProps::new(file_path, read_to(file))) }
                Theme => {
                    if package.themes.is_none() {
                        package.themes = Some(Vec::new())
                    }
                    let themes = package.themes.as_mut().unwrap();
                    let theme = parts::Theme::new(file_path, read_to(file));
                    themes.push(theme);
                }
                TableStyles => { package.table_styles = Some(parts::TableStyles::new(file_path, read_to(file))) }
                NotesSlide => {
                    if package.notes_slides.is_none() {
                        package.notes_slides = Some(Vec::new())
                    }
                    let notes_slides = package.notes_slides.as_mut().unwrap();
                    let notes_slide = parts::NotesSlide::new(file_path, read_to(file));
                    notes_slides.push(notes_slide);
                }
                Tags => {
                    if package.tags.is_none() {
                        package.tags = Some(Vec::new())
                    }
                    let tags = package.tags.as_mut().unwrap();
                    let tag = parts::Tag::new(file_path, read_to(file));
                    tags.push(tag);
                }
                Authors => { package.authors = Some(parts::Authors::new(file_path, read_to(file))) }
                Core => { package.core = Some(parts::Core::new(file_path, read_to(file))) }
                App => { package.app = Some(parts::App::new(file_path, read_to(file))) }
                Custom => { package.custom = Some(parts::Custom::new(file_path, read_to(file))) }
                HandoutMaster => {
                    if package.handout_masters.is_none() {
                        package.handout_masters = Some(Vec::new())
                    }
                    let tags = package.handout_masters.as_mut().unwrap();
                    let tag = parts::HandOutMaster::new(file_path, read_to(file));
                    tags.push(tag);
                }
                _ => { panic!("unknown part") }
            }
        }
        package.content_type = Some(ContentTypes::new(CONTENT_TYPE_FILE_NAME, content_type));
        for file_name in file_names {
            let mut file = zip.by_name(&file_name).unwrap();

            if file_name.contains(MEDIA_DIR) | file_name.contains("_rels") | file_name.contains("thumbnail") {
                if package.medias.is_none() {
                    package.medias = Some(Vec::new())
                }
                let mut buf = Vec::new();
                file.read_to_end(&mut buf);
                let medias = package.medias.as_mut().unwrap();
                let media = parts::Media::new(&file_name, Media { buf });
                medias.push(media);
            }
        }


        package
    }
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
            _ => { Err(format!("invalid content type:{}",s)) }
        }
    }
}