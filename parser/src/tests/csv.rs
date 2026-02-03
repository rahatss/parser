#[cfg(test)]
mod tests {
    use crate::*;
    use std::io::Cursor;

    #[test]
    fn csv_round() {
        let data = vec![Transaction {
            id: 1,
            account: "ACC".to_string(),
            amount: 100,
            currency: "USD".to_string(),
        }];

        let mut buf = Cursor::new(Vec::new());

        csv::Csv::write(&mut buf, &data).expect("csv write failed");

        buf.set_position(0);

        let parsed = csv::Csv::read(&mut buf).expect("csv read failed");

        assert_eq!(data, parsed);
    }

    #[test]
    fn csv_round_ne() {
        let data = vec![Transaction {
            id: 1,
            account: "ACC".to_string(),
            amount: 100,
            currency: "USD".to_string(),
        }];

        let mut buf = Cursor::new(Vec::new());

        csv::Csv::write(&mut buf, &data).expect("csv write failed");

        buf.set_position(0);

        let mut parsed = csv::Csv::read(&mut buf).expect("csv read failed");
        parsed.push(Transaction{
            id: 0,
            account: "".to_string(),
            amount: 0,
            currency: "".to_string(),
        });

        assert_ne!(data, parsed);
    }
}