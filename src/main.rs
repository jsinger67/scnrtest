use std::{env::args, sync::LazyLock, time::Instant};

use scnr::{ScannerBuilder, ScannerMode};

static MODES: LazyLock<[ScannerMode; 2]> = LazyLock::new(|| {
    [
        ScannerMode::new(
            "INITIAL",
            vec![
                (r"\r\n|\r|\n", 0),             // Newline
                (r"[\s--\r\n]+", 1),            // Whitespace
                (r"(//.*(\\r\\n|\\r|\\n))", 2), // Line comment
                (r"(/\*[.\r\n]*?\*/)", 3),      // Block comment
                (r"[a-zA-Z_]\w*", 4),           // Identifier
                (r"\u{22}", 8),                 // String delimiter
                (r".", 9),                      // Error
            ],
            vec![
                (8, 1), // Token "String delimiter" -> Mode "STRING"
            ],
        ),
        ScannerMode::new(
            "STRING",
            vec![
                (r"\r\n|\r|\n", 0),               // Newline
                (r"[\s--\r\n]+", 1),              // Whitespace
                (r"(//.*(\\r\\n|\\r|\\n))", 2),   // Line comment
                (r"(/\*[.\r\n]*?\*/)", 3),        // Block comment
                (r"\u{5c}[\u{22}\u{5c}bfnt]", 5), // Escape sequence
                (r"\u{5c}[\s^\n\r]*\r?\n", 6),    // Line continuation
                (r"[^\u{22}\u{5c}]+", 7),         // String content
                (r"\u{22}", 8),                   // String delimiter
                (r".", 9),                        // Error
            ],
            vec![
                (8, 0), // Token "String delimiter" -> Mode "INITIAL"
            ],
        ),
    ]
});
fn main() {
    env_logger::init();

    if args().len() != 2 {
        eprintln!("Usage: {} filename", args().next().unwrap());
        std::process::exit(1);
    }
    let file_name = args().nth(1).unwrap();

    let input = std::fs::read_to_string(file_name.clone()).unwrap();

    let start = Instant::now();
    let scanner = ScannerBuilder::new()
        .add_scanner_modes(&*MODES)
        .build()
        .unwrap();
    println!("Building the scanner took {:?}", start.elapsed());

    let start = Instant::now();
    let find_iter = scanner.find_iter(&input);
    println!(
        "Creating the FindMatches iterator took {:?}",
        start.elapsed()
    );
    match find_iter {
        Ok(find_iter) => {
            let start = Instant::now();
            let mut count = 0;
            for _ma in find_iter {
                count += 1;
                // println!("Match: {:?}: '{}'", ma, &input[ma.start()..ma.end()]);
            }
            let elapsed_scangen = start.elapsed();
            println!("Find all {} tokens took {:?}", count, elapsed_scangen);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
