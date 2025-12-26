#![feature(f16)]

fn main() {
    // These conversions fail to link on macos due to missing compiler builtins:
    // - __floattihf (i128 → f16)
    // - __floatuntihf (u128 → f16)

    let u = 1000u128;
    let i = -1000i128;

    let f_from_u: f16 = u as f16;
    let f_from_i: f16 = i as f16;

    println!("u128 → f16: {} → {}", u, f_from_u);
    println!("i128 → f16: {} → {}", i, f_from_i);

    // The reverse direction also fails:
    let f = 42.0f16;
    let u_from_f: u128 = f as u128;
    let i_from_f: i128 = f as i128;

    println!("f16 → u128: {} → {}", f, u_from_f);
    println!("f16 → i128: {} → {}", f, i_from_f);
}
