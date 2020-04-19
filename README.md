# Propositional Tableaux Solver

A little propositional formula satisfiability solver using the propositional 
tableaux method.

See http://www.dis.uniroma1.it/liberato/planning/tableau/tableau.html.

## Syntax of Propositional Formula

The input to the solver must be a well-formed propositional formula.

It can be described by the following BNF grammar:

```enbf
<formula>   ::= <propositional-variable>
            |   - <formula>                 # negation
            |   ( <formula> ^ <formula>  )  # conjunction
            |   ( <formula> | <formula>  )  # disjunction
            |   ( <formula> -> <formula>  ) # implication
            |   ( <formula> <-> <formula> ) # bi-implication
```

Whitespaces are stripped and ignored, and a `<propositional-variable>` can be
any sequence of alphanumeric characters `[a-zA-Z][a-zA-Z0-9]*` that begin with
a alphabet character. Cases are respected and `aaa` is a different
propositional variable from `AAA`.

## Running via Cargo

Two ways to supply the propositional formula exist, with the `-c` switch method
taking precedence:

1. CLI argument switch `-c <input_string>`
2. IO redirection

### Command-line String

Using the `-c <input_string>`

```bash
$ cargo run -c "(a^b)"
```

### IO Redirection

Alternatively, redirect the standard input `stdin` to the solver to supply the
propositional formula.

```bash
$ cat input.txt > cargo run
```
