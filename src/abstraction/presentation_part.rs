use rand::Rng;
use crate::abstraction::Slides;
use crate::package::parts::Presentation;
use crate::schemas::presentation::main::CtSlideIdListEntry;


impl Presentation{
    pub fn add_sld(&mut self,r_id:&str){
        if self.body.is_none(){
            self.initial_body();
        }
        let presentation_body = self.body.as_mut().unwrap();
        let sldLst = presentation_body.sld_id_lst.as_mut().unwrap();
        match sldLst.sld_id.as_mut() {
            None => {
                sldLst.sld_id= Some(vec![]);
            }
            Some(sld_list) => {
                sld_list.push(CtSlideIdListEntry{
                    id_attr: Some(rand::thread_rng().gen_range(256..2147483648)),
                    r_id_attr: r_id.to_string(),
                    ext_lst: None,
                });
            }
        }



    }
}