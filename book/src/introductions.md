# Introductions

> 🏗️ 🚧 *This book is currently under construction.* 🚧 🏗️

**enpassant** is an educational chess library written in Rust. This book is intended to be a journey into the rabbit hole of chess programming, but it is coincidentally a way of learning <a href="https://doc.rust-lang.org/rust-by-example/" target="_blank">Rust by Example</a>. It will not be a comprehensive overview of Rust (see <a href="https://doc.rust-lang.org/book/" target="_blank">the official Rust book</a>), but it only assumes a barebones knowledge of the language. I myself don't know much more than the basics, so I've asked one of my friends who happens to be an expert in Rust to accompany us on our journey.

>**FRIEND:** ...

Why don't you say hi?

>**FRIEND:** ...

Might as well get acquainted, the three of us have got a long road ahea-

>**FRIEND:** I just lost 100 rating points playing blitz on Lichess.

Oh. I see.

By the end of this book, you will be able to not only implement a chess engine in Rust, but in any language that you're comfortable in. In this book we will go over five fundamental questions in chess programming:

1. [How do we represent chess in a way that the computer understands?](boardrepr/boardrepr.md)
2. [In a given position, how does a computer know what its moves are?](movegeneration/movegeneration.md)
3. [How should the computer decide how good a given position is?](eval/eval.md)
4. [How should the computer search through all of its moves?](search/search.md)
5. [How can the computer return its chose move in a universal format?](uci/uci.md)

Answering these questions provide at least the basics of a working chess engine. From there you'll be able to delve into deeper topics and make an even more advanced chess engine.

Let's get started, shall we?
