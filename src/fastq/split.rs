
use std::io;
use std::error::Error;
use bio::io::fastq;

pub fn split(out_paths: &[&str]) -> Result<(), Box<Error>> {
    let mut reader = fastq::Reader::new(io::stdin());
    let mut writers = Vec::new();
    for path in out_paths {
        writers.push(try!(fastq::Writer::to_file(path)));
    }
    let mut record = fastq::Record::new();
    let mut i = 0;
    let mut j = 0;
    loop {
        try!(reader.read(&mut record));
        if record.is_empty() {
            return Ok(());
        }
        try!(writers[i].write_record(&record));
        i = (i + 1) % writers.len();
        j += 1;
        if j % 1000 == 0 {
            info!("{} records written.", j);
        }
    }
}
