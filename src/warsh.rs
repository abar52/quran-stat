use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
const DATA_JSON: &str = "data/warshData_v10.json";

#[derive(Serialize, Deserialize)]
struct Aya {
    id: i32,              // "id" : 1,
    jozz: i32,            // "jozz" : 1,
    page: String,         // "page" : "1",
    sura_no: i32,         // "sura_no" : 1,
    sura_name_en: String, // "sura_name_en" : "Al-Fātiḥah",
    line_start: i32,      // "line_start" : 3,
    line_end: i32,        // "line_end" : 3,
    aya_no: i32,          // "aya_no" : 1,
    aya_text: String,     // "aya_text" : "اِ۬لْحَمْدُ لِلهِ رَبِّ اِ۬لْعَٰلَمِينَ ١"
}

pub fn get_ayas() -> Result<()> {
    // Grab JSON file
    let contents = fs::read_to_string(DATA_JSON).expect("Couldn't find or load that file.");
    let ayas: Vec<Aya> = serde_json::from_str(&contents)?;

    println!("ayas count: {}", ayas.len());
    println!("last page: {}", ayas.last().unwrap().page);
    println!("last id: {}", ayas.last().unwrap().id);
    let mut word_count = 0;
    for aya in &ayas {
        let words: Vec<&str> = aya.aya_text.split_whitespace().collect();
        word_count += words.len() - 1;
        match aya.aya_text.find('۩') {
            Some(_) => println!("sajda: {} {} {}", aya.sura_name_en, aya.page, aya.aya_no),
            None => (),
        }
    }
    println!("words: {}", word_count);
    for aya in &ayas {
        if aya.aya_no == 1 {
            println!("sura: {} {}", aya.sura_name_en, aya.page);
        }
    }

    Ok(())
}
