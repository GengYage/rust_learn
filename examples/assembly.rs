use std::arch::asm;

fn main() {
    println!("{}", add(1, 2));
}

/// x86 汇编实现的加法
fn add(x: i32, y: i32) -> i32 {
    let mut result: i32;
    unsafe {
        // e 表示 eax 32位长度
        // r 表示 rax 64位长度
        // 占位符中:前面的数字是后面的参数的索引,从0开始
        // in 可以用于输入立即数
        // out 可用于将寄存器的指输出到rust的变量(可变变量)
        asm!(
        "mov {0:e}, {1:e}",
        "mov {3:e}, {2:e}",
        "add {0:e}, {3:e}",
        out(reg) result,
        in(reg) x,
        in(reg) y,
        out(reg) _,
        )
    }
    result
}

#[cfg(test)]
mod test {
    use crate::add;

    #[test]
    fn test_assembly_add() {
        assert_eq!(3, add(1, 2));
        assert_eq!(0, add(-1, 1));
    }
}
