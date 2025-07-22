Extended Backus-Naur Form Grammar

```
<S> ::= <A> (" iff " <A>)*
<A> ::= <B> (" if " <B>)*
<B> ::= <C> (" or " <C>)*
<C> ::= <D> (" and " <D>)*
<D> ::= <E> | "!" <E>
<E> ::= <v> | "(" <S> ")"
<v> ::= "x" | "y"
```

