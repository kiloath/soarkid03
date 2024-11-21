// #pragma execution_character_set("utf-8")
// #define UTF8
#include "soarkid03.hpp"
#include <iostream>

int main() {
  const char *m_type = "T";    /* A 市場別 */
  const char *stock = "0882";  /* D 股票代碼 */
  const char *t_type = "現股"; /* F 委託別(1:現股, 2:融資, 3:融券) */
  const char *bs = "B";        /* G 買賣別 */
  long long qty = 1000;        /* H 成交股數 */
  double price = 15.95;        /* I 成交價 */
  long long amt = 15972;       /* K 投資成本 */
  double c_price = 13.37;      /* L 現價 */
  long long net = 13337;       /* N 帳面收入 */
  long long fee = 22;          /* R 買進(賣出)手續費 */
  double tax_rate = 0.001;     /* S 交易稅 */
  const char *s_type = "4";    /* U 股票類別 */
  double fee_rate = 0.001425;  /* AB 手續費率 */
  long long min_fee = 20;      /* AC 最低手續費 */
  double market_price = 13.37; /* T 即時現價 */
#ifdef UTF8
  PL pl = get_real_pl_detail_utf8(m_type, stock, t_type, bs, qty, price, amt,
                                  c_price, net, fee, tax_rate, s_type, fee_rate,
                                  min_fee, market_price);
#else
  PL pl = get_real_pl_detail_big5(m_type, stock, t_type, bs, qty, price, amt,
                                  c_price, net, fee, tax_rate, s_type, fee_rate,
                                  min_fee, market_price);
#endif
  std::cout << pl.net << "," << pl.amt << "," << pl.profit << "," << pl.ratio
            << std::endl;
  // std::cout << pl.net << "," << pl.amt << "," << pl.profit << "," << pl.ratio
  // << "," << pl.zh_tw << std::endl;
  // free_string(pl.zh_tw);
  return 0;
}