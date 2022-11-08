// 定义特征
// 使用 trait 关键字来声明一个特征，Summary 是特征名。
// 在大括号中定义了该特征的所有方法，在这个例子中是： fn summarize(&self) -> String
pub trait Summary {
    fn summarize(&self) -> String;
}

// 为类型实现特征
pub struct Post {
    pub title: String,
    // 标题
    pub author: String,
    // 作者
    pub content: String, // 内容
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}


// 关于特征实现与定义的位置，有一条非常重要的原则：如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的！
// 例如我们可以为上面的 Post 类型实现标准库中的 Display 特征，这是因为 Post 类型定义在当前的作用域中。
// 同时，我们也可以在当前包中为 String 类型实现 Summary 特征，因为 Summary 定义在当前作用域中。
//
// 但是你无法在当前作用域中，为 String 类型实现 Display 特征，因为它们俩都定义在标准库中，
// 其定义所在的位置都不在当前作用域，跟你半毛钱关系都没有，看看就行了。
//
// 该规则被称为孤儿规则，可以确保其它人编写的代码不会破坏你的代码，也确保了你不会莫名其妙就破坏了风马牛不相及的代码。


// 使用特征作为函数参数
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 特征对象
// todo!();