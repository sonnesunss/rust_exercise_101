# 第六部分 - 网络编程 - Rust网络编程基础练习题

1. 简单的TCP客户端
描述: 编写一个Rust程序，连接到指定的TCP服务器并发送一条消息。
要求:
连接到127.0.0.1:8080。

发送字符串"Hello, Server!"。

接收服务器的响应并打印。
提示: 使用std::net::TcpStream。注意处理连接失败的错误。

2. 简单的TCP服务器
描述: 编写一个Rust程序，创建一个TCP服务器，监听客户端连接并回显消息。
要求:
监听127.0.0.1:8080。

接收客户端消息，打印后原样返回。

支持单个客户端连接。
提示: 使用std::net::TcpListener。使用BufReader读取客户端数据。

3. UDP客户端
描述: 编写一个Rust程序，发送UDP数据包到指定服务器。
要求:
向127.0.0.1:8080发送字符串"Hello, UDP!"。

接收服务器的响应并打印。
提示: 使用std::net::UdpSocket。注意UDP是无连接的。

4. UDP服务器
描述: 编写一个Rust程序，创建一个UDP服务器，接收并回显客户端消息。
要求:
绑定到127.0.0.1:8080。

接收客户端消息，打印后原样返回。
提示: 使用std::net::UdpSocket的recv_from和send_to。

5. HTTP客户端请求
描述: 编写一个Rust程序，发送HTTP GET请求并打印响应内容。
要求:
使用reqwest库，向https://httpbin.org/get发送GET请求。

打印响应的状态码和body。
提示: 添加reqwest依赖并使用reqwest::blocking::get。

6. 简单的HTTP服务器
描述: 编写一个Rust程序，创建一个简单的HTTP服务器，响应固定内容。
要求:
使用hyper库，监听127.0.0.1:8080。

对任何请求返回状态码200和字符串"Hello, World!"。
提示: 添加hyper依赖，参考hyper文档中的简单服务器示例。

7. TCP多客户端服务器
描述: 扩展TCP服务器，支持同时处理多个客户端连接。
要求:
监听127.0.0.1:8080。

为每个客户端启动一个线程，接收消息并回显。
提示: 使用std::thread为每个TcpStream创建线程。

8. 超时TCP客户端
描述: 编写一个Rust程序，实现带超时的TCP客户端。
要求:
连接到127.0.0.1:8080，设置5秒连接超时。

发送消息并接收响应。
提示: 使用TcpStream::connect_timeout设置超时。

9. JSON解析HTTP响应 -- todo
描述: 编写一个Rust程序，发送HTTP请求并解析JSON响应。
要求:
使用reqwest向https://httpbin.org/json发送GET请求。

解析响应为JSON，提取某个字段（如slideshow.title）并打印。
提示: 使用serde和serde_json解析JSON，定义对应的Rust结构体。

10. 简单的WebSocket客户端 --todo
描述: 编写一个Rust程序，连接到WebSocket服务器并发送消息。
要求:
使用tungstenite库，连接到ws://echo.websocket.org。

发送字符串"Hello, WebSocket!"并打印服务器响应。
提示: 添加tungstenite依赖，参考其文档中的客户端示例。
