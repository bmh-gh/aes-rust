use aes_rust::*;

fn main() {
  //let main_key: u128 = 0xDECAFBAD_C0DEBA5E_DEADC0DE_BADC0DED;
  let main_key: u128 = 0x2b7e151628aed2a6abf7158809cf4f3c;
  let keys = aes_rust::key_schedule(main_key);
  println!("{:x?}", keys)
}