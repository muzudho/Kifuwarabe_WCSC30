use super::super::jotai::uchu::*;
use super::super::model::master::person::Person;
use super::super::model::master::place::*;
use super::super::teigi::conv::*;

impl Uchu {
    /**
     * らいおんの位置
     */
    pub fn get_ms_r(&self, jiai: &Person) -> umasu {
        self.ky.ms_r[sn_to_num(&self.get_teban(jiai))]
    }
}
