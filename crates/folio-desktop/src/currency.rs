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


pub struct Currency{
    name: String,
    /// The ISO 4217 currency code.
    code: String,
    /// The local currency symbol.
    ///
    /// This is the symbol used for display in the respective countries,
    /// so for example `USD`, `CAD` and `AUD` are all `$` as opposed to `US$`, `CA$` and `AU$`.
    symbol: Option<String>,
    /// The number of digits after the decimal.
    precision: Option<u8>
}

// TODO: add constants, use macro
impl Currency{
    fn new(name: &str){

    }
    /// Returns the name of the currency, e.g. "South African Rand".
    pub fn name(&self) -> &str{
        &self.name
    }

    /// Returns the ISO 4217 currency code, e.g. "ZAR".
    pub fn code(&self) -> &str{
        &self.code
    }

    /// Returns the local symbol of the currency. If there is no symbol, it will return the currency code.
    pub fn symbol(&self) -> &str{
        match &self.symbol {
            Some(symbol) => &symbol,
            None => &self.code
        }
    }

    /// Returns the number of digits after the decimal point. This returns an `Option` because not every
    /// currency has subunits.
    pub fn precision(&self) -> Option<u8> {
        self.precision
    }
}