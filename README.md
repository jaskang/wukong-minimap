# 黑神话悟空 - 玩家坐标读取器

使用 Rust 实现的 DLL 注入模块，读取游戏中玩家的实时坐标。

## 功能

- 读取玩家 3D 坐标 (X, Y, Z)
- 读取玩家旋转角度 (Pitch, Yaw, Roll)
- 自动弹出控制台窗口，实时显示
- 同时输出到日志文件

## 快速开始

### 1. 编译

```bash
cargo build --release
```

生成文件: `target\release\wukong_minimap.dll` (216 KB)

### 2. 注入

使用 DLL 注入工具（如 Extreme Injector）将 DLL 注入到 `b1-Win64-Shipping.exe` 进程。

### 3. 查看

注入后会自动弹出**控制台窗口**，实时显示玩家坐标：

```
============================================================
  黑神话悟空 - 玩家坐标读取器 v1.4.0
============================================================
[✓] DLL 注入成功！
[INFO] 控制台窗口已创建
============================================================

[#0001] 位置: (12345.67, 23456.78, 345.89) | 旋转: (0.00, 90.00, 0.00) | 移动: 123.45
```

同时也会在游戏目录创建日志文件：
```
<游戏目录>\b1\Binaries\Win64\wukong_minimap.log
```

## 技术说明

### 内存读取路径

```
GWorld (0x1D5E3770)
  └─> UWorld::OwningGameInstance
       └─> UGameInstance::LocalPlayers[0]
            └─> ULocalPlayer::PlayerController
                 └─> APlayerController::Pawn
                      └─> AActor::RootComponent
                           └─> USceneComponent::ComponentToWorld
                                └─> FTransform::Translation (玩家位置)
```

### 关键偏移量

来自 dump-7 生成的 CppSDK：

- `GWorld`: `0x1D5E3770`
- `OwningGameInstance`: `0x1C0`
- `LocalPlayers`: `0x38`
- `PlayerController`: `0x30`
- `Pawn`: `0x2A8`
- `RootComponent`: `0x198`

### C 接口

DLL 导出了 C 接口，可供其他语言调用：

```c
extern "C" bool get_player_location(double* x, double* y, double* z);
```

## 项目结构

```
src/
├── lib.rs           # DLL 入口
├── memory.rs        # 内存读取（DLL 注入模式）
├── ue5_structs.rs   # UE5 结构体和偏移量
├── player.rs        # 玩家坐标读取
├── console.rs       # 控制台窗口
└── logger.rs        # 日志文件

5.0.0-0+++UE5+Release-5.0-b1/
└── CppSDK/          # Dumper-7 生成的 SDK
```

## 注意事项

- 仅供学习研究使用
- 游戏版本: 5.0.0-0+++UE5+Release-5.0
- 游戏更新后偏移量可能失效，需要重新生成
- 单机使用，避免在联机模式下使用

## 故障排除

**控制台窗口没出现？**
- 检查 DLL 是否成功注入
- 查看游戏目录的日志文件

**显示 InvalidAddress 错误？**
- 等待游戏完全加载
- 确保在游戏场景中（不是主菜单）
- 游戏版本可能不匹配

## 许可证

本项目基于 Apache-2.0 许可证，仅供学习研究使用。

---

**原 C++ 参考代码:**

```c++
SDK::UWorld *World = SDK::UWorld::GetWorld();
SDK::ACharacter *playerCharacter = SDK::UBGUFunctionLibrary::GetPlayerCharacter(World);
SDK::FVector Location = playerCharacter->K2_GetActorLocation();
SDK::FRotator Rotator = playerCharacter->K2_GetActorRotation();
```
