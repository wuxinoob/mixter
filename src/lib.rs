#![feature(file_lock)]
  pub mod control_blk{
    
    use std::{default, fs::File, process::exit, time::Duration};

    use super::encrypt_hex::*;
    pub fn pause()
    {
        use::std::thread::sleep;
        sleep(Duration::new(3, 0));
    }
    pub fn lock_f(file:File)
    {
        if !file.try_lock().ok().unwrap() {
            println!("获取文件锁失败，请关闭其他打开了此文件的程序");
            lock_f(file);
        }
    }
    pub fn control_seq(fd: &mut File,fname:&str)
    {
        if !is_edited(fd){
            println!("检测到压缩包 ---{fname}--- 未混淆，是否要进行混淆？");
            let res = select_option_jump();
            if res ==1{encrypt(fd);}
        }
        else {
            println!("检测到压缩包 ---{fname}--- 存在混淆，去混淆");
            decrypt(fd);
        }
        println!("完成！");
    }
    fn encrypt(fd: &mut File){
        edit_hex(fd);
        edit_tail(fd);

    }
    fn decrypt(fd: &mut File){
        edit_hex(fd);
    }
    ///继续或退出的交互流程
    pub fn select_option_go_on()
    {
        println!("请输入数字");
        println!("1. 继续");
        println!("2. 退出");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "1" => {return;}
            "2" => {println!("程序将在3秒内退出");pause();exit(0)}
            _default => {select_option_go_on();return;}
        }
    }
    /// 继续或跳过的流程
    pub fn select_option_jump() ->i64
    {
        println!("请输入数字");
        println!("1. 继续");
        println!("2. 跳过");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "1" => {return 1;}
            "2" => {println!("跳过此压缩包");return 2;}
            _default => {select_option_jump();return 3;}
        }
    }
    }

pub mod encrypt_hex{
    use std::fs::File;
    use std::fs;
    use std::io::{ Read, Seek, SeekFrom, Write};

    use crate::control_blk::select_option_go_on;

    pub fn get_fd(file_name:&str) ->File //返回对应文件描述符
    {
        
        let fd =match fs::OpenOptions::new().read(true).write(true).open(file_name){
            Ok(fd) => fd,
            Err(_) => {
                println!("文件{file_name}被占用！请尝试解除占用或退出！");
                select_option_go_on();
                get_fd(file_name)
            }
        };
        fd
    }

    ///是否有尾部特征值,若有则去除
    pub fn is_edited(fd:&mut File) -> bool
    {
        let len = fd.metadata().unwrap().len();
        if len < 4096 {return false;}
        fd.seek(SeekFrom::End(-4096)).unwrap();
        let mut buf =vec![0;4096];
        fd.read(&mut buf).unwrap();
//        //dbg!(&buf);
        for i in &buf[..]
        {
            if *i != 50 {return false};
        }
        fd.set_len(len-4096).unwrap();
        true
    }
    ///追加尾部特征值
    pub fn edit_tail(fd:&mut File)
    {
        fd.seek(SeekFrom::End(0)).unwrap();
        let append_str:Vec<u8> = vec![50;4096];
        fd.write_all(&append_str).unwrap();
    }
    ///调换首部，尾部的字节，并且加上乱码假尾部
    pub fn edit_hex(fd:&mut File)//该操作可逆，重复操作就能还原
    {
        if fd.metadata().unwrap().len() <8192 {//压缩包小于8kb时直接全部倒转
            let mut array = Vec::new();
            fd.seek(SeekFrom::Start(0)).unwrap();
            fd.read_to_end(&mut array).unwrap();
            array.reverse();
//            //dbg!(&array);
            fd.seek(SeekFrom::Start(0)).unwrap();
            fd.write_all(&array).unwrap();
        }//否则分别倒转首尾4kb   
        else {
        let mut array_head:Vec<u8> = vec![0;4096];
        fd.seek(SeekFrom::Start(0)).unwrap();
        fd.read_exact(&mut array_head).unwrap();//首部的字节
        fd.seek(SeekFrom::Start(0)).unwrap();
        array_head.reverse();
        fd.write_all(&array_head).unwrap();

        let mut array_tail:Vec<u8> = vec![0;4096];
        fd.seek(SeekFrom::End(-4096)).unwrap();
        fd.read_exact(&mut array_tail).unwrap();
        array_tail.reverse();
        fd.seek(SeekFrom::End(-4096)).unwrap();
        fd.write_all(&array_tail).unwrap();
    }
    }
    pub fn get_wanted_dir_file() -> Vec<String>
    {
        let mut path_vec = Vec::new();
        let path = std::env::current_dir().unwrap();
        for i in fs::read_dir(path).unwrap() {
            let i = i.unwrap();
            path_vec.push(i.file_name().into_string().unwrap());
        }

        let mut compress_f:Vec<String> = Vec::new();
        //dbg!(&path_vec); 
        for f in path_vec{
            let i:Vec<&str> = f.split(".").collect();
            if i.contains(&"zip") || i.contains(&"7z") || i.contains(&"rar")
            {compress_f.push(f);}
        }
        compress_f
    }

}
