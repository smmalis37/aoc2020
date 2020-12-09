pub trait BStrParse {
    fn parse<F: lexical::FromLexical>(&self) -> Result<F, lexical::Error>;
}

impl BStrParse for [u8] {
    fn parse<F: lexical::FromLexical>(&self) -> Result<F, lexical::Error> {
        lexical::parse(self)
    }
}
