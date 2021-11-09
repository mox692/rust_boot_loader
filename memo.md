* u8配列への参照(つまりslice)
  * [u8] -> 配列そのものを表し、これはcompileできない(配列の長さをコンパイラが汲み取ることができない)
  * &[u8]-> 配列への参照(slice)を表し、これはcompileできる.
  * ref: 文字リテラルにbの接頭辞をつけることで、byte列を表現 https://doc.rust-jp.rs/rust-by-example-ja/std/str.html
  * static HELLO: &[u8] = b"Hello World!";