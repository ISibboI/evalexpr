
mod tests {

    use std::path::PathBuf;
    use anyhow::Context;
    use chrono::NaiveDateTime;
    use evalexpr::*;
    use csv::Reader;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Record {
        #[serde(rename = "dt")]
        datetime: String,
        close: f64,
    }


    fn read_csv(file_path: &str) -> anyhow::Result<()> {

        println!("Reading file: {}", file_path);

        let mut rdr = Reader::from_path(file_path)?;
        let mut values = vec![];
        let mut indexes = vec![];
        let mut counter = 0;
        let window_size = 110;
        let mut last_value = 0f64;

        for result in rdr.deserialize() {
            let record: Record = result?;
            values.push(Value::Float(record.close.clone()));
            indexes.push(counter);

            if(counter >= window_size) {
                let result = triangular_moving_average(&values, &indexes[counter - window_size..counter]);
                let x = result.unwrap().as_float().unwrap();
                println!("{:?} - tma = {:?} diff = {}", record, x, x - last_value);
                last_value = x;

            }else{
                println!("{:?}", record);

            }


            counter += 1;
        }

        Ok(())
    }
    #[test]
    fn moving_average_tests() -> anyhow::Result<()> {
        let mut pathname = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        pathname.push("tests");
        pathname.push("close_values.csv");

        read_csv(pathname.to_str().context("Unable to convert pathname to str")?)
    }
}
