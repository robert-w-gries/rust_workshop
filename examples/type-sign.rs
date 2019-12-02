fn main() {
        let si:i32 = -1;
        let ui:u32 = 1;
#[cfg(feature = "broken")]
        println!("-1 < 1 : {}", si < ui );
}
