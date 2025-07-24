Extended Backus-Naur Form Grammar

```
<S> ::= <A> (" iff " <A>)*
<A> ::= <B> (" if " <B>)*
<B> ::= <C> (" or " <C>)*
<C> ::= <D> (" and " <D>)*
<D> ::= <E> | "not" <E>
<E> ::= <v> | "(" <S> ")"
<v> ::= "x" | "y"
```

[Operator Precedence](http://logic.stanford.edu/intrologic/dictionary/operator_precedence.html)

```
¬   <-- highest precedence
∧
∨
⇒
⇔   <-- lowest precedence
```

TODO: proper error handling

Build instructions:
```
cargo build
```
