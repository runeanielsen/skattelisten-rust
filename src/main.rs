#![warn(clippy::all, clippy::pedantic)]

use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct Company {
    csv: String,
    name: String,
    se: String,
    income_year: String,
    company_type: String,
    taxable_income: u64,
    deficit: u64,
    corporate_tax: u64,
}

fn create_company_from_csv_line(csv_line: &str) -> Company {
    let mut row_columns = csv_line.split(',');

    let csv = row_columns.next().unwrap();
    let name = row_columns.next().unwrap();
    let se = row_columns.next().unwrap();
    let income_year = row_columns.next().unwrap();
    _ = row_columns.next(); // Skip 5
    let company_type = row_columns.next().unwrap();
    _ = row_columns.next(); // Skip 7
    _ = row_columns.next(); // Skip 8
    let taxable_income: u64 = row_columns.next().unwrap().parse().unwrap_or(0);
    let deficit = row_columns.next().unwrap().parse().unwrap_or(0);
    let corporate_tax = row_columns.next().unwrap().parse().unwrap_or(0);

    Company {
        csv: String::from(csv),
        name: String::from(name),
        se: String::from(se),
        income_year: String::from(income_year),
        company_type: String::from(company_type),
        taxable_income,
        deficit,
        corporate_tax,
    }
}

fn main() {
    let input_file_name = "";
    let output_file_name = "";

    let input_file = fs::File::open(input_file_name).unwrap();
    let output_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_file_name)
        .unwrap();

    let mut reader = io::BufReader::with_capacity(4096, input_file);
    let mut writer = io::LineWriter::with_capacity(4096, output_file);

    let mut line = String::new();
    loop {
        match io::BufRead::read_line(&mut reader, &mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }

                writer
                    .write_all(
                        serde_json::to_string(&create_company_from_csv_line(&line))
                            .unwrap()
                            .as_bytes(),
                    )
                    .expect("Could not write all.");

                line.clear();
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    writer.flush().unwrap();
}
