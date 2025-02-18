# Tiny BASIC interpreter implemented in Rust

## Short description

The implementation is based upon [this](http://www.ittybittycomputers.com/IttyBitty/TinyBasic/DDJ1/Design.html) design note. The interpreter runs in an interactive mode in which user inputs code. The program is not translated into some intermediate representation, i.e. there is no abstract syntax tree involved. Each line is interpreted as is according to this scheme:

`Line of code -> CharStream -> Interpreter`

The `CharStream` provides methods to consume keywords, numbers, etc., while the interpreter executes the code line using the grammar from the aforementioned design note.

## Some caveats of the current implementation

1. All input is required to be ASCII-only. UTF-8 support might be added later.
2. Line numbers are in range `[0; 32767]`;
3. Numbers are 16-bit signed integers;
4. If the condition in `IF` statement is evaluated to false, the statement following `THEN` is ignored, so any syntax errors which might be present in it will not be detected.