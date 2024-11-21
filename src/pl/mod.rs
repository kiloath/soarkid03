mod pl;
use std::ffi::{c_char, c_double, c_longlong, CStr};
use pl::*;

#[repr(C)]
pub struct PL {
    pub net: f64,    /* 帳面收入 */
    pub amt: f64,    /* 投資成本 */
    pub profit: f64, /* 損益 */
    pub ratio: f64,  /* 投資報酬率 */
    // pub zh_tw: *const c_char,
}

#[no_mangle]
pub extern "C" fn get_real_pl_detail_big5(
    m_type: *const c_char,  /* A 市場別 */
    stock: *const c_char,   /* D 股票代碼 */
    t_type: *const c_char,  /* F 委託別(1:現股, 2:融資, 3:融券) */
    bs: *const c_char,      /* G 買賣別 */
    qty: c_longlong,        /* H 成交股數 */
    price: c_double,        /* I 成交價 */
    amt: c_longlong,        /* K 投資成本 */
    c_price: c_double,      /* L 現價 */
    net: c_longlong,        /* N 帳面收入 */
    fee: c_longlong,        /* R 買進(賣出)手續費 */
    tax_rate: c_double,     /* S 交易稅 */
    s_type: *const c_char,  /* U 股票類別 */
    fee_rate: c_double,     /* AB 手續費率 */
    min_fee: c_longlong,    /* AC 最低手續費 */
    market_price: c_double, /* T 即時現價 */
) -> PL {
    let m_type = big5_to_utf8_2(m_type);
    let stock = big5_to_utf8_2(stock);
    let t_type = big5_to_utf8_2(t_type);
    let bs = big5_to_utf8_2(bs);
    let s_type = big5_to_utf8_2(s_type);
    let qty = qty as f64;
    let price = price as f64;
    let amt = amt as f64;
    let c_price = c_price as f64;
    let net = net as f64;
    let fee = fee as f64;
    let tax_rate = tax_rate as f64;
    let fee_rate = fee_rate as f64;
    let min_fee = min_fee as f64;

    core_get_real_pl_detail(
        m_type.as_str(),
        stock.as_str(),
        t_type.as_str(),
        bs.as_str(),
        qty,
        price,
        amt,
        c_price,
        net,
        fee,
        tax_rate,
        s_type.as_str(),
        fee_rate,
        min_fee,
        market_price,
    )
}

#[no_mangle]
pub extern "C" fn get_real_pl_detail_utf8(
    m_type: *const c_char,  /* A 市場別 */
    stock: *const c_char,   /* D 股票代碼 */
    t_type: *const c_char,  /* F 委託別(1:現股, 2:融資, 3:融券) */
    bs: *const c_char,      /* G 買賣別 */
    qty: c_longlong,        /* H 成交股數 */
    price: c_double,        /* I 成交價 */
    amt: c_longlong,        /* K 投資成本 */
    c_price: c_double,      /* L 現價 */
    net: c_longlong,        /* N 帳面收入 */
    fee: c_longlong,        /* R 買進(賣出)手續費 */
    tax_rate: c_double,     /* S 交易稅 */
    s_type: *const c_char,  /* U 股票類別 */
    fee_rate: c_double,     /* AB 手續費率 */
    min_fee: c_longlong,    /* AC 最低手續費 */
    market_price: c_double, /* T 即時現價 */
) -> PL {
    let m_type = unsafe { CStr::from_ptr(m_type) }.to_str().unwrap();
    let stock = unsafe { CStr::from_ptr(stock) }.to_str().unwrap();
    let t_type = unsafe { CStr::from_ptr(t_type) }.to_str().unwrap();
    let bs = unsafe { CStr::from_ptr(bs) }.to_str().unwrap();
    let s_type = unsafe { CStr::from_ptr(s_type) }.to_str().unwrap();
    let qty = qty as f64;
    let price = price as f64;
    let amt = amt as f64;
    let c_price = c_price as f64;
    let net = net as f64;
    let fee = fee as f64;
    let tax_rate = tax_rate as f64;
    let fee_rate = fee_rate as f64;
    let min_fee = min_fee as f64;

    core_get_real_pl_detail(
        m_type,
        stock,
        t_type,
        bs,
        qty,
        price,
        amt,
        c_price,
        net,
        fee,
        tax_rate,
        s_type,
        fee_rate,
        min_fee,
        market_price,
    )
}
