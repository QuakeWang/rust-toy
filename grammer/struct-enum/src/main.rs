#[derive(Debug)]
enum Gender { // 枚举类型
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

// UserId/TopicId：struct 的特殊形式，称为元组结构体，它们的域都是匿名的，可以用索引访问，适用于简单的结构体。
// Clone：让数据可以复制，Copy：让数据结构可以在函数传递的时候自动按字节拷贝
#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

// User/Topic：标准的结构体，可以把任何类型组合在结构体里使用
#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    onwer: UserId,
}

// 定义聊天室中可能发生的事件
// Event：标准的标签联合体
#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn main() {
    let alice = User { id: UserId(1), name: "Alice".into(), gender: Gender::Female };
    let bob = User { id: UserId(2), name: "Bob".into(), gender: Gender::Male };

    let topic = Topic {id: TopicId(1), name: "rust".into(), onwer: UserId(1)};
    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello Rust".into()));

    println!("event1: {:?}, enevnt2: {:?}, event3: {:?}", event1, event2, event3);
}
