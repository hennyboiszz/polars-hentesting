use polars::prelude::*;


fn main() {
    // read csv data into a polars dataframe and output it to the console
    let df = CsvReader::from_path("./appleStock.csv").unwrap().finish().unwrap();
    println!("{}",df);

    //let df = CsvReader::from_path("./appleStock.csv").unwrap().finish().unwrap();




}
