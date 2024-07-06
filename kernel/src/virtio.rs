use core::{
    ffi::c_void,
    mem::size_of,
    ptr::{self, read_unaligned},
    sync::atomic::{compiler_fence, fence, Ordering},
};

use common::{print, println};

use crate::{mem::paddr_t, MEM_MANAGER, PAGE_SIZE};

pub const SECTOR_SIZE: usize = 512;
pub const VIRTQ_ENTRY_NUM: usize = 16;
pub const VIRTIO_DEVICE_BLK: usize = 2;
pub const VIRTIO_BLK_PADDR: usize = 0x10001000;

const VIRTIO_REG_MAGIC: usize = 0x00;
const VIRTIO_REG_VERSION: usize = 0x04;
const VIRTIO_REG_DEVICE_ID: usize = 0x08;
const VIRTIO_REG_QUEUE_SEL: usize = 0x30;
const VIRTIO_REG_QUEUE_NUM_MAX: usize = 0x34;
const VIRTIO_REG_QUEUE_NUM: usize = 0x38;
const VIRTIO_REG_QUEUE_ALIGN: usize = 0x3c;
const VIRTIO_REG_QUEUE_PFN: usize = 0x40;
const VIRTIO_REG_QUEUE_READY: usize = 0x44;
const VIRTIO_REG_QUEUE_NOTIFY: usize = 0x50;
const VIRTIO_REG_DEVICE_STATUS: usize = 0x70;
const VIRTIO_REG_DEVICE_CONFIG: usize = 0x100;

const VIRTIO_STATUS_ACK: u32 = 1;
const VIRTIO_STATUS_DRIVER: u32 = 2;
const VIRTIO_STATUS_DRIVER_OK: u32 = 4;
const VIRTIO_STATUS_FEAT_OK: u32 = 8;

const VIRTQ_DESC_F_NEXT: u16 = 1;
const VIRTQ_DESC_F_WRITE: u16 = 2;
const VIRTQ_AVAIL_F_NO_INTERRUPT: u16 = 1;

const VIRTIO_BLK_T_IN: usize = 0;
const VIRTIO_BLK_T_OUT: usize = 1;

// #[repr(packed)]
#[repr(align(4096))]
struct VirtqDesc {
    addr: u64,
    len: u32,
    flags: u16,
    next: u16,
}

// #[repr(packed)]
#[repr(align(4096))]
struct VirtqAvail {
    flags: u16,
    idx: u16,
    ring: [u16; VIRTQ_ENTRY_NUM],
}

#[repr(packed)]
// #[repr(align(4096))]
struct VirtUsedElem {
    id: u32,
    len: u32,
}

// #[repr(packed)]
#[repr(align(4096))]
struct VirtqUsed {
    flags: u16,
    idx: u16,
    ring: [VirtUsedElem; VIRTQ_ENTRY_NUM],
}

// ページの先頭に割り当てられる
#[repr(align(4096))]
struct VirtioVirtq {
    descs: [VirtqDesc; VIRTQ_ENTRY_NUM],
    avail: VirtqAvail,
    used: VirtqUsed,
    queue_index: i32,
    used_index: *mut u16,
    last_used_index: u16,
}

// #[repr(packed)]
#[repr(align(4096))]
struct VirtioBlkReq {
    // 1st descriptor
    r#type: u32,
    reserved: u32,
    sector: u64,
    // 2nd descriptor
    data: [u8; SECTOR_SIZE],
    // 3rd descriptor
    status: u8,
}

unsafe fn virtio_reg_read32(offset: usize) -> u32 {
    ptr::read_volatile((VIRTIO_BLK_PADDR + offset) as *const u32)
}

unsafe fn virtio_reg_read64(offset: usize) -> u64 {
    ptr::read_volatile((VIRTIO_BLK_PADDR + offset) as *const u64)
}

unsafe fn virtio_reg_write32(offset: usize, value: u32) {
    println!("virtio_reg_write32: {:x}, {:x}", offset, value);
    ptr::write_volatile((VIRTIO_BLK_PADDR + offset) as *mut u32, value)
}

unsafe fn virtio_reg_fetch_and_or32(offset: usize, value: u32) {
    virtio_reg_write32(offset, virtio_reg_read32(offset) | value)
}

static mut BLK_REQUEST_VQ: *mut VirtioVirtq = ptr::null_mut();
static mut BLK_REQ: *mut VirtioBlkReq = ptr::null_mut();
static mut BLK_REQ_PADDR: paddr_t = 0;
static mut BLK_CAPACITY: u64 = 0;

pub unsafe fn virtio_blk_init() {
    if virtio_reg_read32(VIRTIO_REG_MAGIC) != 0x74726976 {
        panic!("virtio: invalid magic value");
    }
    if virtio_reg_read32(VIRTIO_REG_VERSION) != 1 {
        panic!("virtio: invalid version");
    }
    if virtio_reg_read32(VIRTIO_REG_DEVICE_ID) != VIRTIO_DEVICE_BLK as u32 {
        panic!("virtio: invalid device id");
    }

    // 1. Reset the device.
    virtio_reg_write32(VIRTIO_REG_DEVICE_STATUS, 0);

    // 2. Set the ACKNOWLEDGE status bit: the guest OS has noticed the device.
    virtio_reg_fetch_and_or32(VIRTIO_REG_DEVICE_STATUS, VIRTIO_STATUS_ACK);

    // 3. Set the DRIVER status bit.
    virtio_reg_fetch_and_or32(VIRTIO_REG_DEVICE_STATUS, VIRTIO_STATUS_DRIVER);

    // 5. Set the FEATURES_OK status bit.
    virtio_reg_fetch_and_or32(VIRTIO_REG_DEVICE_STATUS, VIRTIO_STATUS_FEAT_OK);

    // 7. Perform device-specific setup, including discovery of virtqueues for the device
    BLK_REQUEST_VQ = virtq_init(0);

    // 8. Set the DRIVER_OK status bit.
    virtio_reg_write32(VIRTIO_REG_DEVICE_STATUS, VIRTIO_STATUS_DRIVER_OK);

    // ディスクの容量を取得
    BLK_CAPACITY = virtio_reg_read64(VIRTIO_REG_DEVICE_CONFIG + 0) * SECTOR_SIZE as u64;
    println!("virtio-blk: capacity is {} bytes", BLK_CAPACITY);

    for i in 0..BLK_CAPACITY {
       print!("{:02x} ", *((VIRTIO_BLK_PADDR) as *const u8).offset(i as isize));
    }

    // デバイスへの処理要求を格納する領域を確保
    BLK_REQ_PADDR = MEM_MANAGER
        .alloc_pages(align_up(size_of::<VirtioBlkReq>(), PAGE_SIZE));

    println!("yes,,,: {}", align_up(size_of::<VirtioBlkReq>(), PAGE_SIZE) / PAGE_SIZE);
    
    BLK_REQ = BLK_REQ_PADDR as *mut VirtioBlkReq;
    for i in 0..size_of::<VirtioBlkReq>() {
        print!("{:02x} ", *(BLK_REQ.offset(i as isize) as *const u8))
    }

    println!("yes,,,: {:?}", BLK_REQ);
}

fn virtq_init(index: u32) -> *mut VirtioVirtq {
    unsafe {
        println!("virtq_init");
        println!("index1: {}", size_of::<VirtioVirtq>() / PAGE_SIZE);
        println!("index2: {}", align_up(size_of::<VirtioVirtq>(), PAGE_SIZE) / PAGE_SIZE);

        let virtq_paddr = MEM_MANAGER
            .alloc_pages(align_up(size_of::<VirtioVirtq>(), PAGE_SIZE) / PAGE_SIZE)
            as *mut VirtioVirtq;

        let vq: *mut VirtioVirtq = virtq_paddr as *mut VirtioVirtq;

        (*vq).queue_index = index as i32;
        // let x = &(*vq).last_used_index;
        (*vq).used_index = vq.offset(
            size_of::<VirtqDesc>() as isize * VIRTQ_ENTRY_NUM as isize
                + size_of::<VirtqAvail>() as isize
                + size_of::<VirtqUsed>() as isize
                + 4
                + 4,
        ) as *mut u16;

        println!("virtq_init2: {:?}", vq.offset(
            size_of::<VirtqDesc>() as isize * VIRTQ_ENTRY_NUM as isize
                + size_of::<VirtqAvail>() as isize
                + size_of::<VirtqUsed>() as isize
                + 4
                + 4,
        ) as *mut u16);

        // 1. Select the queue writing its index (first queue is 0) to QueueSel.
        virtio_reg_write32(VIRTIO_REG_QUEUE_SEL, index);

        // 5. Notify the device about the queue size by writing the size to QueueNum.
        virtio_reg_write32(VIRTIO_REG_QUEUE_NUM, VIRTQ_ENTRY_NUM as u32);

        // 6. Notify the device about the used alignment by writing its value in bytes to QueueAlign.
        virtio_reg_write32(VIRTIO_REG_QUEUE_ALIGN, 0);

        // 7. Write the physical number of the first page of the queue to the QueuePFN register.
        virtio_reg_write32(VIRTIO_REG_QUEUE_PFN, virtq_paddr as u32);

        vq
    }
}

unsafe fn virtq_kick(vq: *mut VirtioVirtq, desc_index: isize) {
    println!("virtq_kick");
    (*vq).avail.ring[(*vq).avail.idx as usize % VIRTQ_ENTRY_NUM] = desc_index as u16;
    (*vq).avail.idx = (*vq).avail.idx.wrapping_add(1);

    // fence(Ordering::Release); // メモリバリアを挿入
    compiler_fence(Ordering::SeqCst);
    virtio_reg_write32(VIRTIO_REG_QUEUE_NOTIFY, (*vq).queue_index as u32);
    (*vq).avail.idx = (*vq).avail.idx.wrapping_add(1);

    println!("virtq_kick!");

    return;
}

unsafe fn virtq_is_busy(vq: *const VirtioVirtq) -> bool {
    (*vq).used.idx != (*vq).last_used_index
}

pub unsafe fn read_write_disk(buf: &mut [u8], sector: u64, is_write: bool) {
    let blk_capacity = BLK_CAPACITY as u64;
    if sector >= blk_capacity / SECTOR_SIZE as u64 {
        println!(
            "virtio: tried to read/write sector={}, but capacity is {}",
            sector,
            blk_capacity / SECTOR_SIZE as u64
        );
        return;
    }

    // virtio-blkの仕様に従って、リクエストを構築する
    (*BLK_REQ).sector = sector;
    (*BLK_REQ).r#type = if is_write { 1 } else { 0 }; // VIRTIO_BLK_T_OUT = 1, VIRTIO_BLK_T_IN = 0
    if is_write {
        BLK_REQ.as_mut().unwrap().data.copy_from_slice(buf);
    }

    println!("yes");

    // virtqueueのディスクリプタを構築する (3つのディスクリプタを使う)
    let vq = BLK_REQUEST_VQ;
    (*vq).descs[0].addr = BLK_REQ_PADDR as u64;
    (*vq).descs[0].len = (size_of::<u32>() * 2 + size_of::<u64>()) as u32;
    (*vq).descs[0].flags = 2; // VIRTQ_DESC_F_NEXT
    (*vq).descs[0].next = 1;

    // println!("hoge: {:?}", (*vq).descs[0].addr);

    (*vq).descs[1].addr =
        BLK_REQ_PADDR as u64 + (size_of::<u32>() as u64 * 2 + size_of::<u64>() as u64);
    (*vq).descs[1].len = SECTOR_SIZE as u32;
    (*vq).descs[1].flags = 2 | if !is_write { 1 } else { 0 }; // VIRTQ_DESC_F_NEXT | (is_write ? 0 : VIRTQ_DESC_F_WRITE)
    (*vq).descs[1].next = 2;

    println!("hoge2");

    (*vq).descs[2].addr = BLK_REQ_PADDR as u64
        + (size_of::<u32>() as u64 * 2
            + size_of::<u64>() as u64
            + size_of::<[u8; SECTOR_SIZE]>() as u64);
    (*vq).descs[2].len = size_of::<u8>() as u32;
    (*vq).descs[2].flags = 1; // VIRTQ_DESC_F_WRITE


    // デバイスに新しいリクエストがあることを通知する
    virtq_kick(&mut *vq, 0);

    // デバイス側の処理が終わるまで待つ
    while virtq_is_busy(vq) {
        // デバイス側の処理が終わるまで待機
    }

    // virtio-blk: 0でない値が返ってきたらエラー
    if BLK_REQ.as_ref().unwrap().status != 0 {
        println!(
            "virtio: warn: failed to read/write sector={} status={}",
            sector,
            BLK_REQ.as_ref().unwrap().status
        );
        return;
    }
    println!("no");

    println!("data: {:?}", (*BLK_REQ).data);

    // 読み込み処理の場合は、バッファにデータをコピーする
    if !is_write {
        buf.copy_from_slice(&BLK_REQ.as_ref().unwrap().data);
    }
}

fn align_up(value: usize, align: usize) -> paddr_t {
    (value + align - 1) & !(align - 1)
}
