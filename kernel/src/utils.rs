pub fn is_aligned(addr: usize, align: usize) -> bool {
    addr % align == 0
}
