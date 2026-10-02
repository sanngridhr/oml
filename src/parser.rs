use std::{
    borrow::Cow,
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use regex::{Matches, Regex};

#[derive(Debug)]
pub struct ASTNode {
    word: String
}

pub fn lex(code: String) -> Vec<ASTNode> {
    return prepare(code).iter().map(|w| classify(w).unwrap()).collect();
}

fn classify(word: &String) -> Result<ASTNode, String> {
    return match word.as_str() {
        _ => Ok(ASTNode { word: word.to_string() }),
    };
}

struct CodeWithStrings {
    code: String,
    strings: HashMap<String, String>,
}

fn prepare(code: String) -> Vec<String> {
    let re_comments: Regex = Regex::new(r"\(\*.*?\*\)").unwrap();
    let commented: Cow<'_, str> = re_comments.replace_all(&code, "");

    let mut hasher: DefaultHasher = DefaultHasher::new();
    let CodeWithStrings {
        code: stringed,
        strings,
    } = hash_strings(commented.to_string(), &mut hasher);

    let re_pad: Regex = Regex::new(r"\(\)|\{\}|\[\]|[(){}\[\]]").unwrap();
    let padded: Cow<'_, str> = re_pad.replace_all(&stringed, " $0 ");

    let re_split: Regex = Regex::new(r"\s+").unwrap();
    let split: Vec<String> = re_split
        .split(&padded)
        .map(|s: &str| {
            if strings.contains_key(&s.to_string()) {
                strings.get(&s.to_string()).unwrap().to_owned()
            } else {
                s.to_string()
            }
        })
        .collect();
    return split;
}

fn hash_strings(code: String, hasher: &mut DefaultHasher) -> CodeWithStrings {
    let mut code_clone: String = code.clone();
    let mut strings: HashMap<String, String> = HashMap::new();

    let re_strings: Regex = Regex::new("\".+?\"").unwrap();
    let matches: Matches<'_, '_> = re_strings.find_iter(&code);

    for m in matches {
        let s: &str = m.as_str();

        let _: () = s.hash(hasher);
        let hash: String = hasher.finish().to_string();

        code_clone = code_clone.replace(s, &hash);
        strings.insert(hash.to_string(), s.to_string());
    }

    return CodeWithStrings {
        code: code_clone,
        strings,
    };
}
