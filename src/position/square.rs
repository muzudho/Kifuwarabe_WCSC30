use crate::entities::cosmic::smart::square::FILE_0;
use crate::entities::cosmic::smart::square::FILE_10;
use crate::entities::cosmic::smart::square::RANK_0;
use crate::entities::cosmic::smart::square::RANK_10;
use crate::position::RelAdr;
use crate::position::Square;

impl Square {
    pub fn new(number: u8) -> Self {
        Square(number)
    }
    pub fn from(file: u8, rank: u8) -> Self {
        Square(file * 10 + rank)
    }
    pub fn offset(&self, r: &RelAdr) -> Self {
        // TODO rankの符号はどうだったか……☆（＾～＾） 絶対番地の使い方をしてれば問題ないだろ☆（＾～＾）
        // TODO sum は負数になることもあり、そのときは明らかにイリーガルだぜ☆（＾～＾）
        let sum = (self.0 as isize + r.get_address()) as u8;
        // Initialize.
        let mut rank = sum % 10;
        let mut file = 0;
        // Carry.
        if 9 < rank {
            rank = rank % 10;
            file += 1;
        }
        file += sum / 10 % 10;
        // Carry over flow.
        if 9 < file {
            file = file % 10;
        }
        Square::from(file, rank)
    }

    pub fn rotate_180(&self) -> Self {
        let file = FILE_10 - self.file();
        let rank = RANK_10 - self.rank();
        debug_assert!(FILE_0 < file && file < FILE_10, "file={}", file);
        debug_assert!(RANK_0 < rank && rank < RANK_10, "rank={}", rank);
        Square::from(file, rank)
    }
    pub fn number(&self) -> u8 {
        self.0
    }
    /// 盤上のマスなら真。（調べ方は、ざっくり）
    pub fn is_board(&self) -> bool {
        11 <= self.0 && self.0 < 100
    }
    /// 持駒なら真
    pub fn is_hand(&self) -> bool {
        100 <= self.0
    }
    //  /// マスでないなら真
    // pub fn is_none_square(&self) -> bool {
    //     self.0 == SQUARE_NONE
    // }
    /// マス、または持駒なら真
    pub fn is_square(&self) -> bool {
        (11 <= self.0 && self.0 < 20)
            || (21 <= self.0 && self.0 < 30)
            || (31 <= self.0 && self.0 < 40)
            || (41 <= self.0 && self.0 < 50)
            || (51 <= self.0 && self.0 < 60)
            || (61 <= self.0 && self.0 < 70)
            || (71 <= self.0 && self.0 < 80)
            || (81 <= self.0 && self.0 < 90)
            || (91 <= self.0 && self.0 < 100)
            || (100 <= self.0 && self.0 < 116)
    }

    pub fn rank(&self) -> u8 {
        self.0 % 10
    }
    pub fn file(&self) -> u8 {
        self.0 / 10
    }
    /// 壁の中にいる☆（＾～＾）
    pub fn wall(&self) -> bool {
        self.file() % 10 == 0 || self.rank() % 10 == 0
    }

    pub fn to_drop_code(&self) -> &str {
        match self.0 {
            101 | 109 => "R*",
            102 | 110 => "B*",
            103 | 111 => "G*",
            104 | 112 => "S*",
            105 | 113 => "N*",
            106 | 114 => "L*",
            107 | 115 => "P*",
            _ => panic!("(Err.46) drop fail"),
        }
    }
}
