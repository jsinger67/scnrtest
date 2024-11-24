mod arguments;
use arguments::CliArgs;
use clap::Parser;
use std::{fs, time::Instant};

use scnr::{ScannerBuilder, ScannerMode};

fn main() {
    let args = CliArgs::parse();
    let input = args.input.as_ref().map(|f| {
        std::fs::read_to_string(f)
            .unwrap_or_else(|_| panic!("Could not read the input file {}.", f.display()))
    });

    if args.trace {
        env_logger::init_from_env(
            env_logger::Env::default().default_filter_or("scnr::internal::scanner_impl=trace"),
        );
    } else {
        env_logger::init();
    }

    let scanner = match args.modes {
        Some(modes) => {
            let modes = std::fs::read_to_string(modes).unwrap();
            let modes: Vec<ScannerMode> = serde_json::from_str(&modes).unwrap();
            let start = Instant::now();
            let scanner = ScannerBuilder::new()
                .add_scanner_modes(&modes)
                .build()
                .unwrap();
            println!("Building the scanner took {:?}", start.elapsed());
            if args.trace {
                scanner.log_compiled_automata_as_dot(&modes).unwrap();
            }
            if let Some(dot) = args.dot {
                // Delete all previously generated dot files.
                let _ = fs::remove_dir_all(&dot);
                // Create the target folder.
                fs::create_dir_all(&dot).unwrap();

                scanner
                    .generate_compiled_automata_as_dot(&modes, &dot)
                    .unwrap();
            }
            scanner
        }
        None => match args.patterns {
            Some(patterns) => {
                let start = Instant::now();
                let scanner = ScannerBuilder::new()
                    .add_patterns(&patterns)
                    .build()
                    .unwrap();
                println!(
                    "Building the scanner from patterns took {:?}",
                    start.elapsed()
                );
                scanner
            }
            None => {
                panic!("No modes or patterns provided.");
            }
        },
    };

    let input = args.text.or(input);
    if let Some(input) = input {
        let start = Instant::now();
        let find_iter = scanner.find_iter(&input);
        println!(
            "Creating the FindMatches iterator took {:?}",
            start.elapsed()
        );
        let start = Instant::now();
        let mut count = 0;
        for ma in find_iter {
            count += 1;
            if !args.quiet {
                println!("Match: {:?}: '{}'", ma, &input[ma.start()..ma.end()]);
            }
        }
        let elapsed_scangen = start.elapsed();
        println!("Find all {} tokens took {:?}", count, elapsed_scangen);
    }
}
