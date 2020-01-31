//!
//! 盤、升、筋、段
//!

/*
 * 盤の符号は、後手番から見る
 *
 *
 * 19  29  39  49  59  69  79  89  99
 * 18  28  38  48  58  68  78  88  98
 * 17  27  37  47  57  67  77  87  97
 * 16  26  36  46  56  66  76  86  96
 * 15  25  35  45  55  65  75  85  95
 * 14  24  34  44  54  64  74  84  94
 * 13  23  33  43  53  63  73  83  93
 * 12  22  32  42  52  62  72  82  92
 * 11  21  31  41  51  61  71  81  91
 *
 */
/**
 * 盤を回転するのに使うぜ☆（＾～＾）
 */
pub const BAN_MIN: usize = 11;
/**
 * 盤を回転するのに使うぜ☆（＾～＾）
 */
pub const BAN_MAX: usize = 99;
/**
 * 盤のヨコ幅、タテ幅。
 * 筋と段は x,y とは逆方向なので、幅も左端、下端を指す。
 */
//pub const BAN_W :i8 = 9;
//pub const BAN_H :i8 = 9;
pub const BAN_SIZE: usize = 100;
// 1辺の長さ
//pub const BAN_LINE :usize = 10;
/**
 * 筋、段は 1 から始まる、という明示。
 * 増減はよく使うので u8 ではなく i8 にした。
 */
pub const SUJI_0: i8 = 0;
pub const SUJI_1: i8 = 1;
pub const SUJI_9: i8 = 9;
pub const SUJI_10: i8 = 10;
pub const DAN_0: i8 = 0;
pub const DAN_1: i8 = 1;
pub const DAN_2: i8 = 2;
pub const DAN_3: i8 = 3;
pub const DAN_4: i8 = 4;
pub const DAN_5: i8 = 5;
pub const DAN_6: i8 = 6;
pub const DAN_7: i8 = 7;
pub const DAN_8: i8 = 8; //うさぎの打てる段の上限
pub const DAN_9: i8 = 9;
pub const DAN_10: i8 = 10;
/**
 * 升番号 0～99。
 * 10の位を筋、1の位を段とする。0筋、0段は未使用（番兵として使用）
 * 該当なしの場合 0 を使う
 */
#[allow(non_camel_case_types)]
pub type umasu = usize;
/**
 * 升の検索等で、該当なしの場合
 */
pub const MASU_0: umasu = 0;

/**
 * 指し手。打の場合のsrc
 */
pub const SS_SRC_DA: umasu = 0;
