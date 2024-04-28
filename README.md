# SIGINT Handler

This SIGINT handler is based upon the [signal-hook](https://github.com/vorner/signal-hook) library
created by Michal Vaner ('vorner') and Thomas Himmelstoss and discussed by James Elford in his
"[Working with signals in Rust](https://www.jameselford.com/blog/working-with-signals-in-rust-pt1-whats-a-signal/)"
article. This article nicely explains the challenges of properly handling signals. 

This crate takes his work and strips out the excellent generic signal handling implemented in the
signal-hook library to create a handler for just SIGINT for use in other projects.

## License

James' original work is licensed under the MIT license so that is preserved here.

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
