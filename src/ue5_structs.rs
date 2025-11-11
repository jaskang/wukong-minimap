/// 虚幻引擎 5 结构体定义
/// 这些结构体对应于 CppSDK 中的 C++ 类

use std::fmt;

/// FVector - UE5 中的三维向量 (使用 double)
/// 对应 SDK 中的 CoreUObject_structs.hpp::FVector
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl FVector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// 计算到另一个向量的距离
    pub fn distance_to(&self, other: &FVector) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// 计算向量长度
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl fmt::Display for FVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

/// FRotator - UE5 中的旋转角度
/// 对应 SDK 中的 CoreUObject_structs.hpp::FRotator
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FRotator {
    pub pitch: f64,  // Y轴旋转
    pub yaw: f64,    // Z轴旋转
    pub roll: f64,   // X轴旋转
}

impl FRotator {
    pub fn new(pitch: f64, yaw: f64, roll: f64) -> Self {
        Self { pitch, yaw, roll }
    }
}

impl fmt::Display for FRotator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(Pitch: {:.2}, Yaw: {:.2}, Roll: {:.2})",
            self.pitch, self.yaw, self.roll
        )
    }
}

/// FTransform - UE5 中的变换（位置、旋转、缩放）
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FTransform {
    pub rotation: FQuat,
    pub translation: FVector,
    pub scale3d: FVector,
}

/// FQuat - 四元数
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FQuat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

/// UObject - 所有 UE 对象的基类
/// 这是一个简化的表示，只包含我们需要的字段
#[repr(C)]
pub struct UObject {
    pub vtable: usize,          // 0x00 - 虚函数表
    pub flags: u32,             // 0x08 - ObjectFlags
    pub index: u32,             // 0x0C - InternalIndex
    pub class_ptr: usize,       // 0x10 - ClassPrivate
    pub name: usize,            // 0x18 - NamePrivate (FName)
    pub outer: usize,           // 0x20 - OuterPrivate
}

/// AActor - UE5 中所有 Actor 的基类
/// 注意：这是一个简化版本，实际的 AActor 包含更多字段
#[repr(C)]
pub struct AActor {
    pub uobject: UObject,
    // ... 其他字段 ...
    // 我们通过偏移量访问所需的数据，不需要完整定义所有字段
}

/// ACharacter - 角色类，继承自 APawn -> AActor -> UObject
/// 位置信息存储在 RootComponent 中
pub struct ACharacter {
    // 实际上我们不需要完整定义，只需要知道如何访问位置数据
}

/// UWorld - 游戏世界
pub struct UWorld {
    // 简化表示
}

/// 玩家信息
#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub location: FVector,
    pub rotation: FRotator,
    pub level_name: String,
}

impl PlayerInfo {
    pub fn new(location: FVector, rotation: FRotator, level_name: String) -> Self {
        Self {
            location,
            rotation,
            level_name,
        }
    }
}

impl fmt::Display for PlayerInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "玩家位置: {}\n旋转: {}\n关卡: {}",
            self.location, self.rotation, self.level_name
        )
    }
}

// 从 offset.md 和 dump.md 获取的重要偏移量
pub mod offsets {
    /// GWorld 全局变量偏移 (从 dump.md)
    pub const GWORLD: usize = 0x1D5E3770;

    /// GUObjectArray 全局变量偏移 (从 offset.md)
    pub const GUOBJECT_ARRAY: usize = 0x1D47ED00;

    /// GNames (FNamePool) 偏移 (从 offset.md 和 dump.md)
    pub const GNAMES: usize = 0x1D3DFCC0;

    /// Actor 相关偏移
    pub mod actor {
        /// RootComponent 偏移 (AActor::RootComponent)
        /// 从 SDK 确认: 0x01A0
        pub const ROOT_COMPONENT: usize = 0x1A0;
    }

    /// SceneComponent 相关偏移
    pub mod scene_component {
        /// 相对位置 (USceneComponent::RelativeLocation)
        /// 从 SDK 确认: 0x0148
        pub const RELATIVE_LOCATION: usize = 0x148;

        /// 相对旋转 (USceneComponent::RelativeRotation)
        /// 从 SDK 确认: 0x0160
        pub const RELATIVE_ROTATION: usize = 0x160;

        /// 组件速度 (USceneComponent::ComponentVelocity)
        /// 从 SDK 确认: 0x0190
        pub const COMPONENT_VELOCITY: usize = 0x190;
    }

    /// UWorld 相关偏移
    pub mod world {
        /// UWorld::PersistentLevel 偏移
        pub const PERSISTENT_LEVEL: usize = 0x30;

        /// UWorld::OwningGameInstance 偏移 (从 SDK: 0x0190)
        pub const OWNING_GAME_INSTANCE: usize = 0x190;
    }

    /// ULevel 相关偏移
    pub mod level {
        /// ULevel::Actors 数组偏移 (从 dump.md: Off::InSDK::ULevel::Actors: 0x98)
        pub const ACTORS: usize = 0x98;
    }

    /// UGameInstance 相关偏移
    pub mod game_instance {
        /// UGameInstance::LocalPlayers 数组偏移
        pub const LOCAL_PLAYERS: usize = 0x38;
    }

    /// ULocalPlayer 相关偏移
    pub mod local_player {
        /// ULocalPlayer::PlayerController 偏移
        pub const PLAYER_CONTROLLER: usize = 0x30;
    }

    /// APlayerController 相关偏移
    pub mod player_controller {
        /// AController::Pawn (当前控制的角色) 偏移 (从 SDK: 0x02C8)
        pub const PAWN: usize = 0x2C8;
    }
}

