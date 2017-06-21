use std::fs::File;
use std::io::BufReader;

use {Features, ReadSentence, Reader, Sentence, TokenBuilder};

lazy_static!{

pub static ref TEST_SENTENCES: Vec<Sentence> = 
    vec![
        Sentence::new(vec![
            TokenBuilder::new()
                .form("Die")
                .lemma("die")
                .cpos("ART")
                .pos("ART")
                .features(Features::from_string("nsf"))
                .head(2)
                .head_rel("DET")
                .token(),
            TokenBuilder::new()
                .form("Großaufnahme")
                .lemma("Großaufnahme")
                .cpos("N")
                .pos("NN")
                .features(Features::from_string("nsf"))
                .head(0)
                .head_rel("ROOT")
                .token(),
        ]),
        Sentence::new(vec![
            TokenBuilder::new()
                .form("Gilles")
                .lemma("Gilles")
                .cpos("N")
                .pos("NE")
                .features(Features::from_string("nsm"))
                .head(0)
                .head_rel("ROOT")
                .token(),
            TokenBuilder::new()
                .form("Deleuze")
                .lemma("Deleuze")
                .cpos("N")
                .pos("NE")
                .features(Features::from_string(
                    "case:nominative|number:singular|gender:masculine",
                ))
                .head(1)
                .head_rel("APP")
                .token(),
        ]),
    ];
}

pub fn read_sentences(filename: &str) -> Vec<Sentence> {
    Reader::new(BufReader::new(File::open(filename).unwrap()))
        .sentences()
        .map(|s| s.unwrap())
        .collect()

}
