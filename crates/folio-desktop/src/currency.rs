// Copyright (C) 2025 Wakunguma Kalimukwa
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
#![allow(unused)]

// TODO: document removed currencies, check if settings will be parsed
// Removed currencies:
// - XPT - Platinum
// - XTS - Code reserved for testing
// - XSU - Unified System for Regional Compensation (SUCRE)
// - XXX - No currency
// - XDR - Special drawing rights
// - XBB - European Composite Unit
// - XBB - European Monetary Unit
// - XBC - European Unit of Account 9
// - XBD - European Unit of Account 17
// - XUA - Africa Development Bank unit of account
// - XAG - Silver
// - XPD - Palladium
// - XAU - Gold

use std::fmt::Display;
use crate::Error;
use serde::{Deserialize, Serialize};
use std::string::ToString;

macro_rules! generate_currencies {
    ($($code:ident;$name:expr;$symbol:expr;$precision:expr),+) => {
        impl Currency{
            $(
                #[doc = $name]
                pub const $code: Currency = Currency::new($name,stringify!($code),$symbol,$precision);
            )+

            /// All the supported currencies.
            pub const ALL_CURRENCIES: [Currency;169] = [
                $(Currency::$code),+
            ];
        }

        impl std::str::FromStr for Currency{
            type Err = Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($code) => Ok(Currency::$code),)+
                     _ => Err(Error::new("Unsupported currency: {s}"))
                }
            }
        }
    };
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Serialize, Deserialize, Debug)]
pub struct Currency {
    name: &'static str,
    /// The ISO 4217 currency code.
    code: &'static str,
    /// The local currency symbol.
    ///
    /// This is the symbol used for display in the respective countries,
    /// so for example `USD`, `CAD` and `AUD` are all `$` as opposed to `US$`, `CA$` and `AU$`.
    symbol: Option<&'static str>,
    /// The number of digits after the decimal.
    precision: Option<u8>,
}

// TODO: add constants, use macro
impl Currency {
    const fn new(
        name: &'static str,
        code: &'static str,
        symbol: Option<&'static str>,
        precision: Option<u8>,
    ) -> Self {
        Self {
            name,
            code,
            symbol,
            precision,
        }
    }

    /// Returns the name of the currency, e.g. "South African Rand".
    pub fn name(&self) -> &str {
        self.name
    }

    /// Returns the ISO 4217 currency code, e.g. "ZAR".
    pub fn code(&self) -> &str {
        self.code
    }

    /// Returns the local symbol of the currency. If there is no symbol, it will return the currency code.
    pub fn symbol(&self) -> &str {
        match &self.symbol {
            Some(symbol) => symbol,
            None => self.code,
        }
    }

    /// Returns the number of digits after the decimal point. This returns an `Option` because not every
    /// currency has subunits.
    pub fn precision(&self) -> Option<u8> {
        self.precision
    }
}

impl Display for Currency{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code.to_owned())
    }
}

generate_currencies! {
    AED; "United Arab Emirates dirham"; Some("د.إ"); Some(2),
    AFN; "Afghan afghani"; Some("؋"); Some(2),
    ALL; "Albanian lek"; Some("L"); Some(2),
    AMD; "Armenian dram"; Some("֏"); Some(2),
    XCG; "Caribean guilder"; Some("ƒ"); Some(2),
    AOA; "Angolan kwanza"; Some("Kz"); Some(2),
    ARS; "Argentine peso"; Some("$"); Some(2),
    AUD; "Australian dollar"; Some("$"); Some(2),
    AWG; "Aruban florin"; Some("ƒ"); Some(2),
    AZN; "Azerbaijani manat"; Some("₼"); Some(2),
    BAM; "Bosnia and Herzegovina convertible mark"; Some("KM"); Some(2),
    BBD; "Barbados dollar"; Some("Bds$"); Some(2),
    BDT; "Bangladeshi taka"; Some("৳"); Some(2),
    BGN; "Bulgarian lev"; Some("лв."); Some(2),
    BHD; "Bahraini dinar"; Some(".د.ب"); Some(3),
    BIF; "Burundian franc"; Some("FBu"); Some(0),
    BMD; "Bermudian dollar"; Some("$"); Some(2),
    BND; "Brunei dollar"; Some("B$"); Some(2),
    BOB; "Boliviano"; Some("Bs."); Some(2),
    BOV; "Bolivian Mvdol"; None; Some(2),
    BRL; "Brazilian real"; Some("R$"); Some(2),
    BSD; "Bahamian dollar"; Some("$"); Some(2),
    BTN; "Bhutanese ngultrum"; Some("Nu."); Some(2),
    BWP; "Botswana pula"; Some("P"); Some(2),
    BYN; "Belarusian ruble"; Some("Br"); Some(2),
    BZD; "Belize dollar"; Some("$"); Some(2),
    CAD; "Canadian dollar"; Some("$"); Some(2),
    CDF; "Congolese franc"; Some("₣"); Some(2),
    CHE; "WIR Euro"; None; Some(2),
    CHF; "Swiss franc"; Some("₣"); Some(2),
    CHW; "WIR Franc"; Some("¤"); Some(2),
    CLF; "Unidad de Fomento"; None; Some(4),
    CLP; "Chilean peso"; Some("$"); Some(0),
    CNY; "Renminbi (Chinese) yuan"; Some("¥"); Some(2),
    COP; "Colombian peso"; Some("$"); Some(2),
    COU; "Unidad de Valor Real (UVR)"; None; Some(2),
    CRC; "Costa Rican colon"; Some("₡"); Some(2),
    CUC; "Cuban convertible peso"; Some("$"); Some(2),
    CUP; "Cuban peso"; Some("₱"); Some(2),
    CVE; "Cape Verdean escudo"; Some("Esc"); Some(2),
    CZK; "Czech koruna"; Some("Kč"); Some(2),
    DJF; "Djiboutian franc"; Some("₣"); Some(0),
    DKK; "Danish krone"; Some("kr"); Some(2),
    DOP; "Dominican peso"; Some("RD$"); Some(2),
    DZD; "Algerian dinar"; Some("دج"); Some(2),
    EGP; "Egyptian pound"; Some("£"); Some(2),
    ERN; "Eritrean nakfa"; Some("Nfk"); Some(2),
    ETB; "Ethiopian birr"; Some("Br"); Some(2),
    EUR; "Euro"; Some("€"); Some(2),
    FJD; "Fiji dollar"; Some("FJ$"); Some(2),
    FKP; "Falkland Islands pound"; Some("£"); Some(2),
    GBP; "Pound sterling"; Some("£"); Some(2),
    GEL; "Georgian lari"; Some("ლ"); Some(2),
    GHS; "Ghanaian cedi"; Some("GH₵"); Some(2),
    GIP; "Gibraltar pound"; Some("£"); Some(2),
    GMD; "Gambian dalasi"; Some("D"); Some(2),
    GNF; "Guinean franc"; Some("₣"); Some(0),
    GTQ; "Guatemalan quetzal"; Some("Q"); Some(2),
    GYD; "Guyanese dollar"; Some("G$"); Some(2),
    HKD; "Hong Kong dollar"; Some("HK$"); Some(2),
    HNL; "Honduran lempira"; Some("L"); Some(2),
    HRK; "Croatian kuna"; Some("kn"); Some(2),
    HTG; "Haitian gourde"; Some("G"); Some(2),
    HUF; "Hungarian forint"; Some("Ft"); Some(2),
    IDR; "Indonesian rupiah"; Some("Rp"); Some(2),
    ILS; "Israeli new shekel"; Some("₪"); Some(2),
    INR; "Indian rupee"; Some("₹"); Some(2),
    IQD; "Iraqi dinar"; Some("د.ع"); Some(3),
    IRR; "Iranian rial"; Some("﷼"); Some(2),
    ISK; "Icelandic króna"; Some("kr"); Some(0),
    JMD; "Jamaican dollar"; Some("$"); Some(2),
    JOD; "Jordanian dinar"; Some("JD"); Some(3),
    JPY; "Japanese yen"; Some("¥"); Some(0),
    KES; "Kenyan shilling"; Some("Ksh"); Some(2),
    KGS; "Kyrgyzstani som"; Some("С̲"); Some(2),
    KHR; "Cambodian riel"; Some("៛"); Some(2),
    KMF; "Comoro franc"; Some("₣"); Some(0),
    KPW; "North Korean won"; Some("₩"); Some(2),
    KRW; "South Korean won"; Some("₩"); Some(0),
    KWD; "Kuwaiti dinar"; Some("د.ك"); Some(3),
    KYD; "Cayman Islands dollar"; Some("$"); Some(2),
    KZT; "Kazakhstani tenge"; Some("₸"); Some(2),
    LAK; "Lao kip"; Some("₭"); Some(2),
    LBP; "Lebanese pound"; Some("LL"); Some(2),
    LKR; "Sri Lankan rupee"; Some("₨"); Some(2),
    LRD; "Liberian dollar"; Some("L$"); Some(2),
    LSL; "Lesotho loti"; Some("M"); Some(2),
    LYD; "Libyan dinar"; Some("ل.د"); Some(3),
    MAD; "Moroccan dirham"; Some("د.م."); Some(2),
    MDL; "Moldovan leu"; None; Some(2),
    MGA; "Malagasy ariary"; Some("Ar"); Some(2),
    MKD; "Macedonian denar"; Some("ден"); Some(2),
    MMK; "Myanmar kyat"; Some("K"); Some(2),
    MNT; "Mongolian tögrög"; Some("₮"); Some(2),
    MOP; "Macanese pataca"; Some("MOP$"); Some(2),
    MRU; "Mauritanian ouguiya"; Some("UM"); Some(2),
    MUR; "Mauritian rupee"; Some("₨"); Some(2),
    MVR; "Maldivian rufiyaa"; Some("Rf."); Some(2),
    MWK; "Malawian kwacha"; Some("K"); Some(2),
    MXN; "Mexican peso"; Some("$"); Some(2),
    MXV; "Mexican Unidad de Inversion (UDI)"; None; Some(2),
    MYR; "Malaysian ringgit"; Some("RM"); Some(2),
    MZN; "Mozambican metical"; Some("MT"); Some(2),
    NAD; "Namibian dollar"; Some("N$"); Some(2),
    NGN; "Nigerian naira"; Some("₦"); Some(2),
    NIO; "Nicaraguan córdoba"; Some("C$"); Some(2),
    NOK; "Norwegian krone"; Some("kr"); Some(2),
    NPR; "Nepalese rupee"; Some("₨"); Some(2),
    NZD; "New Zealand dollar"; Some("$"); Some(2),
    OMR; "Omani rial"; Some("ر.ع."); Some(3),
    PAB; "Panamanian balboa"; Some("B/."); Some(2),
    PEN; "Peruvian sol"; Some("S/"); Some(2),
    PGK; "Papua New Guinean kina"; Some("K"); Some(2),
    PHP; "Philippine peso"; Some("₱"); Some(2),
    PKR; "Pakistani rupee"; Some("₨"); Some(2),
    PLN; "Polish złoty"; Some("zł"); Some(2),
    PYG; "Paraguayan guaraní"; Some("₲"); Some(0),
    QAR; "Qatari riyal"; Some("ر.ق"); Some(2),
    RON; "Romanian leu"; Some("L"); Some(2),
    RSD; "Serbian dinar"; Some("дин"); Some(2),
    RUB; "Russian ruble"; Some("₽"); Some(2),
    RWF; "Rwandan franc"; Some("FRw"); Some(0),
    SAR; "Saudi riyal"; Some("ر.س"); Some(2),
    SBD; "Solomon Islands dollar"; Some("S$"); Some(2),
    SCR; "Seychelles rupee"; Some("SRe"); Some(2),
    SDG; "Sudanese pound"; None; Some(2),
    SEK; "Swedish krona/kronor"; Some("kr"); Some(2),
    SGD; "Singapore dollar"; Some("S$"); Some(2),
    SHP; "Saint Helena pound"; Some("£"); Some(2),
    SLE; "Sierra Leonean leone"; Some("Le"); Some(2),
    SLL; "Sierra Leonean leone"; Some("Le"); Some(2),
    SOS; "Somali shilling"; Some("Sh.So."); Some(2),
    SRD; "Surinamese dollar"; Some("$"); Some(2),
    SSP; "South Sudanese pound"; None; Some(2),
    STN; "São Tomé and Príncipe dobra"; Some("Db"); Some(2),
    SVC; "Salvadoran colón"; None; Some(2),
    SYP; "Syrian pound"; Some("LS"); Some(2),
    SZL; "Swazi lilangeni"; Some("E"); Some(2),
    THB; "Thai baht"; Some("฿"); Some(2),
    TJS; "Tajikistani somoni"; None; Some(2),
    TMT; "Turkmenistan manat"; None; Some(2),
    TND; "Tunisian dinar"; Some("د.ت"); Some(3),
    TOP; "Tongan paʻanga"; Some("T$"); Some(2),
    TRY; "Turkish lira"; Some("₺"); Some(2),
    TTD; "Trinidad and Tobago dollar"; Some("$"); Some(2),
    TWD; "New Taiwan dollar"; Some("NT$"); Some(2),
    TZS; "Tanzanian shilling"; Some("Tsh"); Some(2),
    UAH; "Ukrainian hryvnia"; Some("₴"); Some(2),
    UGX; "Ugandan shilling"; Some("USh"); Some(0),
    USD; "United States dollar"; Some("$"); Some(2),
    USN; "United States dollar (next day)"; Some("$"); Some(2),
    UYI; "Uruguay Peso en Unidades Indexadas (URUIURUI)"; Some("$U"); Some(0),
    UYU; "Uruguayan peso"; Some("$U"); Some(2),
    UYW; "Unidad previsional"; Some("¤"); Some(4),
    UZS; "Uzbekistan som"; Some("¤"); Some(2),
    VED; "Venezuelan bolívar soberano"; Some("Bs."); Some(2),
    VES; "Venezuelan bolívar soberano"; Some("Bs."); Some(2),
    VND; "Vietnamese đồng"; Some("₫"); Some(0),
    VUV; "Vanuatu vatu"; Some("VT"); Some(0),
    WST; "Samoan tala"; Some("WS$"); Some(2),
    XAF; "CFA franc BEAC"; Some("FCFA"); Some(0),
    XCD; "East Caribbean dollar"; Some("$"); Some(2),
    XOF; "CFA franc BCEAO"; Some("CFA"); Some(0),
    XPF; "CFP franc (franc Pacifique)"; Some("₣"); Some(0),
    YER; "Yemeni rial"; Some("ر.ي"); Some(2),
    ZAR; "South African rand"; Some("R"); Some(2),
    ZMW; "Zambian kwacha"; Some("K"); Some(2),
    ZWL; "Zimbabwean dollar"; Some("$"); Some(2),
    ZWG; "Zimbabwe Gold"; None; Some(2)
}
