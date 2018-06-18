# Vlad

## OVERVIEW

Vlad is the functional interpretable programming language for
mathematics and other computations

## SYNTAX

```
<line> ::= <function>
         | <statement>

<function> ::= FN <name> LPAREN [<argument> (COMMA <argument>)*] RPAREN BEGIN <statement> END

<name> ::= ID
<argument> ::= ID

<statement> ::= <expression>
              | <conditional_statement>
          
<conditional_statement> ::= IF <compound_condition> BEGIN <statement> END ELSE BEGIN <statement> END 

<compound_condition> ::= <condition> ((OR | AND | XOR) <condition>)*

<condition> ::= <bool>
              | LPAREN <compound_condition> RPAREN
              | NOT <compound_condition>
              | <simple_condition>
          
<bool> ::= TRUE | FALSE
          
<simple_condition> ::= <expression> (EQ | NE | LT | GT | LE | GE) <expression>

<expression> ::= <term> ((PLUS | MINUS) <term>)*

<term> ::= <factor> ((MUL | DIV) <factor>)*

<factor> ::= PLUS <factor>
          | MINUS <factor>
          | NUMBER
          | LPAREN <expression> RPAREN
          | <function call>
          | <variable>

<variable> ::= ID

<function_call> ::= <name> LPAREN [<expression> (COMMA <expression>)* ] RPAREN

```

## EXAMPLES

```
#>> fn x_cos(x: real) -> real {
#/>     x * cos(x)
#/> }
#<# function x_cos: (real) -> real
#>> x_cos(0.5)
#<< 0.438791281
#>> fn abs(x:real) -> real {
#/>     if x < 0 {
#/>         -x
#/>     } else {
#/>         x
#/>     }
#/> }
#<< function abs: (real) -> real
#>> abs(-3)
#<< 3
#>> abs(3)
#<< 3
#>>
```