use parol_runtime::{
    lexer::tokenizer::{ERROR_TOKEN, NEW_LINE_TOKEN, UNMATCHABLE_TOKEN, WHITESPACE_TOKEN},
    once_cell::sync::Lazy,
    ScannerConfig, TerminalIndex, Tokenizer,
};

pub const TERMINALS: &[&str; 119] = &[
    /*   0 */ UNMATCHABLE_TOKEN,
    /*   1 */ UNMATCHABLE_TOKEN,
    /*   2 */ UNMATCHABLE_TOKEN,
    /*   3 */ UNMATCHABLE_TOKEN,
    /*   4 */ UNMATCHABLE_TOKEN,
    /*   5 */ r"(?:(?:(?://.*(?:\r\n|\r|\n|$))|(?:(?ms)/\u{2a}.*?\u{2a}/))\s*)+",
    /*   6 */
    r"\u{0022}(?:\\[\u{0022}\\/bfnrt]|u[0-9a-fA-F]{4}|[^\u{0022}\\\u0000-\u001F])*\u{0022}",
    /*   7 */ r"[0-9]+(?:_[0-9]+)*\.[0-9]+(?:_[0-9]+)*[eE][+-]?[0-9]+(?:_[0-9]+)*",
    /*   8 */ r"[0-9]+(?:_[0-9]+)*\.[0-9]+(?:_[0-9]+)*",
    /*   9 */ r"(?:[0-9]+(?:_[0-9]+)*)?'s?[bodh][0-9a-fA-FxzXZ]+(?:_[0-9a-fA-FxzXZ]+)*",
    /*  10 */ r"(?:[0-9]+(?:_[0-9]+)*)?'[01xzXZ]",
    /*  11 */ r"[0-9]+(?:_[0-9]+)*",
    /*  12 */ r"\-:",
    /*  13 */ r"\->",
    /*  14 */ r"\+:",
    /*  15 */ r"\+=|-=|\*=|/=|%=|&=|\|=|\^=|<<=|>>=|<<<=|>>>=",
    /*  16 */ r"\*\*",
    /*  17 */ r"/|%",
    /*  18 */ r"\+|-",
    /*  19 */ r"<<<|>>>|<<|>>",
    /*  20 */ r"<=|>=|<:|>:",
    /*  21 */ r"===|==\?|!==|!=\?|==|!=",
    /*  22 */ r"&&",
    /*  23 */ r"\|\|",
    /*  24 */ r"&",
    /*  25 */ r"\^~|\^|~\^",
    /*  26 */ r"\|",
    /*  27 */ r"~&|~\||!|~",
    /*  28 */ r"`",
    /*  29 */ r"::<",
    /*  30 */ r"::",
    /*  31 */ r":",
    /*  32 */ r",",
    /*  33 */ r"\.\.=",
    /*  34 */ r"\.\.",
    /*  35 */ r"\.",
    /*  36 */ r"=",
    /*  37 */ r"\#",
    /*  38 */ r"<",
    /*  39 */ r"'\{",
    /*  40 */ r"\{",
    /*  41 */ r"\[",
    /*  42 */ r"\(",
    /*  43 */ r">",
    /*  44 */ r"\}",
    /*  45 */ r"\]",
    /*  46 */ r"\)",
    /*  47 */ r";",
    /*  48 */ r"\*",
    /*  49 */ r"(?-u:\b)always_comb(?-u:\b)",
    /*  50 */ r"(?-u:\b)always_ff(?-u:\b)",
    /*  51 */ r"(?-u:\b)assign(?-u:\b)",
    /*  52 */ r"(?-u:\b)as(?-u:\b)",
    /*  53 */ r"(?-u:\b)bit(?-u:\b)",
    /*  54 */ r"(?-u:\b)case(?-u:\b)",
    /*  55 */ r"(?-u:\b)clock(?-u:\b)",
    /*  56 */ r"(?-u:\b)clock_posedge(?-u:\b)",
    /*  57 */ r"(?-u:\b)clock_negedge(?-u:\b)",
    /*  58 */ r"(?-u:\b)const(?-u:\b)",
    /*  59 */ r"(?-u:\b)default(?-u:\b)",
    /*  60 */ r"(?-u:\b)else(?-u:\b)",
    /*  61 */ r"(?-u:\b)embed(?-u:\b)",
    /*  62 */ r"(?-u:\b)enum(?-u:\b)",
    /*  63 */ r"(?-u:\b)export(?-u:\b)",
    /*  64 */ r"(?-u:\b)f32(?-u:\b)",
    /*  65 */ r"(?-u:\b)f64(?-u:\b)",
    /*  66 */ r"(?-u:\b)final(?-u:\b)",
    /*  67 */ r"(?-u:\b)for(?-u:\b)",
    /*  68 */ r"(?-u:\b)function(?-u:\b)",
    /*  69 */ r"(?-u:\b)i32(?-u:\b)",
    /*  70 */ r"(?-u:\b)i64(?-u:\b)",
    /*  71 */ r"(?-u:\b)if_reset(?-u:\b)",
    /*  72 */ r"(?-u:\b)if(?-u:\b)",
    /*  73 */ r"(?-u:\b)import(?-u:\b)",
    /*  74 */ r"(?-u:\b)include(?-u:\b)",
    /*  75 */ r"(?-u:\b)initial(?-u:\b)",
    /*  76 */ r"(?-u:\b)inout(?-u:\b)",
    /*  77 */ r"(?-u:\b)input(?-u:\b)",
    /*  78 */ r"(?-u:\b)inside(?-u:\b)",
    /*  79 */ r"(?-u:\b)inst(?-u:\b)",
    /*  80 */ r"(?-u:\b)interface(?-u:\b)",
    /*  81 */ r"(?-u:\b)in(?-u:\b)",
    /*  82 */ r"(?-u:\b)let(?-u:\b)",
    /*  83 */ r"(?-u:\b)logic(?-u:\b)",
    /*  84 */ r"(?-u:\b)lsb(?-u:\b)",
    /*  85 */ r"(?-u:\b)modport(?-u:\b)",
    /*  86 */ r"(?-u:\b)module(?-u:\b)",
    /*  87 */ r"(?-u:\b)msb(?-u:\b)",
    /*  88 */ r"(?-u:\b)output(?-u:\b)",
    /*  89 */ r"(?-u:\b)outside(?-u:\b)",
    /*  90 */ r"(?-u:\b)package(?-u:\b)",
    /*  91 */ r"(?-u:\b)param(?-u:\b)",
    /*  92 */ r"(?-u:\b)proto(?-u:\b)",
    /*  93 */ r"(?-u:\b)pub(?-u:\b)",
    /*  94 */ r"(?-u:\b)ref(?-u:\b)",
    /*  95 */ r"(?-u:\b)repeat(?-u:\b)",
    /*  96 */ r"(?-u:\b)reset(?-u:\b)",
    /*  97 */ r"(?-u:\b)reset_async_high(?-u:\b)",
    /*  98 */ r"(?-u:\b)reset_async_low(?-u:\b)",
    /*  99 */ r"(?-u:\b)reset_sync_high(?-u:\b)",
    /* 100 */ r"(?-u:\b)reset_sync_low(?-u:\b)",
    /* 101 */ r"(?-u:\b)return(?-u:\b)",
    /* 102 */ r"(?-u:\b)break(?-u:\b)",
    /* 103 */ r"(?-u:\b)signed(?-u:\b)",
    /* 104 */ r"(?-u:\b)step(?-u:\b)",
    /* 105 */ r"(?-u:\b)string(?-u:\b)",
    /* 106 */ r"(?-u:\b)struct(?-u:\b)",
    /* 107 */ r"(?-u:\b)switch(?-u:\b)",
    /* 108 */ r"(?-u:\b)tri(?-u:\b)",
    /* 109 */ r"(?-u:\b)type(?-u:\b)",
    /* 110 */ r"(?-u:\b)u32(?-u:\b)",
    /* 111 */ r"(?-u:\b)u64(?-u:\b)",
    /* 112 */ r"(?-u:\b)union(?-u:\b)",
    /* 113 */ r"(?-u:\b)unsafe(?-u:\b)",
    /* 114 */ r"(?-u:\b)var(?-u:\b)",
    /* 115 */ r"\$[a-zA-Z_][0-9a-zA-Z_$]*",
    /* 116 */ r"(?:r#)?[a-zA-Z_][0-9a-zA-Z_$]*",
    /* 117 */ r"[^{}]*",
    /* 118 */ ERROR_TOKEN,
];

#[allow(unused)]
pub const TERMINAL_NAMES: &[&str; 119] = &[
    /*   0 */ "EndOfInput",
    /*   1 */ "Newline",
    /*   2 */ "Whitespace",
    /*   3 */ "LineComment",
    /*   4 */ "BlockComment",
    /*   5 */ "CommentsTerm",
    /*   6 */ "StringLiteralTerm",
    /*   7 */ "ExponentTerm",
    /*   8 */ "FixedPointTerm",
    /*   9 */ "BasedTerm",
    /*  10 */ "AllBitTerm",
    /*  11 */ "BaseLessTerm",
    /*  12 */ "MinusColonTerm",
    /*  13 */ "MinusGTTerm",
    /*  14 */ "PlusColonTerm",
    /*  15 */ "AssignmentOperatorTerm",
    /*  16 */ "Operator11Term",
    /*  17 */ "Operator10Term",
    /*  18 */ "Operator09Term",
    /*  19 */ "Operator08Term",
    /*  20 */ "Operator07Term",
    /*  21 */ "Operator06Term",
    /*  22 */ "Operator02Term",
    /*  23 */ "Operator01Term",
    /*  24 */ "Operator05Term",
    /*  25 */ "Operator04Term",
    /*  26 */ "Operator03Term",
    /*  27 */ "UnaryOperatorTerm",
    /*  28 */ "BackQuoteTerm",
    /*  29 */ "ColonColonLAngleTerm",
    /*  30 */ "ColonColonTerm",
    /*  31 */ "ColonTerm",
    /*  32 */ "CommaTerm",
    /*  33 */ "DotDotEquTerm",
    /*  34 */ "DotDotTerm",
    /*  35 */ "DotTerm",
    /*  36 */ "EquTerm",
    /*  37 */ "HashTerm",
    /*  38 */ "LAngleTerm",
    /*  39 */ "QuoteLBraceTerm",
    /*  40 */ "LBraceTerm",
    /*  41 */ "LBracketTerm",
    /*  42 */ "LParenTerm",
    /*  43 */ "RAngleTerm",
    /*  44 */ "RBraceTerm",
    /*  45 */ "RBracketTerm",
    /*  46 */ "RParenTerm",
    /*  47 */ "SemicolonTerm",
    /*  48 */ "StarTerm",
    /*  49 */ "AlwaysCombTerm",
    /*  50 */ "AlwaysFfTerm",
    /*  51 */ "AssignTerm",
    /*  52 */ "AsTerm",
    /*  53 */ "BitTerm",
    /*  54 */ "CaseTerm",
    /*  55 */ "ClockTerm",
    /*  56 */ "ClockPosedgeTerm",
    /*  57 */ "ClockNegedgeTerm",
    /*  58 */ "ConstTerm",
    /*  59 */ "DefaultTerm",
    /*  60 */ "ElseTerm",
    /*  61 */ "EmbedTerm",
    /*  62 */ "EnumTerm",
    /*  63 */ "ExportTerm",
    /*  64 */ "F32Term",
    /*  65 */ "F64Term",
    /*  66 */ "FinalTerm",
    /*  67 */ "ForTerm",
    /*  68 */ "FunctionTerm",
    /*  69 */ "I32Term",
    /*  70 */ "I64Term",
    /*  71 */ "IfResetTerm",
    /*  72 */ "IfTerm",
    /*  73 */ "ImportTerm",
    /*  74 */ "IncludeTerm",
    /*  75 */ "InitialTerm",
    /*  76 */ "InoutTerm",
    /*  77 */ "InputTerm",
    /*  78 */ "InsideTerm",
    /*  79 */ "InstTerm",
    /*  80 */ "InterfaceTerm",
    /*  81 */ "InTerm",
    /*  82 */ "LetTerm",
    /*  83 */ "LogicTerm",
    /*  84 */ "LsbTerm",
    /*  85 */ "ModportTerm",
    /*  86 */ "ModuleTerm",
    /*  87 */ "MsbTerm",
    /*  88 */ "OutputTerm",
    /*  89 */ "OutsideTerm",
    /*  90 */ "PackageTerm",
    /*  91 */ "ParamTerm",
    /*  92 */ "ProtoTerm",
    /*  93 */ "PubTerm",
    /*  94 */ "RefTerm",
    /*  95 */ "RepeatTerm",
    /*  96 */ "ResetTerm",
    /*  97 */ "ResetAsyncHighTerm",
    /*  98 */ "ResetAsyncLowTerm",
    /*  99 */ "ResetSyncHighTerm",
    /* 100 */ "ResetSyncLowTerm",
    /* 101 */ "ReturnTerm",
    /* 102 */ "BreakTerm",
    /* 103 */ "SignedTerm",
    /* 104 */ "StepTerm",
    /* 105 */ "StringTerm",
    /* 106 */ "StructTerm",
    /* 107 */ "SwitchTerm",
    /* 108 */ "TriTerm",
    /* 109 */ "TypeTerm",
    /* 110 */ "U32Term",
    /* 111 */ "U64Term",
    /* 112 */ "UnionTerm",
    /* 113 */ "UnsafeTerm",
    /* 114 */ "VarTerm",
    /* 115 */ "DollarIdentifierTerm",
    /* 116 */ "IdentifierTerm",
    /* 117 */ "AnyTerm",
    /* 118 */ "Error",
];

/* SCANNER_0: "INITIAL" */
pub const SCANNER_0: (&[&str; 5], &[TerminalIndex; 112]) = (
    &[
        /*   0 */ UNMATCHABLE_TOKEN,
        /*   1 */ NEW_LINE_TOKEN,
        /*   2 */ WHITESPACE_TOKEN,
        /*   3 */ UNMATCHABLE_TOKEN,
        /*   4 */ UNMATCHABLE_TOKEN,
    ],
    &[
        5,   /* CommentsTerm */
        6,   /* StringLiteralTerm */
        7,   /* ExponentTerm */
        8,   /* FixedPointTerm */
        9,   /* BasedTerm */
        10,  /* AllBitTerm */
        11,  /* BaseLessTerm */
        12,  /* MinusColonTerm */
        13,  /* MinusGTTerm */
        14,  /* PlusColonTerm */
        15,  /* AssignmentOperatorTerm */
        16,  /* Operator11Term */
        17,  /* Operator10Term */
        18,  /* Operator09Term */
        19,  /* Operator08Term */
        20,  /* Operator07Term */
        21,  /* Operator06Term */
        22,  /* Operator02Term */
        23,  /* Operator01Term */
        24,  /* Operator05Term */
        25,  /* Operator04Term */
        26,  /* Operator03Term */
        27,  /* UnaryOperatorTerm */
        28,  /* BackQuoteTerm */
        29,  /* ColonColonLAngleTerm */
        30,  /* ColonColonTerm */
        31,  /* ColonTerm */
        32,  /* CommaTerm */
        33,  /* DotDotEquTerm */
        34,  /* DotDotTerm */
        35,  /* DotTerm */
        36,  /* EquTerm */
        37,  /* HashTerm */
        38,  /* LAngleTerm */
        39,  /* QuoteLBraceTerm */
        40,  /* LBraceTerm */
        41,  /* LBracketTerm */
        42,  /* LParenTerm */
        43,  /* RAngleTerm */
        44,  /* RBraceTerm */
        45,  /* RBracketTerm */
        46,  /* RParenTerm */
        47,  /* SemicolonTerm */
        48,  /* StarTerm */
        49,  /* AlwaysCombTerm */
        50,  /* AlwaysFfTerm */
        51,  /* AssignTerm */
        52,  /* AsTerm */
        53,  /* BitTerm */
        54,  /* CaseTerm */
        55,  /* ClockTerm */
        56,  /* ClockPosedgeTerm */
        57,  /* ClockNegedgeTerm */
        58,  /* ConstTerm */
        59,  /* DefaultTerm */
        60,  /* ElseTerm */
        61,  /* EmbedTerm */
        62,  /* EnumTerm */
        63,  /* ExportTerm */
        64,  /* F32Term */
        65,  /* F64Term */
        66,  /* FinalTerm */
        67,  /* ForTerm */
        68,  /* FunctionTerm */
        69,  /* I32Term */
        70,  /* I64Term */
        71,  /* IfResetTerm */
        72,  /* IfTerm */
        73,  /* ImportTerm */
        74,  /* IncludeTerm */
        75,  /* InitialTerm */
        76,  /* InoutTerm */
        77,  /* InputTerm */
        78,  /* InsideTerm */
        79,  /* InstTerm */
        80,  /* InterfaceTerm */
        81,  /* InTerm */
        82,  /* LetTerm */
        83,  /* LogicTerm */
        84,  /* LsbTerm */
        85,  /* ModportTerm */
        86,  /* ModuleTerm */
        87,  /* MsbTerm */
        88,  /* OutputTerm */
        89,  /* OutsideTerm */
        90,  /* PackageTerm */
        91,  /* ParamTerm */
        92,  /* ProtoTerm */
        93,  /* PubTerm */
        94,  /* RefTerm */
        95,  /* RepeatTerm */
        96,  /* ResetTerm */
        97,  /* ResetAsyncHighTerm */
        98,  /* ResetAsyncLowTerm */
        99,  /* ResetSyncHighTerm */
        100, /* ResetSyncLowTerm */
        101, /* ReturnTerm */
        102, /* BreakTerm */
        103, /* SignedTerm */
        104, /* StepTerm */
        105, /* StringTerm */
        106, /* StructTerm */
        107, /* SwitchTerm */
        108, /* TriTerm */
        109, /* TypeTerm */
        110, /* U32Term */
        111, /* U64Term */
        112, /* UnionTerm */
        113, /* UnsafeTerm */
        114, /* VarTerm */
        115, /* DollarIdentifierTerm */
        116, /* IdentifierTerm */
    ],
);

/* SCANNER_1: "Embed" */
pub const SCANNER_1: (&[&str; 5], &[TerminalIndex; 3]) = (
    &[
        /*   0 */ UNMATCHABLE_TOKEN,
        /*   1 */ UNMATCHABLE_TOKEN,
        /*   2 */ UNMATCHABLE_TOKEN,
        /*   3 */ UNMATCHABLE_TOKEN,
        /*   4 */ UNMATCHABLE_TOKEN,
    ],
    &[
        40,  /* LBraceTerm */
        44,  /* RBraceTerm */
        117, /* AnyTerm */
    ],
);

/* SCANNER_2: "Generic" */
pub const SCANNER_2: (&[&str; 5], &[TerminalIndex; 96]) = (
    &[
        /*   0 */ UNMATCHABLE_TOKEN,
        /*   1 */ NEW_LINE_TOKEN,
        /*   2 */ WHITESPACE_TOKEN,
        /*   3 */ UNMATCHABLE_TOKEN,
        /*   4 */ UNMATCHABLE_TOKEN,
    ],
    &[
        5,   /* CommentsTerm */
        6,   /* StringLiteralTerm */
        7,   /* ExponentTerm */
        8,   /* FixedPointTerm */
        9,   /* BasedTerm */
        10,  /* AllBitTerm */
        11,  /* BaseLessTerm */
        28,  /* BackQuoteTerm */
        29,  /* ColonColonLAngleTerm */
        30,  /* ColonColonTerm */
        31,  /* ColonTerm */
        32,  /* CommaTerm */
        33,  /* DotDotEquTerm */
        34,  /* DotDotTerm */
        35,  /* DotTerm */
        36,  /* EquTerm */
        37,  /* HashTerm */
        38,  /* LAngleTerm */
        39,  /* QuoteLBraceTerm */
        40,  /* LBraceTerm */
        41,  /* LBracketTerm */
        42,  /* LParenTerm */
        43,  /* RAngleTerm */
        44,  /* RBraceTerm */
        45,  /* RBracketTerm */
        46,  /* RParenTerm */
        47,  /* SemicolonTerm */
        48,  /* StarTerm */
        49,  /* AlwaysCombTerm */
        50,  /* AlwaysFfTerm */
        51,  /* AssignTerm */
        52,  /* AsTerm */
        53,  /* BitTerm */
        54,  /* CaseTerm */
        55,  /* ClockTerm */
        56,  /* ClockPosedgeTerm */
        57,  /* ClockNegedgeTerm */
        58,  /* ConstTerm */
        59,  /* DefaultTerm */
        60,  /* ElseTerm */
        61,  /* EmbedTerm */
        62,  /* EnumTerm */
        63,  /* ExportTerm */
        64,  /* F32Term */
        65,  /* F64Term */
        66,  /* FinalTerm */
        67,  /* ForTerm */
        68,  /* FunctionTerm */
        69,  /* I32Term */
        70,  /* I64Term */
        71,  /* IfResetTerm */
        72,  /* IfTerm */
        73,  /* ImportTerm */
        74,  /* IncludeTerm */
        75,  /* InitialTerm */
        76,  /* InoutTerm */
        77,  /* InputTerm */
        78,  /* InsideTerm */
        79,  /* InstTerm */
        80,  /* InterfaceTerm */
        81,  /* InTerm */
        82,  /* LetTerm */
        83,  /* LogicTerm */
        84,  /* LsbTerm */
        85,  /* ModportTerm */
        86,  /* ModuleTerm */
        87,  /* MsbTerm */
        88,  /* OutputTerm */
        89,  /* OutsideTerm */
        90,  /* PackageTerm */
        91,  /* ParamTerm */
        92,  /* ProtoTerm */
        93,  /* PubTerm */
        94,  /* RefTerm */
        95,  /* RepeatTerm */
        96,  /* ResetTerm */
        97,  /* ResetAsyncHighTerm */
        98,  /* ResetAsyncLowTerm */
        99,  /* ResetSyncHighTerm */
        100, /* ResetSyncLowTerm */
        101, /* ReturnTerm */
        102, /* BreakTerm */
        103, /* SignedTerm */
        104, /* StepTerm */
        105, /* StringTerm */
        106, /* StructTerm */
        107, /* SwitchTerm */
        108, /* TriTerm */
        109, /* TypeTerm */
        110, /* U32Term */
        111, /* U64Term */
        112, /* UnionTerm */
        113, /* UnsafeTerm */
        114, /* VarTerm */
        115, /* DollarIdentifierTerm */
        116, /* IdentifierTerm */
    ],
);

pub const MAX_K: usize = 3;

pub static SCANNERS: Lazy<Vec<ScannerConfig>> = Lazy::new(|| {
    vec![
        ScannerConfig::new(
            "INITIAL",
            Tokenizer::build(TERMINALS, SCANNER_0.0, SCANNER_0.1).unwrap(),
            &[],
        ),
        ScannerConfig::new(
            "Embed",
            Tokenizer::build(TERMINALS, SCANNER_1.0, SCANNER_1.1).unwrap(),
            &[],
        ),
        ScannerConfig::new(
            "Generic",
            Tokenizer::build(TERMINALS, SCANNER_2.0, SCANNER_2.1).unwrap(),
            &[],
        ),
    ]
});
