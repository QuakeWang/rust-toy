# HTTPie

以实现 [HTTPie](https://httpie.io/) 为例，来看看 Rust 怎么做 CLI。HTTPie 是用 Python 开发的一个类似 cURL 但对用户更加友善的命令行工具，它可以帮助我们更好地诊断 HTTP 服务。

## 功能分析

梳理一下要实现哪些主要功能：

- 首先是做命令行解析，处理子命令和各种参数，验证用户的输入，并且将这些输入转换成能理解的参数；
- 之后根据解析好的参数，发送一个 HTTP 请求，获得响应；
- 最后使用终端输出响应。

## 流程设计

- 用户：使用 httpie 发送请求 `httpie post https://httpbin.org/post greeting=hola name=QuakeWang`
- 命令行解析：Rust 里，一切都是类型，命令行解析出来的参数，可以用一个结构体保存，可使用 clap 完成；
- HTTP 请求：使用 reqwest 处理 HTTP 请求；
- pretty print：对 HTTPie 来说，漂亮可读的输出很重要，使用 colored 可以为输出加上颜色。

## 使用到的库

- 对于命令行解析，Rust 有很多库可以满足这个需求，我们今天使用官方比较推荐的 [clap](https://github.com/clap-rs/clap)。
- 对于 HTTP 客户端，则可以使用 [reqwest](https://github.com/seanmonstar/reqwest) 的异步接口来完成。
- 对于格式化输出，为了让输出像 Python 版本的 HTTPie 那样显得生动可读，引入一个命令终端多彩显示的库，这里选择比较简单的 [colored](https://github.com/colored-rs/colored)。
- 除此之外，我们还需要一些额外的库：用 anyhow 做错误处理、用 jsonxf 格式化 JSON 响应、用 mime 处理 mime 类型，以及引入 tokio 做异步处理。

## 实现过程

### 创建项目添加依赖

```shell
cargo new httpie
cd httpie

cargo add anyhow@1.0.72
cargo add clap@3 --features derive
cargo add colored@2.0.4
cargo add jsonxf@1.1.1
cargo add mime@0.3.17
cargo add reqwest@0.11.18 --features json
cargo add tokio@1.29.1 --features full
cargo add syntect@4
```

### 代码实现

作为一个简单的 demo 我们只需实现 post 和 get 这两个子命令即可。代码中用到了 clap 提供的宏让 CLI 变得简单，这个宏能够生成一些额外的代码帮助处理 CLI 的解析。只需要 **先用一个数据结构 T 描述 CLI 都会捕获什么数据，之后通过 T::parse() 就能解析出各种命令行参数啦。

```rust
use clap::Parser;

// 定义 httpie 的 CLI 文档入口，它包含若干个子命令

/// A native httpie implementation with Rust, can you imagine how easy it is?
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "QuakeWang")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

/// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
struct Get {
    /// HTTP 请求的 URL
    url: String,
}

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you.
#[derive(Parser, Debug)]
struct Post {
    /// HTTP 请求的 URL
    url: String,
    /// HTTP 请求的 body
    body: Vec<String>,
}
```

那么围绕这两个命令来进一步看看能扩展出哪些方法吧。

首先，对于输入的内容需要确保其是合理正确的，所以要做两个验证分别为，一是验证 URL，另一个是验证 body。

clap 允许用户为每个解析出来的值添加自定义的解析函数，这里我们定义一个 parse_url 函数来验证 URL 是合法的：

```Rust
use anyhow::Result;
use reqwest::Url;

#[derive(Clap, Debug)]
struct Get {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;

    Ok(s.into())
}
```

对于 body 的检测会复杂一点，可以定一个结构体 KvPair 来存储 body 的信息，并且使用 parse_kv_pair 来解析传入的参数是否为 key=value 的格式。

```rust
use std::str::FromStr;
use anyhow::{anyhow, Result};

#[derive(Clap, Debug)]
struct Post {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    /// HTTP 请求的 body
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

/// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug)]
struct KvPair {
    k: String,
    v: String,
}

/// 当我们实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用 = 进行 split，这会得到一个迭代器
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            // 从迭代器中取第一个结果作为 key，迭代器返回 Some(T)/None
            // 将其转换成 Ok(T)/Err(E)，然后用 ? 处理错误
            k: (split.next().ok_or_else(err)?).to_string(),
            // 从迭代器中取第二个结果作为 value
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

/// 因为我们为 KvPair 实现了 FromStr，这里可以直接 s.parse() 得到 KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}
```

这里我们实现了一个 [FromStr trait](https://doc.rust-lang.org/std/str/trait.FromStr.html)，可以把满足条件的字符串转换成 KvPair。FromStr 是 Rust 标准库定义的 trait，实现它之后，就可以调用字符串的 parse() 泛型函数，很方便地处理字符串到某个类型的转换了。

到这里基本的验证已经完成了，在代码设计的过程中我们要掌握符合软件开发的开闭原则（[Open-Closed Principle](https://en.wikipedia.org/wiki/Open%E2%80%93closed_principle)），**通过实现额外的验证函数和 trait 来完成**，而不是一股脑直接把代码塞进住流程。这样才能保证新添加的代码，高度可复用，且彼此独立，并且不用修改住流程。

有了基本的类型，以及验证方式，最后就是核心功能：HTTP 的请求处理了。这部分我们可以直接编写 get 和 post 对应的方法，然后在 main 函数中去调用即可。代码实现如下：

```rust
use reqwest::{header, Client, Response, Url};
use std::{collections::HashMap, str::FromStr};

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    println!("{:?}", resp.text().await?);
    Ok(())
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    println!("{:?}", resp.text().await?);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    // 生成一个 HTTP 客户端
    let client = Client::new();
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
}
```

注意看，这里的 main 函数变成了 async fn，表示的是异步函数。对于 async main，需要使用 #[tokio::main] 宏来自动添加处理异步的运行时。还有一点就是，解析出来的 KvPair 需要装入一个 HashMap，然后传入给 HTTP client 的 JSON 方法。这样 HTTPie 的基本功能就完成啦～～

不过，这里只是讲解了其核心功能的实现，具体的可以参考源码 [main.rs](../httpie/src/main.rs)，里面对于终端输出的信息做了语法高亮的处理，提升了用户体验。

## 使用过程

使用 `cargo build --release`，编译出 release 版本，并将其拷贝到某个 `$PATH` 下的目录，在终端中输入 httpie 即可出现对应的提示信息。
