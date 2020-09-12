use structopt::StructOpt;
use std::fmt::{self, Display, Formatter};

#[derive(StructOpt)]
#[structopt(name = "app")]
pub struct AppArgs {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt)]
pub enum Command {
    /// add operation

    //打印加和
    #[structopt(name = "add")]
    Add(Elements),
    // 打印乘积
    #[structopt(name = "times")]
    Times(Elements),

}

#[derive(StructOpt)]
pub struct Elements {
    pub elements: Vec<i32>,
}
//实现display
impl Display for Elements {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{:#?}]", self.elements)
    }
}

fn main() {
    let tmp = AppArgs::from_args();
    let opt = tmp;//所有权转移
    match opt.command {
        
        Command::Add(e) =>{
            println!("Operants: {}", e);
            let mut sum = 0;
            //迭代器访问
            for i in &e.elements {
                sum += i;
            }
            println!("The add result is {}",sum);
        },
        Command::Times(e) =>{
            println!("Operants: {}", e);
            let mut total = 1;
            for i in &e.elements {
                total *= i;
            }
            println!("The times result is {}",total);
        },

    }
}