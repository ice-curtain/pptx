pub mod slide_part;
pub mod shape;
pub mod paragraph;
pub mod textrun;
pub mod unit;
pub mod presentation_part;
pub mod slide_master_part;

use std::cmp::max;
use std::collections::HashMap;
use std::fmt::format;
use std::num::ParseIntError;
use std::ops::Index;
use regex::{Match, Regex};
use crate::package::Package;
use crate::package::parts::{Media, NotesSlide, Relationship, Rels, Slide, SlideLayout};

pub trait Slides {
    fn get_slides(&mut self) -> &mut [Slide];
    fn merge(&mut self, package: Package);
}


impl Slides for Package {
    fn get_slides(&mut self) -> &mut [Slide] {
        match &mut self.slides {
            Some(slides) => {
                let x = slides.as_mut_slice();
                x
            }
            None => {
                &mut []
            }
        }
    }

    fn merge(&mut self, mut package: Package) {
        let slides = package.slides;
        let slide_part_indexs = add_slide_part(self, slides);
        add_presentation_item(self, &slide_part_indexs);
        add_slide_rel_part(self, &slide_part_indexs, package.slide_rels, package.notes_slides, package.slide_layout_rels, package.medias, package.slide_layouts, package.notes_slide_rels);
    }
}

fn add_slide_rel_part(package: &mut Package, slide_index: &Vec<(i32, i32)>, slide_rels: Option<Vec<Rels>>, mut notes_slides: Option<Vec<NotesSlide>>, mut slide_layout_rels: Option<Vec<Rels>>, mut medias: Option<Vec<Media>>, mut slide_layouts: Option<Vec<SlideLayout>>, mut notes_slide_rels: Option<Vec<Rels>>) {
    let mut map = HashMap::new();
    for (old_index, index) in slide_index {
        let old_path = format!("ppt/slides/_rels/slide{}.xml.rels", old_index);
        let new_path = format!("ppt/slides/_rels/slide{}.xml.rels", index);
        map.insert(old_path, new_path);
    }

    match slide_rels {
        None => {}
        Some(slide_rels) => {
            for mut rel in slide_rels {
                if let Some(new_path) = map.get(&rel.file_path) {
                    let mut image_max_number = package.get_max_image_media_number();
                    let mut slide_layouts_number = package.get_max_slide_layouts_number();
                    let mut notes_slide_number = package.get_max_notes_slide_number();

                    let relations = rel.get_items();
                    for relation in relations {
                        let target = &relation.target;
                        let ppt_path = target.to_string().replace("..", "ppt");
                        match relation.r#type.as_ref() {
                            Relationship::RELATION_TYPE_IMAGE => {
                                image_max_number = image_max_number + 1;
                                let suffix = target.split(".").last().unwrap();
                                let medias_reference = medias.as_mut().unwrap();
                                if let Some(target) = add_media(package, image_max_number, suffix, &ppt_path, medias_reference) {
                                    relation.target = target;
                                };
                            }
                            Relationship::RELATION_TYPE_NOTES_SLIDE => {
                                notes_slide_number = notes_slide_number + 1;
                                let notes_slide_reference = notes_slides.as_mut().unwrap();
                                let old_index = extract_first_digital(&relation.target);
                                if let Some(target) = add_note_slide(package, notes_slide_number, &ppt_path, notes_slide_reference) {
                                    relation.target = target;
                                };
                                package.content_type.as_mut().unwrap().add_notes_slide(&format!("/ppt/notesSlides/notesSlide{}.xml", notes_slide_number));


                                match notes_slide_rels.as_mut() {
                                    None => {
                                        println!("notes_slide_rels is none")
                                    }

                                    Some(notes_slide_rels) => {
                                        match old_index {
                                            None => {
                                                println!("old_index is none")
                                            }
                                            Some(old_index) => {
                                                let note_slide_rel_path = format!("ppt/notesSlides/_rels/notesSlide{}.xml.rels", old_index);
                                                let mut tobe_add_index = None;
                                                for (index, rels) in notes_slide_rels.iter_mut().enumerate() {
                                                    println!("rels.file_path = {},target = {}, eq = {}", rels.file_path, note_slide_rel_path, rels.file_path == note_slide_rel_path);
                                                    if rels.file_path == note_slide_rel_path {
                                                        tobe_add_index = Some(index);
                                                    }
                                                }
                                                println!("tobe_add_index = {:?}", tobe_add_index);
                                                if tobe_add_index.is_some() {
                                                    let mut notes_slide_rel = notes_slide_rels.remove(tobe_add_index.unwrap());
                                                    let new_rels_path = format!("ppt/notesSlides/_rels/notesSlide{}.xml.rels", notes_slide_number);
                                                    println!("{}", new_rels_path);
                                                    notes_slide_rel.file_path = new_rels_path;
                                                    notes_slide_rel.update_slide_target(&format!("../slides/slide{}.xml", extract_first_digital(new_path).unwrap()));

                                                    package.push_notes_slide_rel(notes_slide_rel);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Relationship::RELATION_TYPE_SLIDE_LAYOUT => {
                                if slide_layouts_number >= extract_first_digital(&relation.target).unwrap() {
                                    continue;
                                }

                                slide_layouts_number = slide_layouts_number + 1;

                                let slide_layouts_reference = slide_layouts.as_mut().unwrap();
                                let old_index = extract_first_digital(&relation.target);
                                if let Some(target) = add_slide_layout(package, slide_layouts_number, &ppt_path, slide_layouts_reference) {
                                    relation.target = target;
                                };
                                package.content_type.as_mut().unwrap().add_slide_layout(&format!("/ppt/slideLayouts/slideLayout{}.xml", slide_layouts_number));

                                match slide_layout_rels.as_mut() {
                                    None => {
                                        println!("slide_layout_rels is none")
                                    }

                                    Some(slide_layout_rels) => {
                                        match old_index {
                                            None => {
                                                println!("old_index is none")
                                            }
                                            Some(old_index) => {
                                                let slide_layout_rel_path = format!("ppt/slideLayouts/_rels/slideLayout{}.xml.rels", old_index);
                                                let mut tobe_add_index = None;
                                                for (index, rels) in slide_layout_rels.iter_mut().enumerate() {
                                                    println!("rels.file_path = {},target = {}, eq = {}", rels.file_path, slide_layout_rel_path, rels.file_path == slide_layout_rel_path);
                                                    if rels.file_path == slide_layout_rel_path {
                                                        tobe_add_index = Some(index);
                                                    }
                                                }
                                                println!("tobe_add_index = {:?}", tobe_add_index);
                                                if tobe_add_index.is_some() {
                                                    let mut slide_layout_rel = slide_layout_rels.remove(tobe_add_index.unwrap());

                                                    let items = slide_layout_rel.get_items();
                                                    for item in items {
                                                        let relation_target = &item.target;
                                                        let item_ppt_path = relation_target.to_string().replace("..", "ppt");

                                                        println!("item.type {}", item.r#type);
                                                        match item.r#type.as_ref() {
                                                            Relationship::RELATION_TYPE_IMAGE => {
                                                                println!("hit image");

                                                                image_max_number = image_max_number + 1;
                                                                let suffix = relation_target.split(".").last().unwrap();
                                                                let medias_reference = medias.as_mut().unwrap();

                                                                match add_media(package, image_max_number, suffix, &item_ppt_path, medias_reference) {
                                                                    None => {
                                                                        println!("add media error")
                                                                    }
                                                                    Some(target) => {
                                                                        println!("target = ={}", target);
                                                                        item.target = target;
                                                                    }
                                                                }
                                                            }
                                                            _ => {}
                                                        }
                                                    }
                                                    let new_rels_path = format!("ppt/slideLayouts/_rels/slideLayout{}.xml.rels", slide_layouts_number);
                                                    println!("{}", new_rels_path);
                                                    slide_layout_rel.file_path = new_rels_path;
                                                    package.push_slide_layout_rel(slide_layout_rel);

                                                    let slide_master_rels = package.slide_master_rels.as_mut().unwrap();
                                                    let first_rel = slide_master_rels.get_mut(0).unwrap();
                                                    let new_number = first_rel.get_max_number() + 1;
                                                    let rId = format!("rId{}", new_number);

                                                    let first_slide_master = package.slide_masters.as_mut().unwrap().get_mut(0).unwrap();
                                                    first_slide_master.add_sldlayout(&rId);




                                                    first_rel.push(Relationship {
                                                        id: rId,
                                                        r#type: Relationship::RELATION_TYPE_SLIDE_LAYOUT.to_string(),
                                                        target: format!("../slideLayouts/slideLayout{}.xml", slide_layouts_number),
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    rel.file_path = new_path.to_string();
                    package.push_slide_rel(rel);
                }
            }
        }
    }
}


fn add_slide_layout(package: &mut Package, slide_layouts_number: i32, target_path: &str, slide_layouts_reference: &mut Vec<SlideLayout>) -> Option<String> {
    let mut to_be_add_index = None;

    for (index, slide_layout) in slide_layouts_reference.iter_mut().enumerate() {
        if &slide_layout.file_path == target_path {
            slide_layout.file_path = format!("ppt/slideLayouts/slideLayout{}.xml", slide_layouts_number);
            to_be_add_index = Some(index);
        }
    }
    match to_be_add_index {
        None => { None }
        Some(to_be_add_index) => {
            let slide_layout = slide_layouts_reference.remove(to_be_add_index);
            package.push_slide_layout(slide_layout);
            Some(format!("../slideLayouts/slideLayout{}.xml", slide_layouts_number))
        }
    }
}


fn add_note_slide(package: &mut Package, notes_slide_max_number: i32, target_path: &str, notes_slide_reference: &mut Vec<NotesSlide>) -> Option<String> {
    let mut to_be_add_index = 99999;

    for (index, notes_slide) in notes_slide_reference.iter_mut().enumerate() {
        if &notes_slide.file_path == target_path {
            notes_slide.file_path = format!("ppt/notesSlides/notesSlide{}.xml", notes_slide_max_number);
            to_be_add_index = index;
        }
    }
    if to_be_add_index != 99999 {
        let notes_slide = notes_slide_reference.remove(to_be_add_index);
        package.push_notes_slide(notes_slide);
        return Some(format!("../notesSlides/notesSlide{}.xml", notes_slide_max_number));
    } else {
        return None;
    }
}

fn add_media(package: &mut Package, image_max_number: i32, suffix: &str, ppt_path: &String, medias_reference: &mut Vec<Media>) -> Option<(String)> {
    let mut to_be_add_index = 99999;
    for (index, media) in medias_reference.iter_mut().enumerate() {
        if &media.file_path == ppt_path {
            media.file_path = format!("ppt/media/image{}.{}", image_max_number, suffix);
            to_be_add_index = index;
        }
    }
    if to_be_add_index != 99999 {
        let media = medias_reference.remove(to_be_add_index);
        package.push_media(media);
        return Some(format!("../media/image{}.{}", image_max_number, suffix));
    } else {
        return None;
    }
}


fn add_slide_part(package: &mut Package, slides: Option<Vec<Slide>>) -> Vec<(i32, i32)> {
    let mut max_number = package.get_max_slide_number();
    let mut result = Vec::new();
    if let Some(slides) = slides {
        let mut new_index = max_number;
        for mut slide in slides {
            new_index += 1;
            let old_index = extract_first_digital(&slide.file_path).expect("extract degital form slide file name error");
            slide.file_path = format!("ppt/slides/slide{}.xml", new_index);
            package.slides.as_mut().unwrap().push(slide);
            result.push((old_index, new_index));
            package.content_type.as_mut().unwrap().add_slide(&format!("/ppt/slides/slide{}.xml", new_index));
        }
    }
    result
}

pub fn extract_first_digital(str: &str) -> Option<i32> {
    let regex = Regex::new(r"(\d+)").unwrap();
    let mut max = 0;
    for capture in regex.captures_iter(str) {
        let capture = capture.get(0);
        match capture {
            None => {}
            Some(capture) => {
                let result: Result<i32, ParseIntError> = capture.as_str().parse();
                match result {
                    Ok(result) => {
                        return Some(result);
                    }
                    Err(_) => {}
                }
            }
        }
    }
    return None;
}


fn add_presentation_item(package: &mut Package, slide_part_indexs: &Vec<(i32, i32)>) {
    for (_, index) in slide_part_indexs {
        let mut source_presentation_rels = package.presentation_rels.as_mut().unwrap();
        let mut presentation_relation_max_number = source_presentation_rels.get_max_number();
        presentation_relation_max_number += 1;

        let rId = format!("rId{}", presentation_relation_max_number);

        //add sldId in presentation.xml
        package.presentation.as_mut().unwrap().add_sld(&rId);
        //add a relation item in presentation.xml.rel
        source_presentation_rels.push(Relationship {
            id: rId,
            r#type: Relationship::RELATION_TYPE_SLIDE.to_string(),
            target: format!("slides/slide{}.xml", index),
        });
    }
}



