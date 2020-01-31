use super::super::super::controller::common::conv::*;
use super::super::super::controller::status::uchu::*;
use super::super::super::model::master::person::Person;
use super::super::super::model::master::place::*;

impl Uchu {
    /**
     * らいおんの位置
     */
    pub fn get_ms_r(&self, jiai: &Person) -> umasu {
        self.ky.ms_r[sn_to_num(&self.get_teban(jiai))]
    }
}
