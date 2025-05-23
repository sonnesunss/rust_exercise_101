# Basic

```
程序 -> 文件写入器 (BufWriter) -> 写入缓冲区 -> 文件句柄 (File) -> 文件系统 -> 文件
文件 -> 文件系统 -> 文件句柄 (File) -> 读取缓冲区 -> 文件读取器 (BufReader) -> 程序
```

1. 文件, 所谓文件就是文件系统中的数据实体，rust使用std::fs::File或者tokio::fs::File结构体表示

2. 文件句柄或者文件描述符，操作系统分配的标识符，操作系统使用文件描述符跟踪文件的状态，如文件指针位置、打开模式等， rust的File结构体封装了它，持有一个操作系统级别的文件描述符（file descriptor），并通过它与文件系统交互. 在使用File open、create等操作返回的就是文件描述符

3. 文件缓冲区， 文件缓冲区是程序和文件之间的临时内存区域， 用于优化IO操作的性能.

    + 文件写入缓冲区：在写入文件时，数据通常先写入缓冲区，而不是直接写入磁盘。缓冲区满或显式刷新（如调用 flush）时，数据才会写入文件。这减少了对底层文件系统的直接访问，提高效率。

    + 文件读取缓冲区：在读取文件时，数据通常从文件读取到内存中的缓冲区，程序再从缓冲区读取数据。同样，这减少了对文件系统的直接访问。

    + 在 Rust 中，缓冲区通常由特定的类型（如 std::io::BufReader 或 std::io::BufWriter）管理，而不是直接由 std::fs::File 提供

4. 文件写入器/读取器

文件写入器/读取器是一个抽象概念，通常指实现了 std::io::Write/Read trait 的类型，用于向某个目标（比如文件、内存缓冲区、网络流等）写入数据。

在 Rust 中，std::fs::File 本身实现了 std::io::Write/Read，所以它可以直接作为写入器使用，调用 write/read 或 write_all/read_to_end等方法写入数据。

更常见的“文件写入器”是指 std::io::BufWriter<T>，它是对某个写入目标（例如 File）的包装，提供了缓冲功能。

BufWriter/BufReader 在内部维护一个内存缓冲区，数据先写入缓冲区，缓冲区满或显式调用 flush 时，数据才会写入底层目标（如文件）。

使用 BufWriter/BufReader 的好处是减少对底层文件系统的直接写操作，从而提高性能。


## 大概实现

```rust
pub struct File {
    file_descriptor: u32,  // 不同os是不同的，只是简单演示
    mode: FileMode,
    // etc
}

pub struct BufReader {
    file: File,
    buffer: MemoryBuffer,  // 一块儿内存区域, 作为缓冲区，它有一定的缓存策略，数据先进入这里，根据缓存策略决定何时真正写入文件中
    buffer_size: usize
    // etc
}

impl std::io::Read for File {
    pub fn read(buf: &mut [u8]) -> Result<()> { }
    pub fn read_to_end(buf: &mut [u8]) -> Result<()> {}
}
```
