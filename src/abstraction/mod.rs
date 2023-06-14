pub mod slide;
pub mod shape;
pub mod paragraph;
pub mod textrun;

use crate::package::Package;
use crate::package::parts::Slide;

pub trait Presentation {
    fn get_slides(&mut self)->&mut [Slide];
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
}

