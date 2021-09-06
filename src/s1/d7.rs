// 所有权 ---- ----

fn main() {
    let data = vec![10, 42, 9, 8];
    let v = 42;
    if let Some(pos) = find_pos(data, v) {
        println!("Found {} at {}", v, pos);
    }
}

fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    for (pos, item) in data.iter().enumerate() {
        if *item == v {
            return Some(pos);
        }
    }
    
    None
}

/**
    在find_pos()的栈帧中，第二个参数栈上保存的是0x0000_002A并不是main()中v所在的栈地址，没有所有权的转移(Copy trit)
    也就是说find_pos()函数中无论怎么改变入参v的值，在find_pos()函数结束的时候也不会导致main()中v值的回收，故main()中v的值是不会改变的，是安全的
*/


// =================================================================


fn main() {
    let data = vec![1, 2, 3, 4];
    let data1 = data;
    println!("sum of data1: {}", sum(data1));
    println!("data1: {:?}", data1); // error1
    println!("sum of data: {}", sum(data)); // error2
}

fn sum(data: Vec<u32>) -> u32 {
    data.iter().fold(0, |acc, x| acc + x)
}


// 栈上的数据结构引用堆上的结构，如果栈上的生命周期 > 堆上的生命周期 则可以进行引用
fn main(){
    let x = 1;
    let y = 2;
    let v = vec![&x, &y];
    println!("{:?}", v);
}