use clap::Parser;

/// 打印一个或多个文件前10行到标准输出
#[derive(Parser, Debug)]
#[command(name = "head")]
#[command(author = "wangzhifei <wangzhifeixiang@qq.com>")]
#[command(version = "1.0.0")]
#[group(args=["line_nums","byte_nums"])]
struct Cli {
    #[arg(value_name = "File", default_value = "-")]
    #[arg(help = "输入文件")]
    files: Vec<String>,
    #[arg(short = 'n', long = "lines")]
    #[arg(help = "打印的行数")]
    line_nums: Option<isize>,
    #[arg(short = 'c', long = "bytes")]
    #[arg(help = "打印的字节数")]
    byte_nums: Option<isize>,
}

fn main() {
    let cli = Cli::parse();
    let files = cli.files;
    for ele in files.iter() {
        println!("{ele}");
    }
    if let Some(lines) = cli.line_nums.as_ref() {
        println!("{}", lines);
    }
    if let Some(bytes) = cli.byte_nums.as_ref() {
        println!("{}", bytes);
    }
}
