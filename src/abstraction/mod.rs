pub mod slide;
pub mod shape;
pub mod paragraph;
pub mod textrun;
pub mod unit;

use std::cmp::max;
use crate::package::Package;
use crate::package::parts::Slide;

pub trait Presentation {
    fn get_slides(&mut self) -> &mut [Slide];
    fn merge(&mut self, package: Package);
}


impl Presentation for Package {
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
        let mut max_number = self.get_max_slide_number();

        if let Some(slides) = slides {
            for mut slide in slides {
                max_number += 1;
                slide.file_path = format!("ppt/slides/slide{}.xml", max_number);
                self.slides.as_mut().unwrap().push(slide);
            }
        }
    }
}

