# Silver pancake Syntax

### Introduction:

Silver pancake syntax is based on the ML language family. Elm syntax is used as a base. It is simple to read and force developers to write functions signature that can be used by the compiler to generate compile-time exceptions.

## Grouping tokens:

`( ) [ ] { }`

## Unary/binary operators:

`\+ - ! ~ & *`

LSHIFT = `'<<'`  
RSHIFT = `'>>'`  
EQ = `'=='`  
NOTEQ = `'!='`  
LTEQ = `'<='`  
GTEQ = `'>='`  
AND = `'&&'`  
OR = `'||'`

`\+ - | ^` LSHIFT RSHIFT

`\* / % &`

EQ NOTEQ `<` LTEQ `>` GTEQ  
AND  
OR

`? :`

## Assignment operators:

COLON_ASSIGN = `':='`  
ADD_ASSIGN = `'+='`  
SUB_ASSIGN = `'-='`  
OR_ASSIGN = `'|='`  
XOR_ASSIGN = `'^='`  
LSHIFT_ASSIGN = `'<<='`  
RSHIFT_ASSIGN = `'>>='`  
MUL_ASSIGN = `'*='`  
DIV_ASSIGN = `'/='`  
MOD_ASSIGN = `'%='`  
INC = `'++'`  
DEC = `'--'`  

## Names/literals:

NAME = `[a-zA-Z_][a-zA-Z0-9_]*`  
INT = `0 | [1-9][0-9]* | 0[xX][0-9a-fA-F]+ | 0[0-7]+ | 0[bB][0-1]+ `  
FLOAT = `[0-9]*[.]?[0-9]*([eE][+-]?[0-9]+)?`  
CHAR = `'\'' . '\''`  
STR = `'"' [^"]* '"'`  

## AST S-expression format:


```elm
fact n: Int -> Int =
    if n == 0
        1
    else
        n * fact n-1
```
```lisp
(func fact (n int) int
  (if (== n 0)
    (then
      (return 1))
    (else
      (return (* n (fact (- n 1)))))))
```