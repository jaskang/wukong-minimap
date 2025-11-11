use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::core::PCSTR;

/// 内存读取错误
#[derive(Debug)]
pub enum MemoryError {
    ModuleNotFound(String),
    InvalidAddress,
    ReadFailed,
}

pub type MemoryResult<T> = Result<T, MemoryError>;

/// 内存读取器（DLL 注入模式 - 在进程内部直接读取）
pub struct MemoryReader {
    base_address: usize,
}

impl MemoryReader {
    /// 创建内存读取器（在注入的 DLL 内部使用）
    pub fn current_process() -> MemoryResult<Self> {
        unsafe {
            // 获取游戏主模块基址（当前进程的主模块）
            let base_address = Self::get_module_base_address()?;

            Ok(Self {
                base_address,
            })
        }
    }

    /// 获取模块基址（使用 GetModuleHandleA 获取主模块）
    unsafe fn get_module_base_address() -> MemoryResult<usize> {
        // 传入 null 获取当前进程的主模块（b1-Win64-Shipping.exe）
        let module_handle = GetModuleHandleA(PCSTR::null())
            .map_err(|_| MemoryError::ModuleNotFound("主模块".to_string()))?;
        
        Ok(module_handle.0 as usize)
    }

    /// 读取内存数据（直接指针访问，因为我们在同一进程内）
    pub fn read<T: Copy>(&self, address: usize) -> MemoryResult<T> {
        if address == 0 {
            return Err(MemoryError::InvalidAddress);
        }

        unsafe {
            // 在进程内部，直接通过指针读取
            let ptr = address as *const T;
            
            // 直接读取，不做复杂检查（在同一进程内是安全的）
            Ok(std::ptr::read_volatile(ptr))
        }
    }

    /// 读取指针链
    pub fn read_pointer_chain(&self, base: usize, offsets: &[usize]) -> MemoryResult<usize> {
        let mut address = base;

        for (i, &offset) in offsets.iter().enumerate() {
            if i < offsets.len() - 1 {
                // 不是最后一个偏移，需要解引用指针
                address = self.read::<usize>(address)?;
                if address == 0 {
                    return Err(MemoryError::InvalidAddress);
                }
                address += offset;
            } else {
                // 最后一个偏移，直接加上
                address += offset;
            }
        }

        Ok(address)
    }

    /// 从相对地址计算绝对地址
    pub fn get_absolute_address(&self, relative_offset: usize) -> usize {
        self.base_address + relative_offset
    }

    /// 获取基址
    pub fn base_address(&self) -> usize {
        self.base_address
    }

    /// 读取字符串（UTF-8）
    pub fn read_string(&self, address: usize, max_length: usize) -> MemoryResult<String> {
        if address == 0 {
            return Err(MemoryError::InvalidAddress);
        }

        unsafe {
            let ptr = address as *const u8;
            let mut bytes = Vec::new();
            
            for i in 0..max_length {
                let byte = std::ptr::read_volatile(ptr.add(i));
                if byte == 0 {
                    break;
                }
                bytes.push(byte);
            }

            Ok(String::from_utf8_lossy(&bytes).to_string())
        }
    }
}

