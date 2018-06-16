# Vlad

## OVERVIEW

Vlad is the functional interpretable programming language for
mathematics and other computations

## SYNTAX

```
line: function
    | statement

function: fn name(args) -> (return types),* {
    statement
}

type : number | bool ( | list ? )

number : natural | integer | rational | real | complex
natural : > 0 -> utype
integer :  -> itype
rational : p / q -> (integer, integer)
real : -> ftype
complex : a + bi -> (real, real)

bool: true | false

( list: [head | tail] ? )

args : (arg:type),*

statement : expression
          | conditional_statement
          
conditional_statement: if condition: statement (else: (statement | conditional_statement))? 

condition : bool
          | expression (EQL | GT | LT | GTE | LTE | NEQ) expression
          | condition ((AND|OR|XOR) condition)?

expression : term ((PLUS | MINUS) term)*

term : factor ((MUL | DIV) factor)*

factor : PLUS factor
       | MINUS factor
       | INTEGER
       | LPAREN expression RPAREN
       | argument

argument : ID
```

## EXAMPLES

```
fn x_cos(x: float) -> float {
    x * cos(x)
}
```