use crate::entities::cosmic::smart::features::HandAddressType;
use crate::entities::cosmic::smart::square::AbsoluteAddress;
use crate::entities::law::cryptographic::num_to_lower_case;
use std::fmt;

/// 棋譜にも使うので、取った駒の情報を記憶しておくんだぜ☆（＾～＾）
/// 投了なら これを使わず、None にしろだぜ☆（＾～＾）
///
/// Copy: 配列の要素の初期化時に使う☆（＾～＾）
#[derive(Clone, Copy)]
pub struct Movement {
    // 移動元升。Dropのときは None だぜ☆（＾～＾）
    pub source: Option<AbsoluteAddress>,
    // 移動先升。
    pub destination: AbsoluteAddress,
    // 移動後に成るなら真
    pub promote: bool,
    // 打の場合、打った駒種類
    pub drop: Option<HandAddressType>,
}
impl Default for Movement {
    /// ゴミの値を作るぜ☆（＾～＾）
    fn default() -> Self {
        Movement {
            source: None,
            destination: AbsoluteAddress::default(),
            promote: false,
            drop: None,
        }
    }
}
impl Movement {
    pub fn new(
        source: Option<AbsoluteAddress>,
        destination: AbsoluteAddress,
        promote: bool,
        drop: Option<HandAddressType>,
    ) -> Self {
        Movement {
            source: source,
            destination: destination,
            promote: promote,
            drop: drop,
        }
    }

    // pub fn from_hash(hash: u64) -> Option<Movement> {
    //     if hash == 0 {
    //         None
    //     } else {
    //         // 逆順で押し込んであるんで、正順に引き出す☆（＾～＾）
    //         let (hash, src52) = pop_sq_from_hash(hash);
    //         let (hash, dst53) = pop_sq_from_hash(hash);
    //         let (hash, pro54) = pop_bool_from_hash(hash);
    //         let (_hash, drop55) = pop_drop_from_hash(hash);
    //         if let Some(dst) = dst53 {
    //             Some(Movement::new(src52, dst, pro54, drop55))
    //         } else {
    //             panic!("dst53={:?}",dst53)
    //         }
    //     }
    // }

    // pub fn to_hash(&self) -> u64 {
    //     let mut hash = 0;
    //     // 正順で取り出すことを考えて、逆順で押し込む☆（＾～＾）
    //     hash = push_drop_to_hash(hash, self.drop);
    //     hash = push_bool_to_hash(hash, self.promote);
    //     hash = push_sq_to_hash(hash, Some(&self.destination));
    //     push_sq_to_hash(hash, self.source.as_ref())
    // }

    // pub fn set(&mut self, b: &Movement) {
    //     self.source = b.source;
    //     self.destination = b.destination;
    //     self.promote = b.promote;
    //     self.drop = b.drop;
    // }
}
impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (dx, dy) = self.destination.to_file_rank();

        if let Some(source_val) = self.source {
            let (sx, sy) = source_val.to_file_rank();
            write!(
                f,
                "{}{}{}{}{}",
                sx,
                num_to_lower_case(sy),
                dx,
                num_to_lower_case(dy),
                if self.promote { "+" } else { "" }
            )
        } else {
            const DROPS: [&str; 8] = ["?", "R", "B", "G", "S", "N", "L", "P"];
            write!(
                f,
                "{}*{}{}{}",
                if let Some(drp) = self.drop {
                    DROPS[drp as usize]
                } else {
                    "?"
                },
                dx,
                num_to_lower_case(dy),
                if self.promote { "+" } else { "" }
            )
        }
    }
}
impl fmt::Debug for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Movement({}{}{}{})",
            if let Some(source_val) = self.source {
                source_val.square_number()
            } else {
                0
            },
            self.destination.square_number(),
            self.promote,
            if let Some(drp) = self.drop {
                format!("{:?}", drp)
            } else {
                "-".to_string()
            }
        )
    }
}
