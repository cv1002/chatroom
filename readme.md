# 简易聊天室项目

## 架构示意图

<center>

```mermaid
graph TB
    Peer         <--> ServiceNode
    ServiceNode  <--> MessageQueue
    ServiceNode   --> DataBase
```

</center>

## 数据结构

<center>

```mermaid
erDiagram
    Room {
      id    u64
      peers Peers
    }
    Peer {
      id       u64
      name     String
      password String
    }
    Message {
      id      u64
      roomid  u64
      time    u64
      speaker u64
      content String
    }
```
```mermaid
graph LR
  Room --> | 多个Room对应多个Peer | Peer
  Room --> | 每个Room对应多个Message | Message
  Peer --> | 每个Peer对应多个Message | Message
```

</center>

- Room
  - 聊天室的抽象
  - 承载一组Peer
  - Room与Peer呈现多对多关系
  - 这是需要永久保存的数据
  ```rust
  struct Room {
      id: u64,

      peers: Peers,
  }
  struct Rooms {
      inner: Vec<Room>,
  }
  ```

- Peer
  - 每个客户端的抽象
  - Room与Peer呈现多对多关系
  - 这是需要永久保存的数据
  ```rust
  struct Peer {
      id: u64,
      passsword: String,

      name: String,
      token: String,
  }
  struct Peers {
      inner: Vec<Peer>,
  }
  ```

- Chat
  - 聊天记录的抽象
  - 承载一个聊天室中的所有聊天记录
  - Chat与Room呈现一对一关系
  - 这是需要妥善保存的数据，一般每隔一段时间清理
  ```rust
  struct Chat {
      // 标识一个聊天室
      id: u64,
      // 该聊天室的最近消息记录
      messages: Messages,
  }
  struct Message {
      // Message的Id，用以快速查询Message
      id: u64,
      // 时间戳格式的Time
      time: u64,
      // 发送聊天记录的人，只记录Id
      speaker: u64,
      // 聊天记录的具体内容
      content: String,
  }
  struct Messages {
      inner: Vec<Message>
  }
  ```