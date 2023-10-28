# 网络编程

## std::net

Rust 标准库提供了std::ne，为整个 TCP/IP 协议栈的使用提供了封装。

- TCP：TcpListener / TcpStream，处理服务器的监听以及客户端的连接；
- UDP：UdpSocket，处理 UDP Socket
- 其它：IpAddr 是 IPv4 和 IPv6 地址的封装；SocketAddr，表示 IP 地址 + 端口的数据结构

如果要创建一个 TCP server，我们可以使用 TcpListener 绑定某个端口，然后用 loop 循环处理接收到的客户端请求。接收到请求后，会得到一个 TcpStream，它实现了 Read / Write trait，可以像读写文件一样，进行 socket 的读写：[async_server](/advanced/network/examples/async_listener.rs)。

对于客户端，我们可以用 TcpStream::connect() 得到一个 TcpStream。一旦客户端的请求被服务器接受，就可以发送或者接收数据：[async_client](/advanced/network/examples/async_client.rs)。

## rocket

在 HTTP 协议下，基本上使用 JSON 构建 REST API / JSON API 是业界常见的处理方法，客户端和服务器一般来说也有足够好的生态系统来支持这样的处理。从开发者的角度来看，只需要使用 serde 让定义的 Rust 数据结构具备 Serialize / Deserialize 的能力，然后使用 serde_json 生成序列化后的 JSON 数据。

可以参考下面这个例子：[rocker_server](/advanced/network/examples/rocket_server.rs)

Rocket 是 Rust 的一个全功能的 Web 框架，类似于 Python 的 Django。通过使用 rocket，数十行代码即可运行一个 Web Server。
