 * use an enum instead of a struct for TokenType
 * _far_ more robust dataflow for errors throughout the compiler
 * switch to a query type system for lexing, parsing & interpreting
    - incremental compilation :)
 * split src by '\n', allows for fast relexing
 * map out _specifically_ where Impetuous can be used, and where Iterator must be used
