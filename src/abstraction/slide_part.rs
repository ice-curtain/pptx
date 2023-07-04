use std::str::{Chars, from_utf8, from_utf8_unchecked};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::abstraction::shape::Shape;
use crate::package::parts::Slide;
use crate::schemas::drawing::main::{CtRegularTextRun, CtTextBody};
use crate::schemas::presentation::main::{CtShape, ShapeChoice};

/// part类型的是因为需要判断body，因此需要加上_part来跟其他元素进行区别

impl Slide {

    pub fn list_shape(&mut self) -> Vec<&mut Shape> {
        if self.body.is_none() {
            self.initial_body();
        }
        let sp_tree = &mut self.body.as_mut().unwrap().c_sld.sp_tree;
        let mut result = Vec::new();
        let items = &mut sp_tree.items;
        for shapechoice in items {
            match shapechoice {
                ShapeChoice::Shape(shape) => {
                    result.push(shape)
                }
                _ => {}
            }
        }
        result
    }
}


pub trait GetAutoSize {
    fn get_auto_size(&self) -> (usize, usize);
}

impl GetAutoSize for CtTextBody {
    fn get_auto_size(&self) -> (usize, usize) {
        for paragraph in self.p.iter() {
            let textruns = paragraph.r.as_ref().unwrap();
            for textrun in textruns {
                let count = textrun.t.word_count();
            }
        }
        (0, 0)
    }
}

impl WordCount for CtTextBody {
    fn word_count(&self) -> usize {
        let mut total = 0;
        for paragraph in self.p.iter() {
            let textruns = paragraph.r.as_ref().unwrap();
            for textrun in textruns {
                let count = textrun.t.word_count();
                total += count;
            }
        }
        total
    }
}

///
/// 在PPT中，英文字符是中文字符的2分之一，因此计算长宽时，我们以半个字体的大小作为单位计算数量
///
pub trait WordCount {
    fn word_count(&self) -> usize;
}

impl WordCount for String {
    fn word_count(&self) -> usize {
        let chars = self.chars();
        let mut count = 0;
        for c in chars {
            if c.is_ascii() {
                println!("{},width:{}", c, (&c.to_string()).width());
                count += 1;
            } else if is_cjk(&c.to_string()) {
                println!("{},width:{}", c, (&c.to_string()).width_cjk());
                count += 2;
            }
        }
        count
    }
}


fn is_cjk(s: &str) -> bool {
    let mut cursor = UnicodeSegmentation::graphemes(s, true);
    if let Some(s) = cursor.next() {
        let code_point = s.chars().next().unwrap() as u32;
        return code_point >= 0x4E00 && code_point <= 0x9FFF;
    }
    false
}

