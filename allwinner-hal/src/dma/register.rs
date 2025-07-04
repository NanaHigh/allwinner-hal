use volatile_register::{RO, RW};

/// 直接内存访问控制器（DMAC）通道寄存器
#[repr(C)]
#[derive(Debug, Default)]
pub struct ChannelRegisterBlock {
    /// DMAC 通道使能寄存器
    pub enable: ChannelEnable,
    /// DMAC 通道暂停寄存器
    pub pause: ChannelPause,
    /// DMAC 通道起始地址寄存器
    pub start_addr: ChannelStartAddr,
    /// DMAC 通道配置寄存器
    pub config: ChannelConfig,
    /// DMAC 通道当前源地址寄存器
    pub current_src_addr: ChannelCurrentSrcAddr,
    /// DMAC 通道当前目标地址寄存器
    pub current_destination: ChannelCurrentDestAddr,
    /// DMAC 通道剩余字节计数寄存器
    pub byte_counter_left: ChannelByteCounterLeft,
    /// DMAC 通道参数寄存器
    pub parameter: ChannelParameter,
    _reserved0: [u8; 0x8],
    /// DMAC 模式寄存器
    pub mode: ChannelMode,
    /// DMAC 前描述符地址寄存器
    pub former_desc_addr: ChannelFormerDescAddr,
    /// DMAC 包数量寄存器
    pub package_num: ChannelPackageNum,
    _reserved1: [u8; 0xC],
}

/// DMAC 通道使能寄存器
#[repr(C)]
pub struct ChannelEnable {
    pub dma_en: RW<u32>, // DMA使能位
}

impl ChannelEnable {
    // 获取 `dma_en` 位的值
    #[inline]
    pub fn dma_en(&self) -> bool {
        (self.dma_en.read() & 0x01) != 0
    }

    // 设置 `dma_en` 位的值
    #[inline]
    pub fn set_dma_en(&mut self, value: bool) {
        let mut reg = self.dma_en.read();
        if value {
            reg |= 0x01;  // 设置使能位
        } else {
            reg &= !0x01; // 清除使能位
        }
        self.dma_en.write(reg);
    }
}

/// DMAC 通道暂停寄存器
#[repr(C)]
pub struct ChannelPause {
    pub dma_pause: RW<u32>, // DMA暂停位
}

impl ChannelPause {
    // 获取 `dma_pause` 位的值
    #[inline]
    pub fn dma_pause(&self) -> bool {
        (self.dma_pause.read() & 0x01) != 0
    }

    // 设置 `dma_pause` 位的值
    #[inline]
    pub fn set_dma_pause(&mut self, value: bool) {
        let mut reg = self.dma_pause.read();
        if value {
            reg |= 0x01;  // 设置暂停位
        } else {
            reg &= !0x01; // 清除暂停位
        }
        self.dma_pause.write(reg);
    }
}

/// DMAC 通道起始地址寄存器
#[repr(C)]
pub struct ChannelStartAddr {
    pub dma_desc_addr: RW<u32>, // DMA描述符地址
}

impl ChannelStartAddr {
    // 设置描述符地址
    #[inline]
    pub fn set_desc_addr(&mut self, addr: u32) {
        self.dma_desc_addr.write(addr);
    }
}

/// DMAC 通道配置寄存器
#[repr(C)]
pub struct ChannelConfig {
    pub bmode_sel: RW<u32>, // 块模式选择位
    pub dma_dest_data_width: RW<u32>, // 目标数据宽度
    pub dma_addr_mode: RW<u32>, // 地址模式
    pub dma_src_data_width: RW<u32>, // 源数据宽度
    pub dma_src_addr_mode: RW<u32>, // 源地址模式
}

impl ChannelConfig {
    // 获取源数据宽度
    #[inline]
    pub fn dma_src_data_width(&self) -> u32 {
        self.dma_src_data_width.read()
    }

    // 设置源数据宽度
    #[inline]
    pub fn set_dma_src_data_width(&mut self, width: u32) {
        self.dma_src_data_width.write(width);
    }
}

/// DMAC 通道当前源地址寄存器
#[repr(C)]
pub struct ChannelCurrentSrcAddr {
    pub dma_cur_src: RO<u32>, // 当前源地址
}

impl ChannelCurrentSrcAddr {
    // 获取当前源地址
    #[inline]
    pub fn dma_cur_src(&self) -> u32 {
        self.dma_cur_src.read()
    }
}

/// DMAC 通道当前目标地址寄存器
#[repr(C)]
pub struct ChannelCurrentDestAddr {
    pub dma_cur_dest: RO<u32>, // 当前目标地址
}

impl ChannelCurrentDestAddr {
    // 获取当前目标地址
    #[inline]
    pub fn dma_cur_dest(&self) -> u32 {
        self.dma_cur_dest.read()
    }
}

/// DMAC 通道剩余字节计数寄存器
#[repr(C)]
pub struct ChannelByteCounterLeft {
    pub dma_bcnt_left: RO<u32>, // 剩余字节计数
}

impl ChannelByteCounterLeft {
    // 获取剩余字节计数
    #[inline]
    pub fn dma_bcnt_left(&self) -> u32 {
        self.dma_bcnt_left.read()
    }
}

/// DMAC 通道参数寄存器
#[repr(C)]
pub struct ChannelParameter {
    pub wait_cycles: RW<u32>, // 等待时钟周期数
}

impl ChannelParameter {
    // 设置等待时钟周期
    #[inline]
    pub fn set_wait_cycles(&mut self, cycles: u32) {
        self.wait_cycles.write(cycles);
    }
}

/// DMAC 模式寄存器
#[repr(C)]
pub struct ChannelMode {
    pub dma_src_mode: RW<u32>, // 源通信模式
    pub dma_dst_mode: RW<u32>, // 目标通信模式
}

impl ChannelMode {
    // 获取源通信模式
    #[inline]
    pub fn dma_src_mode(&self) -> u32 {
        self.dma_src_mode.read()
    }

    // 设置源通信模式
    #[inline]
    pub fn set_dma_src_mode(&mut self, mode: u32) {
        self.dma_src_mode.write(mode);
    }
}

/// DMAC 前描述符地址寄存器
#[repr(C)]
pub struct ChannelFormerDescAddr {
    pub dma_fdesc_addr: RO<u32>, // 前描述符地址
}

impl ChannelFormerDescAddr {
    // 获取前描述符地址
    #[inline]
    pub fn dma_fdesc_addr(&self) -> u32 {
        self.dma_fdesc_addr.read()
    }
}

/// DMAC 包数量寄存器
#[repr(C)]
pub struct ChannelPackageNum {
    pub dma_pkg_num: RO<u32>, // 包数量
}

impl ChannelPackageNum {
    // 获取包数量
    #[inline]
    pub fn dma_pkg_num(&self) -> u32 {
        self.dma_pkg_num.read()
    }
}

