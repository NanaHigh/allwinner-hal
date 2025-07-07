use volatile_register::{RO, RW};

/// Direct Memory Access Controller registers.
#[repr(C)]
pub struct RegisterBlock {
    /// DMAC IRQ Enable Register 0.
    pub irq_enable0: RW<u32>,
    /// DMAC IRQ Enable Register 1.
    pub irq_enable1: RW<u32>,
    _reserved0: [u8; 0x8],
    /// DMAC IRQ Pending Register 0.
    pub irq_pending0: RW<u32>,
    /// DMAC IRQ Pending Register 1.
    pub irq_pending1: RW<u32>,
    _reserved1: [u8; 0x10],
    /// DMAC Auto Gating Register.
    pub auto_gating: RW<u32>,
    _reserved2: [u8; 0x4],
    /// DMAC Status Register.
    pub status: RO<u32>,
    _reserved3: [u8; 0xCC],
    /// DMAC Channels' Registers.
    pub channels: [ChannelRegisterBlock; 16],
}

/// DMAC channel registers.
#[repr(C)]
pub struct ChannelRegisterBlock {
    /// DMAC Channel Enable Register.
    pub enable: ChannelEnableRegister,
    /// DMAC Channel Pause Register.
    pub pause: ChannelPauseRegister,
    /// DMAC Channel Start Address Register.
    pub start_addr: ChannelStartAddrRegister,
    /// DMAC Channel Configuration Register.
    pub config: ChannelConfigRegister,
    /// DMAC Channel Current Source Register.
    pub current_src_addr: ChannelCurrentSrcAddrRegister,
    /// DMAC Channel Current Destination Register.
    pub current_destination: ChannelCurrentDestAddrRegister,
    /// DMAC Channel Byte Counter Left Register.
    pub byte_counter_left: ChannelByteCounterLeftRegister,
    /// DMAC Channel Parameter Register.
    pub parameter: ChannelParameterRegister,
    _reserved0: [u8; 0x8],
    /// DMAC Mode Register.
    pub mode: ChannelModeRegister,
    /// DMAC Former Descriptor Address Register.
    pub former_desc_addr: ChannelFormerDescAddrRegister,
    /// DMAC Package Number Register.
    pub package_num: ChannelPackageNumRegister,
    _reserved1: [u8; 0xC],
}

impl ChannelEnableRegister {
    // DMA Channel Enable (Bit 0)
    pub const DMA_EN: u32 = 0x1 << 0;

    /// Check if DMA channel is enabled.
    #[inline]
    pub const fn is_dma_enabled(self) -> bool {
        (self.0 & Self::DMA_EN) != 0
    }

    /// Enable DMA channel (set bit 0).
    #[inline]
    pub fn enable_dma(&mut self) {
        self.0 |= Self::DMA_EN;
    }

    /// Disable DMA channel (clear bit 0).
    #[inline]
    pub fn disable_dma(&mut self) {
        self.0 &= !Self::DMA_EN;
    }
}

impl ChannelPauseRegister {
    // DMA Pause bit (Bit 0)
    pub const DMA_PAUSE: u32 = 0x1 << 0;

    /// Check if DMA transfer is paused.
    #[inline]
    pub const fn if_dma_pause(self) -> bool {
        (self.0 & Self::DMA_PAUSE) != 0
    }

    /// Pause DMA transfer (set bit 0).
    #[inline]
    pub fn pause_dma(&mut self) {
        self.0 |= Self::DMA_PAUSE;
    }

    /// Resume DMA transfer (clear bit 0).
    #[inline]
    pub fn resume_dma(&mut self) {
        self.0 &= !Self::DMA_PAUSE;
    }
}

impl ChannelStartAddrRegister {
    // DMA descriptor address lower 30 bits (Bits [31:2])
    pub const DMA_DESC_ADDR: u32 = 0x3FFFFFFF << 2;

    // DMA descriptor address higher 2 bits (Bits [1:0])
    pub const DMA_DESC_HIGH_ADDR: u32 = 0x3 << 0;

    /// Get lower 30 bits of the DMA descriptor address.
    #[inline]
    pub const fn dma_desc_addr(self) -> u32 {
        (self.0 & Self::DMA_DESC_ADDR) >> 2
    }

    /// Get higher 2 bits of the DMA descriptor address.
    #[inline]
    pub const fn dma_desc_high_addr(self) -> u32 {
        (self.0 & Self::DMA_DESC_HIGH_ADDR) >> 0
    }

    /// Get full 32-bit DMA descriptor address with alignment.
    #[inline]
    pub const fn full_dma_desc_addr(self) -> u32 {
        (self.dma_desc_high_addr() << 30) | (self.dma_desc_addr() << 2)
    }
}

impl ChannelConfigRegister {
    // BMODE_SEL (Bit 30)
    pub const BMODE_SEL: u32 = 0x1 << 30;

    // DMA_DEST_DATA_WIDTH (Bits [26:25])
    pub const DMA_DEST_DATA_WIDTH: u32 = 0x3 << 25;

    // DMA_ADDR_MODE (Bit 24)
    pub const DMA_ADDR_MODE: u32 = 0x1 << 24;

    // DMA_DEST_BLOCK_SIZE (Bits [23:22])
    pub const DMA_DEST_BLOCK_SIZE: u32 = 0x3 << 22;

    // DMA_DEST_DRQ_TYPE (Bits [21:16])
    pub const DMA_DEST_DRQ_TYPE: u32 = 0x3F << 16;

    // DMA_SRC_DATA_WIDTH (Bits [10:9])
    pub const DMA_SRC_DATA_WIDTH: u32 = 0x3 << 9;

    // DMA_SRC_ADDR_MODE (Bit 8)
    pub const DMA_SRC_ADDR_MODE: u32 = 0x1 << 8;

    // DMA_SRC_BLOCK_SIZE (Bits [7:6])
    pub const DMA_SRC_BLOCK_SIZE: u32 = 0x3 << 6;

    // DMA_SRC_DRQ_TYPE (Bits [5:0])
    pub const DMA_SRC_DRQ_TYPE: u32 = 0x3F << 0;

    /// Get the BMODE_SEL bit.
    #[inline]
    pub const fn bmode_sel(self) -> u32 {
        (self.0 & Self::BMODE_SEL) >> 30
    }

    /// Get the DMA_DEST_DATA_WIDTH bits.
    #[inline]
    pub const fn dma_dest_data_width(self) -> u32 {
        (self.0 & Self::DMA_DEST_DATA_WIDTH) >> 25
    }

    /// Get the DMA_ADDR_MODE bit.
    #[inline]
    pub const fn dma_addr_mode(self) -> u32 {
        (self.0 & Self::DMA_ADDR_MODE) >> 24
    }

    /// Get the DMA_DEST_BLOCK_SIZE bits.
    #[inline]
    pub const fn dma_dest_block_size(self) -> u32 {
        (self.0 & Self::DMA_DEST_BLOCK_SIZE) >> 22
    }

    /// Get the DMA_DEST_DRQ_TYPE bits.
    #[inline]
    pub const fn dma_dest_drq_type(self) -> u32 {
        (self.0 & Self::DMA_DEST_DRQ_TYPE) >> 16
    }

    /// Get the DMA_SRC_DATA_WIDTH bits.
    #[inline]
    pub const fn dma_src_data_width(self) -> u32 {
        (self.0 & Self::DMA_SRC_DATA_WIDTH) >> 9
    }

    /// Get the DMA_SRC_ADDR_MODE bit.
    #[inline]
    pub const fn dma_src_addr_mode(self) -> u32 {
        (self.0 & Self::DMA_SRC_ADDR_MODE) >> 8
    }

    /// Get the DMA_SRC_BLOCK_SIZE bits.
    #[inline]
    pub const fn dma_src_block_size(self) -> u32 {
        (self.0 & Self::DMA_SRC_BLOCK_SIZE) >> 6
    }

    /// Get the DMA_SRC_DRQ_TYPE bits.
    #[inline]
    pub const fn dma_src_drq_type(self) -> u32 {
        (self.0 & Self::DMA_SRC_DRQ_TYPE) >> 0
    }
}

impl ChannelCurrentSrcAddrRegister {
    // DMA Current Source Address (Bits [31:0])
    pub const DMA_CUR_SRC: u32 = 0xFFFFFFFF;

    /// Get the DMA current source address.
    #[inline]
    pub const fn dma_cur_src(self) -> u32 {
        self.0 & Self::DMA_CUR_SRC
    }
}

impl ChannelCurrentDestAddrRegister {
    // DMA Current Destination Address (Bits [31:0])
    pub const DMA_CUR_DEST: u32 = 0xFFFFFFFF;

    /// Get the DMA current destination address.
    #[inline]
    pub const fn dma_cur_dest(self) -> u32 {
        self.0 & Self::DMA_CUR_DEST
    }
}

impl ChannelByteCounterLeftRegister {
    // DMA Byte Counter Left (Bits [24:0])
    pub const DMA_BCNT_LEFT: u32 = 0xFFFFFF;

    /// Get the DMA byte counter left value.
    #[inline]
    pub const fn dma_bcnt_left(self) -> u32 {
        self.0 & Self::DMA_BCNT_LEFT
    }
}

impl ChannelParameterRegister {
    // WAIT_CYC (Bits [7:0])
    pub const WAIT_CYC: u32 = 0xFF << 0;

    /// Get the WAIT_CYC value.
    #[inline]
    pub const fn wait_cyc(self) -> u32 {
        self.0 & Self::WAIT_CYC
    }
}

impl ChannelModeRegister {
    // DMA_DST_MODE (Bit 3)
    pub const DMA_DST_MODE: u32 = 0x1 << 3;

    // DMA_SRC_MODE (Bit 2)
    pub const DMA_SRC_MODE: u32 = 0x1 << 2;

    /// Get the DMA destination communication mode.
    #[inline]
    pub const fn dma_dst_mode(self) -> u32 {
        (self.0 & Self::DMA_DST_MODE) >> 3
    }

    /// Get the DMA source communication mode.
    #[inline]
    pub const fn dma_src_mode(self) -> u32 {
        (self.0 & Self::DMA_SRC_MODE) >> 2
    }

    /// Set the DMA destination communication mode.
    #[inline]
    pub fn set_dma_dst_mode(&mut self, enable: bool) {
        if enable {
            self.0 |= Self::DMA_DST_MODE;
        } else {
            self.0 &= !Self::DMA_DST_MODE;
        }
    }
}

impl ChannelFormerDescAddrRegister {
    // DMA Former Descriptor Address (Bits [31:0])
    pub const DMA_FDESC_ADDR: u32 = 0xFFFFFFFF;

    /// Get the DMA former descriptor address.
    #[inline]
    pub const fn dma_fdesc_addr(self) -> u32 {
        self.0 & Self::DMA_FDESC_ADDR
    }
}

impl ChannelPackageNumRegister {
    // DMA Package Number (Bits [31:0])
    pub const DMA_PKG_NUM: u32 = 0xFFFFFFFF;

    /// Get the DMA package number.
    #[inline]
    pub const fn dma_pkg_num(self) -> u32 {
        self.0 & Self::DMA_PKG_NUM
    }
}

#[cfg(test)]
mod tests {
    use super::{ChannelRegisterBlock, RegisterBlock};
    use core::mem::{offset_of, size_of};
    #[test]
    fn offset_registerblock() {
        assert_eq!(offset_of!(RegisterBlock, irq_enable0), 0x0);
        assert_eq!(offset_of!(RegisterBlock, irq_enable1), 0x4);
        assert_eq!(offset_of!(RegisterBlock, irq_pending0), 0x10);
        assert_eq!(offset_of!(RegisterBlock, irq_pending1), 0x14);
        assert_eq!(offset_of!(RegisterBlock, auto_gating), 0x28);
        assert_eq!(offset_of!(RegisterBlock, status), 0x30);
        assert_eq!(offset_of!(RegisterBlock, channels), 0x100);
    }

    #[test]
    fn offset_channel_registerblock() {
        assert_eq!(offset_of!(ChannelRegisterBlock, enable), 0x0);
        assert_eq!(offset_of!(ChannelRegisterBlock, pause), 0x4);
        assert_eq!(offset_of!(ChannelRegisterBlock, start_addr), 0x08);
        assert_eq!(offset_of!(ChannelRegisterBlock, config), 0xC);
        assert_eq!(offset_of!(ChannelRegisterBlock, current_src_addr), 0x10);
        assert_eq!(offset_of!(ChannelRegisterBlock, current_destination), 0x14);
        assert_eq!(offset_of!(ChannelRegisterBlock, byte_counter_left), 0x18);
        assert_eq!(offset_of!(ChannelRegisterBlock, parameter), 0x1C);
        assert_eq!(offset_of!(ChannelRegisterBlock, mode), 0x28);
        assert_eq!(offset_of!(ChannelRegisterBlock, former_desc_addr), 0x2C);
        assert_eq!(offset_of!(ChannelRegisterBlock, package_num), 0x30);
        assert_eq!(size_of::<ChannelRegisterBlock>(), 0x40);
    }
}
