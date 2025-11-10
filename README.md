```c++
SDK::UWorld *World = SDK::UWorld::GetWorld();
SDK::UGameplayStatics *GameplayStatics = SDK::UGameplayStatics::GetDefaultObj();
SDK::ACharacter *playerCharacter = SDK::UBGUFunctionLibrary::GetPlayerCharacter(World);
SDK::APlayerController *playerController = GameplayStatics->GetPlayerController(World, 0);
// 获取当前关卡名称
std::string currentLevelName(GameplayStatics->GetCurrentLevelName(World, false).ToString());
strncpy_s(info.level, sizeof(info.level), currentLevelName.c_str(), _TRUNCATE);
// 获取位置和角度
SDK::FVector Location = playerCharacter->K2_GetActorLocation();
SDK::FRotator Rotator = playerCharacter->K2_GetActorRotation();
```
