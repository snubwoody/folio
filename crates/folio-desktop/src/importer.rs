use std::io::Read;

/// Loads a csv file.
///
/// The result will be sent to the frontend for the user to configure.
pub fn load_csv(reader: impl Read) -> crate::Result<Vec<csv::StringRecord>>{
    let mut csv_reader = csv::Reader::from_reader(reader);
    let mut records = vec![];
    for result in csv_reader.records(){
        records.push(result?);
    }
    Ok(records)
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn load_without_header() -> crate::Result<()>{
        let data = "
05/12/2020,PayPal,29.00,,29.00
05/12/2022,Credit card payment,,200.00,0.00
05/01/2022,Playstation,13.22,,13.22
05/01/2021,Walmart,245.35,,13.22
";
        let records = load_csv(data.as_bytes())?;
        dbg!(&records);
        assert_eq!(records.len(),4);
        Ok(())
    }
}