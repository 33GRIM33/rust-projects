// PHASE 1: READ DATA
// TODo: read housing.csv file
// TODo: create two empty Vec<f64>: x_values and y_values
// TODo: loop through lines (skip header line)
// TODo: split each line by comma
// TODo: parse first part to f64, push to x_values
// TODo: parse second part to f64, push to y_values
// CHECKPOINT: print both vectors to verify

//usign serde to read files
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Record {
    x1: f64,
    x2: f64,
    y: f64,
}

use std::fs;

// fn read_from_csv_file(path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
//     let mut rdr = csv::Reader::from_path(path)?;
//     let mut records: Vec<Record> = Vec::new();
//     for result in rdr.deserialize::<Record>() {
//         let record = result?;
//         records.push(record);
//     }
//     Ok(records)
// }

fn mean(data: &Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;
    for i in 0..data.len() {
        sum += data[i];
    }
    sum / data.len() as f64
}

// PHASE 3: LINEAR REGRESSION
// TODo: write linear_regression() function that takes two &Vec<f64>
// TODo: inside function, calculate x_mean and y_mean
// TODo: loop through data, calculate numerator = sum of (x-x_mean)*(y-y_mean)
// TODo: loop through data, calculate denominator = sum of (x-x_mean)^2
// TODo: slope = numerator / denominator
// TODo: intercept = y_mean - slope * x_mean
// TODo: return (slope, intercept)
// CHECKPOINT: print slope and intercept

fn linear_regression(x: &Vec<f64>, y: &Vec<f64>) -> (f64, f64) {
    let x_mean = mean(x);
    let y_mean = mean(y);
    let mut numerator = 0.0;
    let mut denominator = 0.0;
    for i in 0..x.len() {
        numerator += (x[i] - x_mean) * (y[i] - y_mean);
        denominator += (x[i] - x_mean).powf(2.0);
    }
    let slope = numerator / denominator;
    let intercept = y_mean - slope * x_mean;
    (slope, intercept)
}

fn main() {
    // PHASE 1: READ DATA
    //u can simple iterate over record because this coudl also eb an erroo thats why rust will give erpro if u try to run the belo code
    // for r in record{
    //     println!("{:?}", r);
    // }
    let mut x1: Vec<f64> = Vec::new();
    let mut x2: Vec<f64> = Vec::new();
    let mut y1: Vec<f64> = Vec::new();

    // TODO: loop through contents.lines()
    // TODO: skip first line (header)
    // TODO: split by comma
    // TODO: parse and push to vectors
    let c = std::fs::read_to_string("temp_data.csv").unwrap();

    for line in c.lines().skip(1){
        let record: Vec<&str> = line.split(',').collect();
        let x11 = record[0].parse::<f64>().unwrap();
        let x22 = record[1].parse::<f64>().unwrap();
        let y = record[2].parse::<f64>().unwrap();
        // println!("record : {:?}", record);
        // println!("x : {:?}", x);
        x1.push(x11);
        x2.push(x22);
        y1.push(y);
    }


    // for r in record.unwrap() {
    //     x1.push(r.x1);
    //     x2.push(r.x2);
    //     y.push(r.y);
    // }
    // for i in 0..x1.len() {
    //     println!("{:?}", x1[i]);
    // }
    // for i in 0..x2.len() {
    //     println!("{:?}", x2[i]);
    // }
    // for i in 0..y.len() {
    //     println!("{:?}", y[i]);
    // }

    // PHASE 2: CALCULATE MEANS
    // TODo: write mean() function that takes &Vec<f64> returns f64
    // TODo: calculate x_mean using mean function
    // TODo: calculate y_mean using mean function
    // CHECKPOINT: print means (should be x=2000, y=300000)

    //Calculating means
    let x1_mean = mean(&x1);
    let x2_mean = mean(&x2);
    let y_mean = mean(&y1);
    println!("{} {} {} ", x1_mean, x2_mean, y_mean);

    // PHASE 4: OUTPUT
    // TODo: print equation in format "y = {slope}x + {intercept}"

    let (slope,intercept) = linear_regression(&x1,&y1);
    println!("y = {}x + {}",slope,intercept)
}
