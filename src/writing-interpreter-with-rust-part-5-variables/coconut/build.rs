use cfgrammar::yacc::YaccKind;
use lrlex::CTLexerBuilder;

fn main() {
    CTLexerBuilder::new()
        .rust_edition(lrlex::RustEdition::Rust2021)
        .lrpar_config(|ctp| {
            ctp.yacckind(YaccKind::Grmtools)
                .rust_edition(lrpar::RustEdition::Rust2021)
                .grammar_in_src_dir("coconut.y")
                .unwrap()
        })
        .lexer_in_src_dir("coconut.l")
        .unwrap()
        .build()
        .unwrap();
}
