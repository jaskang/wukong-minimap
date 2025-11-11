/// 示例：读取玩家坐标
/// 
/// 这个示例展示了如何使用 PlayerReader 来读取游戏中玩家的位置和旋转信息。
/// 
/// 使用方法：
/// 1. 启动黑神话悟空游戏
/// 2. 运行: cargo run --example read_player_position

use wukong_minimap::{PlayerReader, MemoryError};
use std::time::Duration;
use std::thread;

fn main() {
    println!("========================================");
    println!("  黑神话悟空 - 玩家坐标读取器");
    println!("========================================\n");

    println!("正在初始化内存读取器...");
    
    // 创建玩家读取器
    let reader = match PlayerReader::new() {
        Ok(r) => {
            println!("✓ 内存读取器初始化成功！");
            r
        }
        Err(e) => {
            eprintln!("✗ 初始化失败: {:?}", e);
            eprintln!("\n请确保：");
            eprintln!("1. 游戏正在运行");
            eprintln!("2. 游戏进程名为 b1-Win64-Shipping.exe");
            eprintln!("3. 以管理员权限运行此程序");
            return;
        }
    };

    println!("\n开始监控玩家位置... (按 Ctrl+C 退出)\n");

    let mut last_x = 0.0;
    let mut last_y = 0.0;
    let mut last_z = 0.0;
    let mut read_count = 0;
    let mut error_count = 0;

    loop {
        match reader.get_player_info() {
            Ok(info) => {
                read_count += 1;
                error_count = 0; // 重置错误计数

                // 计算移动距离
                let dx = info.location.x - last_x;
                let dy = info.location.y - last_y;
                let dz = info.location.z - last_z;
                let distance = (dx * dx + dy * dy + dz * dz).sqrt();

                // 清屏（简单方式）
                print!("\x1B[2J\x1B[1;1H");

                println!("========================================");
                println!("  玩家位置信息 #{}", read_count);
                println!("========================================");
                println!();
                println!("📍 位置坐标:");
                println!("   X: {:.2}", info.location.x);
                println!("   Y: {:.2}", info.location.y);
                println!("   Z: {:.2}", info.location.z);
                println!();
                println!("🧭 旋转角度:");
                println!("   Pitch: {:.2}°", info.rotation.pitch);
                println!("   Yaw:   {:.2}°", info.rotation.yaw);
                println!("   Roll:  {:.2}°", info.rotation.roll);
                println!();
                println!("🗺️  关卡: {}", info.level_name);
                println!();
                
                if distance > 0.1 {
                    println!("🏃 移动距离: {:.2} 单位", distance);
                } else {
                    println!("🧍 静止中");
                }
                
                println!();
                println!("----------------------------------------");
                println!("提示: 按 Ctrl+C 退出监控");
                println!("========================================");

                // 更新上次位置
                last_x = info.location.x;
                last_y = info.location.y;
                last_z = info.location.z;
            }
            Err(e) => {
                error_count += 1;
                
                if error_count == 1 {
                    eprintln!("⚠️  读取失败: {:?}", e);
                }
                
                // 如果连续失败太多次，提示用户
                if error_count == 10 {
                    eprintln!("\n⚠️  警告：连续读取失败 {} 次", error_count);
                    eprintln!("可能原因：");
                    eprintln!("1. 游戏已关闭");
                    eprintln!("2. 内存偏移量已变化（游戏更新）");
                    eprintln!("3. 玩家角色尚未加载");
                } else if error_count > 30 {
                    eprintln!("✗ 连续失败超过 30 次，退出程序。");
                    break;
                }
            }
        }

        // 每 500ms 读取一次
        thread::sleep(Duration::from_millis(500));
    }
}

