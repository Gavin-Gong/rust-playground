/// i 开头带符号整数
/// u 开头无符号整数
/// size 根据平台分配
/// 数值大小如 i32 表示 32 bit
pub fn int_feat() {
    {
        let _a: i8 = 1;
        let _b: i16 = 1;
        let _c: i32 = 1;
        let _d: i64 = 1;
        let _e: i128 = 1;
        let _f: isize = 1;
    }
    {
        let _a: u8 = 1;
        let _b: u16 = 1;
        let _c: u32 = 1;
        let _d: u64 = 1;
        let _e: u128 = 1;
        let _f: usize = 1;
    }
}

pub fn float_feat() {
    {
        let _c: f32 = 1.0;
        let _d: f64 = 1.0;
    }
}

pub fn unit_feat() {
    let _a = ();
}

pub fn bool_feat() {
    let _b: bool = true;
}

pub fn char_feat() {
    let _c = 'Z';
}
