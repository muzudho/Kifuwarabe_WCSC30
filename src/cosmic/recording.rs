//!
//! * History (棋譜)
//! * Movement (指し手)
//! * Phase (先後。手番,相手番)
//! * Person (先手,後手)
//!
use crate::cosmic::smart::features::HandAddressType;
use crate::cosmic::smart::square::AbsoluteAddress;
use crate::law::cryptographic::num_to_lower_case;
use crate::law::generate_move::Piece;
use std::fmt;

/// 手目数。何手目まで指せるか。
/// 棋譜を残す配列のサイズでもある。
/// 大会ルールで 320手が上限なので、終端子として投了を１個入れておけるように +1 する。
pub const PLY_LEN: usize = 321;

/// 同一局面何回で千日手
pub const SENNTITE_NUM: isize = 4;

pub struct History {
    /// 手目。増減するので符号付きにしておくぜ☆（＾～＾）i8 は -128～127 なんで手数が収まらん☆（＾～＾）
    pub ply: isize,
    /// 棋譜
    /// TODO 0手目を初期局面にしたいので、最初にパスを入れてほしい☆（＾～＾）
    pub movements: [Movement; PLY_LEN],
    /// 棋譜に対応した各局面の局面ハッシュ
    pub position_hashs: [u64; PLY_LEN],
    /// 取った駒
    pub captured_pieces: [Option<Piece>; PLY_LEN],
}
impl Default for History {
    fn default() -> History {
        History {
            ply: 0,
            movements: [Movement::default(); PLY_LEN],
            position_hashs: [0; PLY_LEN],
            /// 取った駒
            captured_pieces: [None; PLY_LEN],
        }
    }
}
impl History {
    /// 手番
    pub fn get_friend(&self) -> Phase {
        // 手番
        if self.ply % 2 == 0 {
            Phase::First
        } else {
            Phase::Second
        }
    }
}

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

    /*
    pub fn from_hash(hash: u64) -> Option<Movement> {
        if hash == 0 {
            None
        } else {
            // 逆順で押し込んであるんで、正順に引き出す☆（＾～＾）
            let (hash, src52) = pop_sq_from_hash(hash);
            let (hash, dst53) = pop_sq_from_hash(hash);
            let (hash, pro54) = pop_bool_from_hash(hash);
            let (_hash, drop55) = pop_drop_from_hash(hash);
            Some(Movement::new(src52, dst53.unwrap(), pro54, drop55))
        }
    }

    pub fn to_hash(&self) -> u64 {
        let mut hash = 0;
        // 正順で取り出すことを考えて、逆順で押し込む☆（＾～＾）
        hash = push_drop_to_hash(hash, self.drop);
        hash = push_bool_to_hash(hash, self.promote);
        hash = push_sq_to_hash(hash, Some(&self.destination));
        push_sq_to_hash(hash, self.source.as_ref())
    }
    */

    pub fn set(&mut self, b: &Movement) {
        self.source = b.source;
        self.destination = b.destination;
        self.promote = b.promote;
        self.drop = b.drop;
    }
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
                source_val.address()
            } else {
                0
            },
            self.destination.address(),
            self.promote,
            if let Some(drp) = self.drop {
                format!("{:?}", drp)
            } else {
                "-".to_string()
            }
        )
    }
}

/*
///
/// 先後。単純にプレイヤー１を先手、プレイヤー２を後手とする。
/// 駒落ち戦での通称　上手／下手　の場合、上手は先手、下手は後手とする。
///
/// #[derive(PartialEq)]
pub enum Person {
    Friend,
    _Opponent,
}
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Person::*;
        match *self {
            Friend => write!(f, "Fr"),
            _Opponent => write!(f, "Op"),
        }
    }
}
*/
/*
pub fn turn_person(person: &Person) -> Person {
    use self::Person::*;
    match *person {
        Friend => Opponent,
        Opponent => Friend,
    }
}
*/

/// 局面ハッシュを作るときに、フェーズ用に配列があって、それのサイズに使ってるぜ☆（＾～＾）
pub const PHASE_FIRST: usize = 0;
pub const PHASE_SECOND: usize = 1;
pub const PHASE_LEN: usize = 2;

/// 先後。単純にプレイヤー１を先手、プレイヤー２を後手とする。
/// 駒落ち戦での通称　上手／下手　の場合、上手は先手、下手は後手とする。
#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    First,
    Second,
}
impl Phase {
    pub fn turn(self) -> Phase {
        use self::Phase::*;
        match self {
            First => Second,
            Second => First,
        }
    }
}
/// 後手（上手）を盤の下側に持ってきて表示するのを基本とするぜ☆（＾～＾）
impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        // Windows Terminal では ▲、▽が半角サイズで表示されるので、それに合わせている☆（＾～＾） Microsoft 製品に最適化していいのか知らないが……☆（＾～＾）
        use self::Phase::*;
        match *self {
            First => write!(f, " ▲"),
            Second => write!(f, " ▽"),
        }
    }
}
