# thread-sample

[std::thread](https://doc.rust-lang.org/stable/std/thread/)のサンプルコード。

## sum

数列を合計値を計算するプログラム。  
数列をスレッド毎に分割して、それぞれのスレッドで合計値を計算することで高速化する。  

### 関連

- [std::thread::spawn](https://doc.rust-lang.org/stable/std/thread/fn.spawn.html)
- [std::sync::Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)
