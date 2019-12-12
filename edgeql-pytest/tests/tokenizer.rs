#![allow(unused)]

mod py;


#[test]
fn simple_query() -> py::RunResult {
    py::run("\
        from edb.edgeql._edgeql_rust import tokenize\n\
        tokens = list(map(repr, tokenize('SELECT 1+1;')))\n\
        assert tokens == [
            '<Token SELECT \"SELECT\">',
            '<Token ICONST \"1\">',
            '<Token + \"+\">',
            '<Token ICONST \"1\">',
            '<Token ; \";\">',
        ], tokens
    ")
}

#[test]
fn tokenizer_error() -> py::RunResult {
    py::run("\
        from edb.edgeql._edgeql_rust import tokenize, TokenizerError\n\
        try:\n    \
            tokenize('$``')\n\
        except TokenizerError:\n    \
            pass\n\
        else:\n    \
            raise AssertionError('no tokenizer error')\n\
    ")
}