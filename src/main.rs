use polars::prelude::*;

fn main() {
    // read csv data into a polars dataframe 
    let df = CsvReader::from_path("/Users/dylanhennessy/projects/polars-hentesting/appleStock.csv").unwrap().finish().unwrap();
    println!("{}",df);



}
