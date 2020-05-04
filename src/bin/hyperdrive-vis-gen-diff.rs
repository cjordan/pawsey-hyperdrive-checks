// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/*! This executable simply compares each of the present "hyperdrive_bandxx.bin"
    files against those in the "baseline" directory. Reports whether the largest
    difference between any two floats is larger than 0.001. Will fall over if
    the directory doesn't exist, or the corresponding file inside the directory
    doesn't exist.
*/

use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::{Path, PathBuf};

use anyhow::bail;
use byteorder::{ByteOrder, LittleEndian};
use glob::glob;

fn glob_files(path: &str) -> Vec<PathBuf> {
    glob(path)
        .unwrap()
        .map(|p| {
            let pb = PathBuf::from(p.unwrap());
            let file_name = pb.file_name().unwrap();
            PathBuf::from(file_name)
        })
        .collect()
}

fn read_f32s(path: &Path) -> Result<Vec<f32>, Error> {
    let mut file = File::open(path)?;
    let mut bytes = vec![];
    file.read_to_end(&mut bytes)?;

    let mut data = vec![0.0; bytes.len() / 4];
    LittleEndian::read_f32_into(&bytes, &mut data);
    Ok(data)
}

fn main() -> Result<(), anyhow::Error> {
    if !PathBuf::from("./baseline").is_dir() {
        bail!("Directory \"baseline\" is not present in the PWD! This should contain baseline hyperdrive binary files.")
    };

    let present_files = glob_files("hyperdrive_band??.bin");

    // Check that all present files are in baseline_files.
    {
        let baseline_files = glob_files("baseline/hyperdrive_band??.bin");
        for p in &present_files {
            if !baseline_files.contains(&p) {
                bail!("{:?} is missing from the \"baseline\" directory!", p);
            }
        }
    }

    // Now check the differences between the floats.
    let mut max_diff = 0.0;
    for p in present_files {
        println!("Checking {:?} ...", p);

        // Read in the present and baseline data.
        let p_data = read_f32s(&p)?;
        if p_data.is_empty() {
            eprintln!("WARNING: {:?} didn't contain any data", p);
            continue;
        }

        let mut b_file_path = PathBuf::from("baseline");
        b_file_path.push(&p);
        let b_data = read_f32s(&b_file_path)?;
        if b_data.is_empty() {
            eprintln!("WARNING: {:?} didn't contain any data", b_file_path);
            continue;
        }

        // Check that they have an equal amount of data.
        if p_data.len() != b_data.len() {
            eprintln!(
                "WARNING: {:?} and {:?} have different amounts of data",
                p, b_file_path
            );
            continue;
        }

        let biggest_diff = p_data
            .into_iter()
            .zip(b_data.into_iter())
            .fold(0.0, |acc, (p, d)| {
                let diff = (p - d).abs();
                if diff > acc {
                    diff
                } else {
                    acc
                }
            });
        println!("Biggest difference for {:?}: {}", p, biggest_diff);

        if biggest_diff > max_diff {
            max_diff = biggest_diff;
        }
    }

    println!("Maximum difference: {}", max_diff);

    if max_diff > 0.001 {
        println!("Difference is too large; exiting with code -1.");
        std::process::exit(-1);
    }

    Ok(())
}
