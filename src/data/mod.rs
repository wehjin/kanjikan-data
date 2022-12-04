use std::collections::HashSet;
use serde::{Deserialize, Serialize};

pub static BOOK_STR: &'static str = include_str!("book.json");

fn book() -> Book {
	let book: Book = serde_json::from_str(BOOK_STR).expect("book in BOOK_STR");
	book
}

pub fn cards() -> Vec<Card> {
	let cards = book().chapters.iter().map(|chapter| {
		let cards = chapter.characters.iter().map(|character| {
			let cards = character.usages.iter().map(|usage| {
				Card {
					prompt: usage.furigana.kanji_or_kana(),
					answer: usage.furigana.kana(),
					meaning: usage.meaning.to_string(),
					chapter_num: chapter.chapter,
					character_num: character.number,
					character_meaning: character.meaning.to_string(),
					character: character.character.to_string(),
				}
			}).collect::<Vec<_>>();
			cards
		}).flatten().collect::<Vec<_>>();
		cards
	}).flatten().collect::<Vec<_>>();
	cards
}

pub fn font() -> Font {
	let mut set: HashSet<char> = HashSet::new();
	fn add_glyphs(set: &mut HashSet<char>, glyphs: &String) {
		for c in glyphs.chars() {
			set.insert(c);
		}
	}
	for card in cards() {
		add_glyphs(&mut set, &card.prompt);
		add_glyphs(&mut set, &card.answer);
	}
	let glyphs = set.iter().collect::<String>();
	Font { glyphs }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Font {
	glyphs: String,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Card {
	prompt: String,
	answer: String,
	meaning: String,
	chapter_num: usize,
	character_num: usize,
	character_meaning: String,
	character: String,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Book {
	chapters: Vec<Chapter>,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Chapter {
	chapter: usize,
	characters: Vec<Character>,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Character {
	number: usize,
	character: String,
	meaning: String,
	usages: Vec<Usage>,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Usage {
	furigana: FuriString,
	meaning: String,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(from = "String", into = "String")]
pub struct FuriString {
	#[serde(skip)]
	pub chars: Vec<FuriChar>,
}

impl From<String> for FuriString {
	fn from(s: String) -> Self {
		let parts = s.split(PARTS_SEPARATOR).collect::<Vec<_>>();
		let chars = parts.into_iter().map(|part| {
			let variants = part.split(VARIANTS_SEPARATOR).collect::<Vec<_>>();
			if variants.len() > 1 {
				FuriChar::Kanji {
					kanji: variants[0].to_string(),
					kana: variants[1].to_string(),
				}
			} else {
				FuriChar::Kana { kana: variants[0].to_string() }
			}
		}).collect::<Vec<_>>();
		FuriString { chars }
	}
}

impl Into<String> for FuriString {
	fn into(self) -> String {
		let s = self.chars.into_iter()
			.map(|char| {
				let variants = match char {
					FuriChar::Kanji { kanji, kana } => {
						format!("{}{}{}", kanji, VARIANTS_SEPARATOR, kana)
					}
					FuriChar::Kana { kana } => {
						kana
					}
				};
				variants
			}).collect::<Vec<_>>().join(PARTS_SEPARATOR);
		s
	}
}

impl FuriString {
	pub fn kana(&self) -> String {
		self.chars.iter().map(|char| char.kana()).collect::<String>()
	}
	pub fn kanji_or_kana(&self) -> String {
		self.chars.iter().map(|char| char.kanji_or_kana()).collect::<String>()
	}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum FuriChar {
	Kanji { kanji: String, kana: String },
	Kana { kana: String },
}

impl FuriChar {
	pub fn kana(&self) -> String {
		match self {
			FuriChar::Kanji { kana, .. } => {
				kana.to_string()
			}
			FuriChar::Kana { kana, .. } => {
				kana.to_string()
			}
		}
	}
	pub fn kanji_or_kana(&self) -> String {
		match self {
			FuriChar::Kanji { kanji, .. } => {
				kanji.to_string()
			}
			FuriChar::Kana { kana, .. } => {
				kana.to_string()
			}
		}
	}
}

const VARIANTS_SEPARATOR: &'static str = "｜";
const PARTS_SEPARATOR: &'static str = "　";