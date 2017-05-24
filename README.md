# nlp-tokenize
It tokenizes a text file by spaces and line breaks.
It then categorizes the tokens using bitflags.

The token has this format:
```
struct Token {
   index: usize,
   flags: Flags,
   data: String
}
```

Example output:
```
Token { index: 358158, flags: ALPHA, 			data: "TIME" }
Token { index: 358159, flags: ALPHA, 			data: "From" }
Token { index: 358160, flags: ALPHA, 			data: "Amit" }
Token { index: 358161, flags: ALPHA | STOP | BRACKET, 	data: "(Sanskrit)," }
Token { index: 358162, flags: ALPHA, 			data: "which" }
Token { index: 358163, flags: ALPHA, 			data: "means" }
Token { index: 358164, flags: ALPHA, 			data: "endless" }
Token { index: 358165, flags: ALPHA, 			data: "or" }
Token { index: 358166, flags: ALPHA | FULL_STOP, 	data: "immeasurable." }
```

Benchmark 1:
```
Using HashSet for flags
2Mb/s	338000 tokens/s
```

Benchmark 2:
```
Using Bitflags
446600 tokens/s
```

Run it like this:
`cargo run -- <FILE>`
