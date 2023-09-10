use clap::Parser;

// 创建一个命令行程序，首先需要对命令行参数抽象
// 该结构体就是对命令行参数的抽象
// 对命令行参数抽象就是对未知参数和选项参数用合适类型的字段表示
// 抽象出的结构体上面的文档注释就是对命令行程序的简单介绍(about)
/// 一个简单的欢迎人的程序(我是文档注释)
#[derive(Parser, Debug)]
// #[command(author,version,about,long_about=None)]
#[command(name = "greet")]
#[command(author = "王志飞 <wangzhifeixiang@qq.com>")]
#[command(version)]
#[command(author)]
#[command(about = "一个简单的欢迎程序(我是about的值)")]
// #[command(long_about="一个简单的欢迎程序(我是long_about的值)")]
#[command(long_about)]
struct Args {
    /// 被欢迎的人的名字
    #[arg(short, long)]
    name: String,
    /// 欢迎的次数
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args: Args = Args::parse();
    for _ in 0..args.count {
        println!(
            "{} {}",
            if args.name.is_empty() {
                "hello"
            } else {
                "hello, "
            },
            args.name
        );
    }
}
