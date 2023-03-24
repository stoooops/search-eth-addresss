mod crypto;

fn main() {
    let result = crypto::generate().unwrap();
    println!("mnemonic: {}", result.mnemonic.phrase());
    println!("address: {}", result.address);
}
