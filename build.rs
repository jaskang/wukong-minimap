// use std::{fs, path::Path};

fn main() {
    // 注释掉不需要的 b1sdk 链接
    // println!("cargo:rustc-link-lib=static=b1sdk");
    // println!("cargo:rustc-link-search=native=./target");

    // 获取输出目录环境变量
    // let out_dir = std::env::var("OUT_DIR").unwrap();
    // let profile = std::env::var("PROFILE").unwrap();

    // // 设置源 DLL 路径和目标路径
    // let target_dir = Path::new(&out_dir)
    //     .ancestors()
    //     .nth(3)
    //     .unwrap()
    //     .to_path_buf();

    // let dll_name = "wukong_minimap.dll";
    // // 修改为你的实际路径
    // let source_path = Path::new("target/release").join(dll_name);
    // // 修改为你的游戏实际安装路径
    // let dest_path = Path::new("D:/Games/BlackMythWukong/b1/Binaries/Win64").join(dll_name);

    // println!("cargo:rerun-if-changed={}", source_path.display());

    // if source_path.exists() {
    //     match fs::copy(&source_path, &dest_path) {
    //         Ok(_) => println!("成功复制 DLL 到游戏目录"),
    //         Err(e) => println!("复制 DLL 失败: {}", e),
    //     }
    // } else {
    //     println!("未找到源 DLL: {}", source_path.display());
    // }
}
