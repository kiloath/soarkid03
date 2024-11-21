use encoding_rs::BIG5;
use std::ffi::{c_char, CStr, CString};

use super::PL;

pub fn big5_to_utf8_2(big5: *const c_char) -> String {
    let c_str = unsafe { CStr::from_ptr(big5) };
    let big5_bytes = c_str.to_bytes();
    let (utf8_string, _, _) = BIG5.decode(big5_bytes);
    utf8_string.into_owned()
}

pub fn core_get_real_pl_detail(
    m_type: &str,      /* A 市場別 */
    stock: &str,       /* D 股票代碼 */
    t_type: &str,      /* F 委託別(1:現股, 2:融資, 3:融券) */
    bs: &str,          /* G 買賣別 */
    qty: f64,          /* H 成交股數 */
    price: f64,        /* I 成交價 */
    amt: f64,          /* K 投資成本 */
    c_price: f64,      /* L 現價 */
    net: f64,          /* N 帳面收入 */
    fee: f64,          /* R 買進(賣出)手續費 */
    tax_rate: f64,     /* S 交易稅 */
    s_type: &str,      /* U 股票類別 */
    fee_rate: f64,     /* AB 手續費率 */
    min_fee: f64,      /* AC 最低手續費 */
    market_price: f64, /* T 即時現價 */
) -> PL {
    // 計算手續費
    let new_fee = (qty * market_price * fee_rate).floor();
    let new_fee = new_fee.max(min_fee);
    // 計算交易稅
    let new_tax = if bs == "B" {
        (qty * market_price * tax_rate).floor()
    } else {
        0.0
    };
    // 計算融資常數
    let c_factor = 0.0;
    // 計算融券常數
    let d_factor = 0.0;
    // 計算帳面收入
    let net = match t_type {
        "1" if bs == "B" => qty * market_price - new_fee - new_tax,
        "2" => qty * market_price - new_fee - new_tax - c_factor,
        "3" => d_factor - qty * market_price - new_fee,
        _ => net,
    };
    // 計算投資成本
    let amt = match (t_type, bs) {
        ("1", "S") => (qty * market_price).floor() + new_fee,
        _ => amt,
    };
    // 計算損益
    let profit = match t_type {
        "1" if bs == "S" => amt - net,
        _ => net - amt,
    };
    // 計算投資報酬率
    let ratio = (profit / amt * 10000.0).round() / 100.0;

    // let zh_tw = CString::new("中文").expect("CString::new failed").into_raw();

    PL {
        net,
        amt,
        profit,
        ratio,
        // zh_tw,
    }
}
