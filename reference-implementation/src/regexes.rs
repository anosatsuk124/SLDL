use lazy_static::lazy_static;

lazy_static! {
    pub static ref INDEX: &'static str = r"([a-z]+)";
    pub static ref PREDICATE_NAME: &'static str = r"([A-Z]+\w*)";
    pub static ref SENTENCE_DEF: &'static str = r"(.+)";
    pub static ref ATOM: &'static str = r"\w+";
    pub static ref COMMA: &'static str = r"\s*,\s*";
    pub static ref COLON: &'static str = r"\s*:\s*";
    pub static ref DEF_OP: &'static str = r"\s*->\s*";
    pub static ref CURLY_BRACKET: (&'static str, &'static str) = (r"\s*\{\s*", r"\s*\}\s*");
    pub static ref ARG: String = format!(r"(\w+)");
    pub static ref ARGS: String = format!(
        r"(\(\s*({Arg}\s*(?:{Comma}{Arg})*)?\s*\))",
        Arg = ARG.to_string(),
        Comma = COMMA.to_string()
    );
    pub static ref PREDICATE: String = format!(
        r"({PredicateName}\s*{Args})",
        PredicateName = PREDICATE_NAME.to_string(),
        Args = ARGS.to_string(),
    );
    pub static ref SENTENCE: String = format!(
        r"({Predicate}{DefOp}{OpenCurlyBracket}(({Index}{Colon}{SentenceDef})+{Comma})*({Index}{Colon}{SentenceDef}){CloseCurlyBracket}|{Predicate}{DefOp}{SentenceDef})",
        Predicate = PREDICATE.to_string(),
        DefOp = DEF_OP.to_string(),
        OpenCurlyBracket = CURLY_BRACKET.0.to_string(),
        CloseCurlyBracket = CURLY_BRACKET.1.to_string(),
        SentenceDef = SENTENCE_DEF.to_string(),
        Index = INDEX.to_string(),
        Colon = COLON.to_string(),
        Comma = COMMA.to_string(),
    );
    pub static ref SENTENCES: String = format!(
        r"(?P<Sentences>Sentence{OpenCurlyBracket}({Sentence}{Comma})*{Sentence}+{CloseCurlyBracket})",
        Sentence = SENTENCE.to_string(),
        OpenCurlyBracket = CURLY_BRACKET.0.to_string(),
        CloseCurlyBracket = CURLY_BRACKET.1.to_string(),
        Comma = COMMA.to_string(),
    );
    pub static ref PROGRAM: String = SENTENCES.clone();
}
