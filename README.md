TODO: proper error handling

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

Operator Precedence

```
¬   <-- highest precedence
∧
∨
⇒
⇔   <-- lowest precedence
```

