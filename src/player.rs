/// 玩家坐标读取模块
/// 用于从游戏内存中读取玩家的位置、旋转等信息

use crate::memory::{MemoryReader, MemoryResult, MemoryError};
use crate::ue5_structs::{FVector, FRotator, PlayerInfo, offsets};

/// TArray 结构体 - UE5 中的动态数组
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct TArray<T> {
    data: usize,        // 数据指针
    count: i32,         // 元素数量
    max: i32,           // 最大容量
    _phantom: std::marker::PhantomData<T>,
}

/// 玩家坐标读取器
pub struct PlayerReader {
    memory: MemoryReader,
    gworld_address: usize,
}

impl PlayerReader {
    /// 创建新的玩家读取器
    pub fn new() -> MemoryResult<Self> {
        use tracing::info;
        
        let memory = MemoryReader::current_process()?;
        let gworld_address = memory.get_absolute_address(offsets::GWORLD);

        info!("游戏模块基址: 0x{:X}", memory.base_address());
        info!("GWorld 地址: 0x{:X}", gworld_address);

        Ok(Self {
            memory,
            gworld_address,
        })
    }

    /// 获取 UWorld 指针
    fn get_world_ptr(&self) -> MemoryResult<usize> {
        self.memory.read::<usize>(self.gworld_address)
    }

    /// 获取 PersistentLevel 指针
    #[allow(dead_code)]
    fn get_persistent_level(&self, world_ptr: usize) -> MemoryResult<usize> {
        if world_ptr == 0 {
            return Err(MemoryError::InvalidAddress);
        }
        self.memory.read::<usize>(world_ptr + offsets::world::PERSISTENT_LEVEL)
    }

    /// 获取 OwningGameInstance 指针
    fn get_game_instance(&self, world_ptr: usize) -> MemoryResult<usize> {
        if world_ptr == 0 {
            return Err(MemoryError::InvalidAddress);
        }
        self.memory.read::<usize>(world_ptr + offsets::world::OWNING_GAME_INSTANCE)
    }

    /// 获取 LocalPlayers 数组
    fn get_local_players(&self, game_instance_ptr: usize) -> MemoryResult<TArray<usize>> {
        use tracing::debug;
        
        if game_instance_ptr == 0 {
            return Err(MemoryError::InvalidAddress);
        }
        
        let addr = game_instance_ptr + offsets::game_instance::LOCAL_PLAYERS;
        debug!("读取 LocalPlayers TArray 地址: 0x{:X} (GameInstance+0x{:X})", 
            addr, offsets::game_instance::LOCAL_PLAYERS);
        
        self.memory.read::<TArray<usize>>(addr)
    }

    /// 获取第一个本地玩家
    fn get_first_local_player(&self, game_instance_ptr: usize) -> MemoryResult<usize> {
        use tracing::debug;
        
        let local_players = self.get_local_players(game_instance_ptr)?;
        
        debug!("LocalPlayers TArray: data=0x{:X}, count={}", local_players.data, local_players.count);
        
        if local_players.count > 0 && local_players.data != 0 {
            let player = self.memory.read::<usize>(local_players.data)?;
            debug!("LocalPlayers[0] = 0x{:X}", player);
            Ok(player)
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// 获取玩家控制器
    fn get_player_controller(&self, local_player_ptr: usize) -> MemoryResult<usize> {
        if local_player_ptr == 0 {
            return Err(MemoryError::InvalidAddress);
        }
        self.memory.read::<usize>(local_player_ptr + offsets::local_player::PLAYER_CONTROLLER)
    }

    /// 获取玩家角色 (Pawn)
    fn get_player_character(&self, player_controller_ptr: usize) -> MemoryResult<usize> {
        if player_controller_ptr == 0 {
            return Err(MemoryError::InvalidAddress);
        }
        self.memory.read::<usize>(player_controller_ptr + offsets::player_controller::PAWN)
    }

    /// 从 Actor 获取 RootComponent
    fn get_root_component(&self, actor_ptr: usize) -> MemoryResult<usize> {
        if actor_ptr == 0 {
            return Err(MemoryError::InvalidAddress);
        }
        self.memory.read::<usize>(actor_ptr + offsets::actor::ROOT_COMPONENT)
    }

    /// 从 SceneComponent 读取位置
    fn get_location_from_component(&self, component_ptr: usize) -> MemoryResult<FVector> {
        if component_ptr == 0 {
            return Err(MemoryError::InvalidAddress);
        }
        
        // 直接读取相对位置
        self.memory.read::<FVector>(component_ptr + offsets::scene_component::RELATIVE_LOCATION)
    }

    /// 从 SceneComponent 读取旋转
    fn get_rotation_from_component(&self, component_ptr: usize) -> MemoryResult<FRotator> {
        if component_ptr == 0 {
            return Err(MemoryError::InvalidAddress);
        }
        
        // 读取相对旋转
        self.memory.read::<FRotator>(component_ptr + offsets::scene_component::RELATIVE_ROTATION)
    }

    /// 获取玩家信息
    pub fn get_player_info(&self) -> MemoryResult<PlayerInfo> {
        use tracing::debug;
        
        // 1. 获取 World
        let world_ptr = self.get_world_ptr()?;
        debug!("World 指针: 0x{:X}", world_ptr);
        if world_ptr == 0 {
            return Err(MemoryError::InvalidAddress);
        }

        // 2. 获取 GameInstance
        let game_instance_ptr = self.get_game_instance(world_ptr)?;
        debug!("GameInstance 指针: 0x{:X}", game_instance_ptr);
        if game_instance_ptr == 0 {
            return Err(MemoryError::InvalidAddress);
        }

        // 3. 获取第一个本地玩家
        let local_player_ptr = self.get_first_local_player(game_instance_ptr)?;
        debug!("LocalPlayer 指针: 0x{:X}", local_player_ptr);
        if local_player_ptr == 0 {
            return Err(MemoryError::InvalidAddress);
        }

        // 4. 获取玩家控制器
        let player_controller_ptr = self.get_player_controller(local_player_ptr)?;
        debug!("PlayerController 指针: 0x{:X}", player_controller_ptr);
        if player_controller_ptr == 0 {
            return Err(MemoryError::InvalidAddress);
        }

        // 5. 获取玩家角色
        let character_ptr = self.get_player_character(player_controller_ptr)?;
        debug!("Character 指针: 0x{:X}", character_ptr);
        if character_ptr == 0 {
            return Err(MemoryError::InvalidAddress);
        }

        // 6. 获取 RootComponent
        let root_component = self.get_root_component(character_ptr)?;
        debug!("RootComponent 指针: 0x{:X}", root_component);
        if root_component == 0 {
            return Err(MemoryError::InvalidAddress);
        }

        // 7. 读取位置和旋转
        let location = self.get_location_from_component(root_component)?;
        let rotation = self.get_rotation_from_component(root_component)?;

        // 8. 获取关卡名称 (简化版本)
        let level_name = self.get_level_name(world_ptr).unwrap_or_else(|_| "Unknown".to_string());

        Ok(PlayerInfo::new(location, rotation, level_name))
    }

    /// 获取关卡名称
    fn get_level_name(&self, _world_ptr: usize) -> MemoryResult<String> {
        // 这是一个简化版本，实际实现需要读取 FName
        // 由于 FName 的读取比较复杂，这里暂时返回默认值
        Ok("CurrentLevel".to_string())
    }

    /// 持续监控玩家位置
    pub fn monitor_player_position(&self, interval_ms: u64) {
        use std::thread;
        use std::time::Duration;
        use tracing::{info, warn, error};

        let mut last_x = 0.0;
        let mut last_y = 0.0;
        let mut last_z = 0.0;
        let mut error_count = 0;

        loop {
            match self.get_player_info() {
                Ok(info) => {
                    error_count = 0;
                    
                    // 计算移动距离
                    let dx = info.location.x - last_x;
                    let dy = info.location.y - last_y;
                    let dz = info.location.z - last_z;
                    let distance = (dx * dx + dy * dy + dz * dz).sqrt();
                    
                    // 只在位置变化时输出（减少日志量）
                    if distance > 1.0 {
                        info!(
                            x = %info.location.x,
                            y = %info.location.y,
                            z = %info.location.z,
                            distance = %distance,
                            "Player moved"
                        );
                        
                        last_x = info.location.x;
                        last_y = info.location.y;
                        last_z = info.location.z;
                    }
                }
                Err(e) => {
                    error_count += 1;
                    
                    if error_count == 1 {
                        error!("读取玩家信息失败: {:?}", e);
                    } else if error_count == 10 {
                        warn!("连续读取失败 {} 次", error_count);
                    } else if error_count > 30 {
                        error!("连续失败超过 30 次，停止监控");
                        break;
                    }
                }
            }

            thread::sleep(Duration::from_millis(interval_ms));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_reader_creation() {
        // 注意：这个测试需要游戏运行才能通过
        let result = PlayerReader::new();
        match result {
            Ok(_) => println!("PlayerReader 创建成功"),
            Err(e) => println!("PlayerReader 创建失败: {:?}", e),
        }
    }
}

