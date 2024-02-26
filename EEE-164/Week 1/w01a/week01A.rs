use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);

    let mut output_file = File::create("output.txt")?;
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap()?.parse().unwrap();
    
    let mut series_total = 0.0;
    let mut parallel_total = 0.0;
    for i in 1..=n {
        if let Some(line) = lines.next() {
            let x: f64 = line.unwrap().parse().unwrap(); // parse each line as an integer
            series_total += x; 
            if i == 1 {
                parallel_total = x
            } else {
                parallel_total = (parallel_total*x)/(parallel_total+x);
            }
            // println!("test: {}\n", parallel_total);     
        }
    }

    writeln!(output_file, "{:.4}\n{:.4}", series_total, parallel_total)?;
    Ok(())
}
