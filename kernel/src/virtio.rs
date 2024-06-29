use core::ptr;


const SECTOR_SIZE: usize = 512;
const VIRTQ_ENTRY_NUM: usize = 16;
const VIRTIO_DEVICE_BLK: usize = 2;
const VIRTIO_BLK_PADDR: usize = 0x10001000;

const VIRTIO_REG_MAGIC: u32 = 0x00;
const VIRTIO_REG_VERSION: u32 = 0x04;
const VIRTIO_REG_DEVICE_ID: u32 = 0x08;
const VIRTIO_REG_QUEUE_SEL: u32 = 0x30;
const VIRTIO_REG_QUEUE_NUM_MAX: u32 = 0x34;
const VIRTIO_REG_QUEUE_NUM: u32 = 0x38;
const VIRTIO_REG_QUEUE_ALIGN: u32 = 0x3c;
const VIRTIO_REG_QUEUE_PFN: u32 = 0x40;
const VIRTIO_REG_QUEUE_READY: u32 = 0x44;
const VIRTIO_REG_QUEUE_NOTIFY: u32 = 0x50;
const VIRTIO_REG_DEVICE_STATUS: u32 = 0x70;
const VIRTIO_REG_DEVICE_CONFIG: u32 = 0x100;

const VIRTIO_STATUS_ACK: u8 = 1;
const VIRTIO_STATUS_DRIVER: u8 = 2;
const VIRTIO_STATUS_DRIVER_OK: u8 = 4;
const VIRTIO_STATUS_FEAT_OK: u8 = 8;

const VIRTQ_DESC_F_NEXT: u16 = 1;
const VIRTQ_DESC_F_WRITE: u16 = 2;
const VIRTQ_AVAIL_F_NO_INTERRUPT: u16 = 1;

const VIRTIO_BLK_T_IN: u32 = 0;
const VIRTIO_BLK_T_OUT: u32 = 1;

#[repr(packed)]
struct VirtqDesc {
    addr: u64,
    len: u32,
    flags: u16,
    next: u16,
}

#[repr(packed)]
struct VirtqAvail {
    flags: u16,
    idx: u16,
    ring: [u16; VIRTQ_ENTRY_NUM],
}

#[repr(packed)]
struct VirtUsedElem {
    id: u32,
    len: u32,
}

#[repr(packed)]
struct VirtqUsed {
    flags: u16,
    idx: u16,
    ring: [VirtUsedElem; VIRTQ_ENTRY_NUM],
}

#[repr(packed)]
struct VirtioVirtq {
    desc: [VirtqDesc; VIRTQ_ENTRY_NUM],
    avail: VirtqAvail,
    used: VirtqUsed,
    queue_index: i32,
    used_index: *mut u16,
    last_used_index: u16,
}

#[repr(packed)]
struct VirtioBlkReq {
    r#type: u32,
    reserved: u32,
    sector: u64,
    data: [u8; SECTOR_SIZE],
    status: u8,
}

unsafe fn virtio_reg_read32(offset: usize) -> u32 {
    ptr::read_volatile((VIRTIO_BLK_PADDR + offset) as *const u32)
}

unsafe fn virtio_reg_read64(offset: usize) -> u64 {
    ptr::read_volatile((VIRTIO_BLK_PADDR + offset) as *const u64)
}

unsafe fn virtio_reg_write32(offset: usize, value: u32)  {
    ptr::write_volatile((VIRTIO_BLK_PADDR + offset) as *mut u32, value)
}

unsafe fn virtio_reg_fetch_and_or32(offset: usize, value: u32)  {
    virtio_reg_write32(offset, virtio_reg_read32(offset) | value)
}