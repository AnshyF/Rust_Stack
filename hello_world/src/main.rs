mod stack;

fn main() {
    let mut s = stack::Stack::new();
    s.push(1);
    s.push(2);
    for item in s {
        println!("{}", item); // 输出 2, 1
    }
}
