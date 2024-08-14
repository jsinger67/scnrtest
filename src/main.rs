mod arguments;
use arguments::CliArgs;
use clap::Parser;
use std::{sync::LazyLock, time::Instant};

use scnr::{ScannerBuilder, ScannerMode};

static MODES: LazyLock<[ScannerMode; 1]> = LazyLock::new(|| {
    [ScannerMode::new(
        "INITIAL",
        vec![
            (r"\r\n|\r|\n", 1),                 // Newline
            (r"[\s--\r\n]+", 2),                // Whitespace
            (r"(//.*(\r\n|\r|\n))", 3),         // Line comment
            (r"/\*([.\r\n--*]|\*[^/])*\*/", 4), // Block comment
            (r"%start", 5),                     // Start
            (r"%title", 6),                     // Title
            (r"%comment", 7),                   // Comment
            (r"%user_type", 8),                 // User type
            (r"=", 9),                          // Assignment
            (r"%grammar_type", 10),             // Grammar type
            (r"%line_comment", 11),             // Line comment
            (r"%block_comment", 12),            // Block comment
            (r"%auto_newline_off", 13),         // Auto newline off
            (r"%auto_ws_off", 14),              // Auto whitespace off
            (r"%on", 15),                       // On
            (r"%enter", 16),                    // Enter
            (r"%%", 17),                        // %%
            (r"::", 18),                        // DoubleColon
            (r":", 19),                         // Colon
            (r";", 20),                         // Semicolon
            (r"\|", 21),                        // Or
            (r"<", 22),                         // LT
            (r">", 23),                         // GT
            (r#""(\\.|[^\\])*""#, 24),          // String
            (r"'(\\'|[^'])*'", 25),             // Raw String
            (r"/(\\.|[^\\])*/", 26),            // Regex
            (r"\(", 27),                        // LParen
            (r"\)", 28),                        // RParen
            (r"\[", 29),                        // LBracket
            (r"\]", 30),                        // RBracket
            (r"\{", 31),                        // LBrace
            (r"\}", 32),                        // RBrace
            (r"[a-zA-Z_][a-zA-Z0-9_]*", 33),    // Identifier
            (r"%scanner", 34),                  // Scanner
            (r",", 35),                         // Comma
            (r"%sc", 36),                       // SC
            (r"%push", 37),                     // Push
            (r"%pop", 38),                      // Pop
            (r"\^", 39),                        // Caret
            (".", 40),                          // Error
        ],
        vec![],
    )]
});

// static MODES: LazyLock<[ScannerMode; 2]> = LazyLock::new(|| {
//     [
//         ScannerMode::new(
//             "INITIAL",
//             vec![
//                 (r"\r\n|\r|\n", 0),             // Newline
//                 (r"[\s--\r\n]+", 1),            // Whitespace
//                 (r"(//.*(\\r\\n|\\r|\\n))", 2), // Line comment
//                 (r"(/\*[.\r\n]*?\*/)", 3),      // Block comment
//                 (r"[a-zA-Z_]\w*", 4),           // Identifier
//                 (r"\u{22}", 8),                 // String delimiter
//                 (r".", 9),                      // Error
//             ],
//             vec![
//                 (8, 1), // Token "String delimiter" -> Mode "STRING"
//             ],
//         ),
//         ScannerMode::new(
//             "STRING",
//             vec![
//                 (r"\r\n|\r|\n", 0),               // Newline
//                 (r"[\s--\r\n]+", 1),              // Whitespace
//                 (r"(//.*(\\r\\n|\\r|\\n))", 2),   // Line comment
//                 (r"(/\*[.\r\n]*?\*/)", 3),        // Block comment
//                 (r"\u{5c}[\u{22}\u{5c}bfnt]", 5), // Escape sequence
//                 (r"\u{5c}[\s^\n\r]*\r?\n", 6),    // Line continuation
//                 (r"[^\u{22}\u{5c}]+", 7),         // String content
//                 (r"\u{22}", 8),                   // String delimiter
//                 (r".", 9),                        // Error
//             ],
//             vec![
//                 (8, 0), // Token "String delimiter" -> Mode "INITIAL"
//             ],
//         ),
//     ]
// });

fn main() {
    env_logger::init();

    let args = CliArgs::parse();
    let input = std::fs::read_to_string(args.parfile.clone()).unwrap();

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
            for ma in find_iter {
                count += 1;
                if !args.quiet {
                    println!("Match: {:?}: '{}'", ma, &input[ma.start()..ma.end()]);
                }
            }
            let elapsed_scangen = start.elapsed();
            println!("Find all {} tokens took {:?}", count, elapsed_scangen);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
