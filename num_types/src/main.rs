extern crate typename;
use typename::TypeName;

fn main() {
    let a = 10;
    let b = 5u32;
    let c = 4.3;
    let d = 5.1f32;
    let e = 8usize;
    println!("a({}): {}, b({}): {}, c({}): {}, d({}): {}, e({}): {}, e size: {}", 
        a.type_name_of(), a, 
        b.type_name_of(), b, 
        c.type_name_of(), c, 
        d.type_name_of(), d,
        e.type_name_of(), e,
        std::mem::size_of::<usize>());
}