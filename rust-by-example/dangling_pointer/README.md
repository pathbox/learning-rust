let mut x = 5;
let y = &mut x;

y 是一个（指向）可变引用的不可变绑定，它意味着你不能把 y 与其它变量绑定
（ y = &mut z ），不过 y 可以用来把 x 绑定到别的值上（ *y = 5 ）。一个
微妙的区别。

当然，如果你想它们都可变：
let mut x = 5;
let mut y = &mut x;

# enum Message {
#   Move { x: i32, y: i32 },
# }
let x: Message = Message::Move { x: 3, y: 4 };
  enum BoardGameTurn {
  Move { squares: i32 },
  Pass,
}
let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };

枚举并不是简单的枚举了，枚举的每个元素可以是复杂的结构体

# enum Message {
#   Write(String),
# }
let v = vec!["Hello".to_string(), "World".to_string()];
let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();
 