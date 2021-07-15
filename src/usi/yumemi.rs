use crate::entities::spaceship::equipment::Telescope;
use crate::usi::Yumemi;

impl Yumemi {
    /// 望遠鏡を覗き込みましょう。
    pub fn look_into_the_telescope() {
        Telescope::look();
    }
}
