pub fn func() {
    let _a: usize = 0usize;
    let _b = _a as usize;

    let _c: usize = std::cmp::min(0usize, 1usize);
    let _d = _c as usize;

    let _e: usize = std::cmp::min(0usize, 1usize) as usize;
}
