# nlp-tokenize
It opens a text file, turns it into a byte (u8) vector and tokenizes it by spaces and line breaks.

It then categorizes the tokens using HasSets.

Benchmark has results of 2Mb/s, or 338000 tokens/s. If someone knows how to increase performance, I value your feedback. =)

Run it like this:
`cargo run -- <FILE>`
