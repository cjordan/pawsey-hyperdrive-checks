// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/*! This executable simply compares each of the "hyperdrive_bandxx.bin" files in
    the present working directory against those in the "baseline"
    directory. Reports whether the largest difference between any two floats is
    larger than some tolerance. Will fall over if the baseline directory doesn't
    exist, or if there is some kind of mis-match between the hyperdrive files.
*/

use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use anyhow::bail;
use byteorder::{ByteOrder, LittleEndian};
use glob::glob;
use structopt::StructOpt;

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

fn read_f32s(path: &Path) -> Result<Vec<f32>, anyhow::Error> {
    let mut file = File::open(path)?;
    let mut bytes = vec![];
    file.read_to_end(&mut bytes)?;

    let mut data = vec![0.0; bytes.len() / 4];
    if 4 * data.len() != bytes.len() {
        bail!(
            "An invalid number of bytes were read from {:?}. Does this file contain really floats?",
            path
        );
    }
    LittleEndian::read_f32_into(&bytes, &mut data);
    Ok(data)
}

/// This executable simply compares each of the "hyperdrive_bandxx.bin" files in
/// the present working directory against those in the "baseline"
/// directory. Reports whether the largest difference between any two floats is
/// larger than some tolerance. Will fall over if the baseline directory doesn't
/// exist, or if there is some kind of mis-match between the hyperdrive files.
#[derive(StructOpt, Debug)]
#[structopt(author)]
struct Opt {
    /// The directory containing hyperdrive simulate-vis outputs to compare
    /// against.
    #[structopt(
        name = "BASELINE_DIR",
        default_value = "./baseline",
        parse(from_os_str)
    )]
    baseline_dir: PathBuf,

    /// If the maximum difference between any two files is bigger than this
    /// number, then fail.
    #[structopt(short, long, default_value = "0.001")]
    tolerance: f32,

    /// Do not print anything; the success or failure is determined only by the
    /// exit code.
    #[structopt(short, long)]
    quiet: bool,
}

fn main() -> Result<(), anyhow::Error> {
    let options = Opt::from_args();

    if !PathBuf::from(&options.baseline_dir).is_dir() {
        bail!(
            "Directory {:?} does not exist! This should contain baseline hyperdrive binary files.",
            options.baseline_dir
        )
    };

    let baseline_str = &options
        .baseline_dir
        .to_str()
        .expect("The baseline dir contained invalid unicode");
    let present_files = glob_files("hyperdrive_band??.bin");
    if present_files.is_empty() {
        bail!("PWD does not have any hyperdrive_band??.bin files!")
    }

    // Check that all present files are in baseline_files.
    {
        let baseline_files = glob_files(&format!("{}/hyperdrive_band??.bin", baseline_str));
        for p in &present_files {
            if !baseline_files.contains(&p) {
                bail!("{:?} is missing from {}!", p, baseline_str);
            }
        }
    }

    // Now check the differences between the floats.
    let mut max_diff = None;
    for p in present_files {
        if !options.quiet {
            println!("Checking {:?} ...", p);
        }

        // Read in the present and baseline data.
        let p_data = read_f32s(&p)?;
        if p_data.is_empty() {
            bail!("{:?} didn't contain any data", p);
        }

        let mut b_file_path = PathBuf::from(baseline_str);
        b_file_path.push(&p);
        let b_data = read_f32s(&b_file_path)?;
        if b_data.is_empty() {
            bail!("{:?} didn't contain any data", b_file_path);
        }

        // Check that they have an equal amount of data.
        if p_data.len() != b_data.len() {
            bail!(
                "bail: {:?} and {:?} have different amounts of data",
                p,
                b_file_path
            );
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
        if !options.quiet {
            println!("Biggest difference for {:?}: {}", p, biggest_diff);
        }

        max_diff = max_diff.map_or(Some(biggest_diff), |m| {
            if biggest_diff > m {
                Some(biggest_diff)
            } else {
                Some(m)
            }
        });
    }

    let max_diff = max_diff.expect("max_diff never got set!");

    if !options.quiet {
        println!("Maximum difference: {}", max_diff);
    }

    if max_diff > options.tolerance {
        if !options.quiet {
            println!("Difference is too large; exiting with code -1.");
        }
        std::process::exit(-1);
    }

    Ok(())
}
