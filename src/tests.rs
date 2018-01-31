use std::fs::File;
use std::io::BufReader;

use {Features, ReadSentence, Reader, Token, TokenBuilder};

lazy_static!{

pub static ref TEST_SENTENCES: Vec<Vec<Token>> =
    vec![
        vec![
            TokenBuilder::new("Die")
                .lemma("die")
                .cpos("ART")
                .pos("ART")
                .features(Features::from_string("nsf"))
                .head(2)
                .head_rel("DET")
                .p_head(3)
                .p_head_rel("TEST")
                .token(),
            TokenBuilder::new("Großaufnahme")
                .lemma("Großaufnahme")
                .cpos("N")
                .pos("NN")
                .features(Features::from_string("nsf"))
                .head(0)
                .head_rel("ROOT")
                .token(),
        ],
        vec![
            TokenBuilder::new("Gilles")
                .lemma("Gilles")
                .cpos("N")
                .pos("NE")
                .features(Features::from_string("nsm"))
                .head(0)
                .head_rel("ROOT")
                .token(),
            TokenBuilder::new("Deleuze")
                .lemma("Deleuze")
                .cpos("N")
                .pos("NE")
                .features(Features::from_string(
                    "case:nominative|number:singular|gender:masculine",
                ))
                .head(1)
                .head_rel("APP")
                .token(),
        ],
    ];
}

pub fn read_sentences(filename: &str) -> Vec<Vec<Token>> {
    Reader::new(BufReader::new(File::open(filename).unwrap()))
        .sentences()
        .map(|s| s.unwrap())
        .collect()
}
