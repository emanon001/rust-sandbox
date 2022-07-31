# thread-sample

[std::thread](https://doc.rust-lang.org/stable/std/thread/)のサンプルコード。

## sum

数列の合計値を計算するプログラム。  
数列をスレッド毎に分割して、それぞれのスレッドで合計値を計算することで高速化する。  

### 関連

- [std::thread::spawn](https://doc.rust-lang.org/stable/std/thread/fn.spawn.html)
- [std::sync::Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)

## mutex

スレッド間の共有データへの書き込みを行うプログラム。  

### 関連

- [std::sync::Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)
- [std::sync::Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
