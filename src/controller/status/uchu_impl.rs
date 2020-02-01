use super::super::super::controller::common::conv::*;
use super::super::super::controller::status::uchu::*;
use super::super::super::model::master::person::Person;
use super::super::super::model::master::square::*;

impl Uchu {
    /**
     * らいおんの位置
     */
    pub fn get_sq_r(&self, jiai: &Person) -> Square {
        Square::from_umasu(self.ky.ms_r[sn_to_num(&self.get_teban(jiai))])
    }
}
