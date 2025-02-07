use std::fs::File;
use std::process::exit;
use mixter::control_blk::*;
use mixter::encrypt_hex::{get_fd, get_wanted_dir_file};

fn main()
{
    println!("本程序用来可以简单的混淆或去混淆文件头、尾，从而防止自动文件检查等情况");
    println!("本程序将读取当前文件路径下的以.zip/rar/7z结尾的文件并处理，请确保该程序与目的压缩包在同一路径下");
    println!("项目地址为：https://github.com/wuxinoob/mixter");
    select_option_go_on();
    let mut file_vec = get_wanted_dir_file();
    if file_vec.len() == 0 {println!("当前目录下没有.zip/rar/7z压缩文件，请确认路径是否正确");pause();exit(0)}
    println!("欲处理文件为：");
    for i in &file_vec {   println!("{i}")   }
    //dbg!(&file_vec);
    select_option_go_on();
        let mut fd_vec:Vec<File> = Vec::new(); 
        for i in &file_vec{
        fd_vec.push(get_fd(i));
    }
    
    fd_vec.reverse();
    file_vec.reverse();

    while fd_vec.len()>0{
        control_seq(&mut fd_vec.pop().unwrap(),& file_vec.pop().unwrap());
    }
    println!("程序将在3秒后退出");
    pause();
}