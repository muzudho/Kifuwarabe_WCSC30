use crate::position::rotation::{Angle, Degree45Orthant};
use crate::position::RelAdr;
use std::fmt;

impl RelAdr {
    pub fn new(file: i8, rank: i8) -> Self {
        RelAdr {
            file: file,
            rank: rank,
        }
    }

    pub fn file(&self) -> i8 {
        self.file
    }
    pub fn rank(&self) -> i8 {
        self.rank
    }

    /// # Arguments
    ///
    /// * `r` - (Relative file, relative rank).
    pub fn number(&self) -> i8 {
        10 * self.file + self.rank
    }

    /// # Arguments
    ///
    /// * `r` - (Relative file, relative rank).
    pub fn rotate_180(&mut self) -> &mut Self {
        self.file *= -1;
        self.rank *= -1;
        self
    }

    /// Counterclockwise
    ///
    /// # Arguments
    ///
    /// * `r` - (Relative file, relative rank).
    pub fn rotate_90_ccw(&mut self) -> &mut Self {
        // 象限は、何度回転するかによって境界線の位置が変わってくるので、回転の直前で調べるしかないぜ☆（＾～＾）
        // でも、 90°回転のときは 象限は１つしかないけどな☆（＾～＾）全象限同じ式だぜ☆（*＾～＾*）
        let new_file = -self.rank;
        let new_rank = self.file;
        self.file = new_file;
        self.rank = new_rank;
        self
    }

    /// Counterclockwise
    ///
    /// # Arguments
    ///
    /// * `r` - (Relative file, relative rank).
    pub fn rotate_45_ccw(&mut self) -> &mut Self {
        // 象限は、何度回転するかによって境界線の位置が変わってくるので、回転の直前で調べるしかないぜ☆（＾～＾）
        let orthant = Degree45Orthant::new(self);
        match orthant {
            Degree45Orthant::IVOrI => {
                let distance = self.file;
                let mut file2 = self.file;
                let mut rank2 = self.rank + distance;
                let over = rank2.abs() - distance.abs();
                if 0 < over {
                    rank2 = distance;
                    file2 -= over;
                }
                self.file = file2;
                self.rank = rank2;
                self
            }
            Degree45Orthant::IIOrIII => {
                let distance = self.file;
                let mut file2 = self.file;
                let mut rank2 = self.rank + distance;
                let over = rank2.abs() - distance.abs();
                if 0 < over {
                    rank2 = distance;
                    file2 += over;
                }
                self.file = file2;
                self.rank = rank2;
                self
            }
            Degree45Orthant::CoIOrCoII => {
                let distance = self.rank;
                let mut file2 = self.file - distance;
                let mut rank2 = self.rank;
                let over = rank2.abs() - distance.abs();
                if 0 < over {
                    file2 = distance;
                    rank2 -= over;
                }
                self.file = file2;
                self.rank = rank2;
                self
            }
            Degree45Orthant::CoIIIOrCoIV => {
                let distance = self.rank;
                let mut file2 = self.file - distance;
                let mut rank2 = self.rank;
                let over = rank2.abs() - distance.abs();
                if 0 < over {
                    file2 = distance;
                    rank2 -= over;
                }
                self.file = file2;
                self.rank = rank2;
                self
            }
        }
    }

    /// 反時計回り(Counterclockwise;ccw)に回します
    ///
    /// # Arguments
    ///
    /// * `r` - (Relative file, relative rank).
    pub fn rotate_ccw(&mut self, angle: Angle) -> &mut Self {
        use crate::position::rotation::Angle::*;
        match angle {
            Ccw0 => self,
            Ccw45 => self.rotate_45_ccw(),
            Ccw90 => self.rotate_90_ccw(),
            Ccw135 => self.rotate_45_ccw().rotate_90_ccw(),
            Ccw180 => self.rotate_180(),
            Ccw225 => self.rotate_45_ccw().rotate_180(),
            Ccw270 => self.rotate_90_ccw().rotate_180(),
            Ccw315 => self.rotate_45_ccw().rotate_90_ccw().rotate_180(),
        }
    }

    /// 段を２倍にします。桂馬に使います。
    ///
    /// # Arguments
    ///
    /// * `r` - (Relative file, relative rank).
    pub fn double_rank(&mut self) -> &mut Self {
        let rank2 = 2 * self.rank;
        let carry = rank2 / 10;
        let file2 = if carry != 0 {
            self.file + carry
        } else {
            self.file
        };
        self.file = file2;
        self.rank = rank2;
        self
    }
}
/// 回転してみるまで象限は分からないので、出せるのは筋、段、相対番地だけだぜ☆（＾～＾）
impl fmt::Debug for RelAdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}x {}y rel{}sq)", self.file, self.rank, self.number())
    }
}
