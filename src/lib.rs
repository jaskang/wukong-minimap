// 模块声明
pub mod memory;
pub mod ue5_structs;
pub mod player;

// 重新导出主要类型
pub use memory::{MemoryReader, MemoryError, MemoryResult};
pub use ue5_structs::{FVector, FRotator, PlayerInfo, offsets};
pub use player::PlayerReader;

use windows::Win32::Foundation::HINSTANCE;
use std::ffi;
use tracing::{info, error};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(_hmodule: HINSTANCE, reason: u32, _: *mut ffi::c_void) {
    const DLL_PROCESS_ATTACH: u32 = 1;
    
    if reason == DLL_PROCESS_ATTACH {
        // 在 DLL 加载时启动一个线程来监控玩家位置
        std::thread::spawn(move || {
            // 初始化 tracing (同时输出到控制台和文件)
            if let Err(e) = init_tracing() {
                eprintln!("初始化 tracing 失败: {}", e);
                return;
            }

            info!("=== Wukong Player Position Reader v1.4 ===");
            info!("DLL 注入成功");
            info!("等待 5 秒让游戏初始化...");
            
            // 等待一段时间让游戏初始化
            std::thread::sleep(std::time::Duration::from_secs(5));
            
            match PlayerReader::new() {
                Ok(reader) => {
                    info!("玩家坐标读取器初始化成功");
                    info!("开始监控玩家位置...");
                    
                    // 每秒读取一次玩家位置
                    reader.monitor_player_position(1000);
                }
                Err(e) => {
                    error!("玩家坐标读取器初始化失败: {:?}", e);
                }
            }
        });
    }
}

/// 初始化 tracing 日志系统（同时输出到控制台和文件）
fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter, Layer};
    use tracing_appender::rolling;
    use windows::Win32::System::Console::AllocConsole;
    
    // 创建控制台窗口
    unsafe {
        let _ = AllocConsole();
    }
    
    // 创建日志文件
    let file_appender = rolling::never(".", "wukong_minimap.log");
    
    // 文件输出层（无 ANSI 颜色）
    let file_layer = fmt::layer()
        .with_writer(file_appender)
        .with_ansi(false)
        .with_target(false)
        .with_filter(EnvFilter::from_default_env()
            .add_directive(tracing::Level::INFO.into()));
    
    // 控制台输出层（显示 DEBUG 级别）
    let console_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_ansi(false)  // Windows 控制台不支持 ANSI
        .with_target(false)
        .with_filter(EnvFilter::from_default_env()
            .add_directive(tracing::Level::DEBUG.into()));
    
    tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .init();
    
    Ok(())
}

/// 获取玩家当前位置（供外部调用）
#[no_mangle]
pub extern "C" fn get_player_location(x: *mut f64, y: *mut f64, z: *mut f64) -> bool {
    if x.is_null() || y.is_null() || z.is_null() {
        return false;
    }

    match PlayerReader::new() {
        Ok(reader) => match reader.get_player_info() {
            Ok(info) => unsafe {
                *x = info.location.x;
                *y = info.location.y;
                *z = info.location.z;
                true
            },
            Err(_) => false,
        },
        Err(_) => false,
    }
}
