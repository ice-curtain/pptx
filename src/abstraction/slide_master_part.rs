use rand::Rng;
use crate::package::parts::SlideMaster;
use crate::schemas::presentation::main::CtSlideLayoutIdListEntry;

impl SlideMaster{
    pub fn add_sldlayout(&mut self,r_id:&str){
        if self.body.is_none(){
            self.initial_body();
        }
        let slide_master_body = self.body.as_mut().unwrap();
        let sldLst = slide_master_body.sld_layout_id_lst.as_mut().unwrap();
        match sldLst.sld_layout_id.as_mut() {
            None => {
                sldLst.sld_layout_id= Some(vec![]);
            }
            Some(sld_list) => {
                sld_list.push(CtSlideLayoutIdListEntry{
                    id_attr: Some(rand::thread_rng().gen_range(2147483648..u32::MAX)),
                    r_id_attr: r_id.to_string(),
                    ext_lst: None,
                });
            }
        }
    }
}