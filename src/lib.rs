mod common;
mod schemas;
mod package;
mod zip;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::read_xml_dcl;
    use crate::schemas::drawing::main::{CtOfficeStyleSheet, CtTableStyleList};
    use crate::schemas::presentation::main::{
        CtCommentAuthorList, CtNotesMaster, CtNotesSlide, CtPresentation, CtPresentationProperties,
        CtSlide, CtSlideLayout, CtSlideMaster, CtTagList, CtViewProperties,
    };
    use std::fs::File;
    use std::io::Write;
    use std::str::FromStr;
    use std::time::Instant;
    use crate::package::content_type::ContentType;
    use crate::package::{Package, PartEnum};
    use crate::zip::open;

    #[test]
    fn it_works() {
        let theme_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types  xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
    <Default  Extension="png" ContentType="image/png"/>
    <Default  Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
    <Default  Extension="xml" ContentType="application/xml"/>
    <Override  PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>
    <Override  PartName="/ppt/slideMasters/slideMaster1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"/>
    <Override  PartName="/ppt/slideMasters/slideMaster2.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"/>
    <Override  PartName="/ppt/slides/slide1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
    <Override  PartName="/ppt/slides/slide2.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
    <Override  PartName="/ppt/slides/slide3.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
    <Override  PartName="/ppt/slides/slide4.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
    <Override  PartName="/ppt/slides/slide5.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
    <Override  PartName="/ppt/slides/slide6.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
    <Override  PartName="/ppt/slides/slide7.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
    <Override  PartName="/ppt/slides/slide8.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
    <Override  PartName="/ppt/slides/slide9.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
    <Override  PartName="/ppt/slides/slide10.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
    <Override  PartName="/ppt/slides/slide11.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
    <Override  PartName="/ppt/slides/slide12.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
    <Override  PartName="/ppt/slides/slide13.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
    <Override  PartName="/ppt/slides/slide14.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
    <Override  PartName="/ppt/slides/slide15.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
    <Override  PartName="/ppt/slides/slide16.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
    <Override  PartName="/ppt/slides/slide17.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
    <Override  PartName="/ppt/notesMasters/notesMaster1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml"/>
    <Override  PartName="/ppt/commentAuthors.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml"/>
    <Override  PartName="/ppt/presProps.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presProps+xml"/>
    <Override  PartName="/ppt/viewProps.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml"/>
    <Override  PartName="/ppt/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
    <Override  PartName="/ppt/tableStyles.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml"/>
    <Override  PartName="/ppt/slideLayouts/slideLayout1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"/>
    <Override  PartName="/ppt/slideLayouts/slideLayout2.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"/>
    <Override  PartName="/ppt/slideLayouts/slideLayout3.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"/>
    <Override  PartName="/ppt/slideLayouts/slideLayout4.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"/>
    <Override  PartName="/ppt/slideLayouts/slideLayout5.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"/>
    <Override  PartName="/ppt/theme/theme2.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
    <Override  PartName="/ppt/theme/theme3.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
    <Override  PartName="/ppt/notesSlides/notesSlide1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"/>
    <Override  PartName="/ppt/notesSlides/notesSlide2.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"/>
    <Override  PartName="/ppt/tags/tag1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.tags+xml"/>
    <Override  PartName="/ppt/notesSlides/notesSlide3.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"/>
    <Override  PartName="/ppt/notesSlides/notesSlide4.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"/>
    <Override  PartName="/ppt/tags/tag2.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.tags+xml"/>
    <Override  PartName="/ppt/notesSlides/notesSlide5.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"/>
    <Override  PartName="/ppt/tags/tag3.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.tags+xml"/>
    <Override  PartName="/ppt/notesSlides/notesSlide6.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"/>
    <Override  PartName="/ppt/tags/tag4.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.tags+xml"/>
    <Override  PartName="/ppt/notesSlides/notesSlide7.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"/>
    <Override  PartName="/ppt/tags/tag5.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.tags+xml"/>
    <Override  PartName="/ppt/notesSlides/notesSlide8.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"/>
    <Override  PartName="/ppt/notesSlides/notesSlide9.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"/>
    <Override  PartName="/ppt/notesSlides/notesSlide10.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"/>
    <Override  PartName="/ppt/notesSlides/notesSlide11.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"/>
    <Override  PartName="/ppt/notesSlides/notesSlide12.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"/>
    <Override  PartName="/ppt/notesSlides/notesSlide13.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"/>
    <Override  PartName="/ppt/notesSlides/notesSlide14.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"/>
    <Override  PartName="/ppt/notesSlides/notesSlide15.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"/>
    <Override  PartName="/ppt/notesSlides/notesSlide16.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"/>
    <Override  PartName="/ppt/authors.xml" ContentType="application/vnd.ms-powerpoint.authors+xml"/>
    <Override  PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
    <Override  PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/>
    <Override  PartName="/docProps/custom.xml" ContentType="application/vnd.openxmlformats-officedocument.custom-properties+xml"/>
</Types>"#;
        let theme: ContentType = quick_xml::de::from_str(theme_xml).unwrap();
        // println!("{:?}",theme);
        // println!("{:?}",quick_xml::se::to_string(&theme));
        let mut file = File::create("/Users/kevin/Documents/ECMA-376-1_5th_edition_december_2016/OfficeOpenXML-XMLSchema-Strict/dist.xml").unwrap();

        let mut buffer = String::new();

        quick_xml::se::to_writer(&mut buffer, &theme).unwrap();

        let dcl = read_xml_dcl(&theme_xml);
        if let Ok(dcl) = dcl {
            let _ = file.write(dcl.as_bytes());
        }
        let result = file.write(buffer.as_ref());
        println!("{:?}", result);
        // quick_xml::se::to_writer(file,&theme);
    }



    #[test]
    fn open_file(){
        let start = Instant::now();
        let archive = open("/Users/kevin/Downloads/2月胰腺癌月报-20220321 (2).pptx");
        let duration = start.elapsed();
        println!("Time elapsed in read zip is: {:?}", duration);
        let package = Package::from(archive);
        println!("package memory is {}",std::mem::size_of::<Package>());
        let duration = start.elapsed();
        println!("Time elapsed in deserialize() is: {:?}", duration);
        package.save();
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
        //RUST_MIN_STACK=6291456
    }
}
