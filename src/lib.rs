//! Common tree-sitter grammars for tokensave.
//!
//! Tier: **medium** — 17 languages (includes all lite-tier languages).
//!
//! Additional languages: C++, C#, Ruby, Kotlin, Swift, Scala, PHP, TSX, Bash, Lua, Dart

pub use tokensave_lite_treesitters;
pub use tree_sitter;

pub mod languages {
    pub use tokensave_lite_treesitters::languages::*;
    pub use tree_sitter_bash;
    pub use tree_sitter_cpp;
    pub use tree_sitter_c_sharp;
    pub use tree_sitter_dart;
    pub use tree_sitter_kotlin;
    pub use tree_sitter_lua;
    pub use tree_sitter_php;
    pub use tree_sitter_ruby;
    pub use tree_sitter_scala;
    pub use tree_sitter_swift;
}

/// Returns (name, language_fn) pairs for all medium-tier languages.
pub fn all_languages() -> Vec<(&'static str, tree_sitter::Language)> {
    let mut langs = tokensave_lite_treesitters::all_languages();
    langs.extend([
        ("cpp", tree_sitter_cpp::LANGUAGE.into()),
        ("c_sharp", tree_sitter_c_sharp::LANGUAGE.into()),
        ("ruby", tree_sitter_ruby::LANGUAGE.into()),
        ("kotlin", tree_sitter_kotlin::LANGUAGE.into()),
        ("swift", tree_sitter_swift::LANGUAGE_SWIFT.into()),
        ("scala", tree_sitter_scala::LANGUAGE.into()),
        ("php", tree_sitter_php::LANGUAGE_PHP.into()),
        ("tsx", tree_sitter_typescript::LANGUAGE_TSX.into()),
        ("bash", tree_sitter_bash::LANGUAGE.into()),
        ("lua", tree_sitter_lua::LANGUAGE.into()),
        ("dart", tree_sitter_dart::LANGUAGE.into()),
    ]);
    langs
}
