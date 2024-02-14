enum TYPE {
    DOT,
    COMMA,
    SPACE
}

struct token {
    kind:TYPE,
    tokenV: &str
}