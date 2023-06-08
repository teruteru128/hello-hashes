fn main() {
    use hex_literal::hex;
    use sha2::{Sha512, Digest};
    use ripemd::Ripemd160;
    let sigpubkey = hex!("0437ae03031abb29b66b77faeff07fdecaa0e5d59dbb72d967664232d76a77068b7519d185fbeef4d4230d1c906d8d04dadbda44c6d4b7c96c14a4ec7413a5bd50");
    let encpubkey = hex!("048c14abe922a2fdeaf0c632120d3666fd3eac5c66739e211f8fe8b77c251d18228efc2f0f9bc1c423ad269b1f9e97663a014b200a55dfd84ea9e4a0f175871f50");
    let mut sha512 = Sha512::new();
    sha512.update(sigpubkey);
    sha512.update(encpubkey);
    let hash = sha512.finalize();
    let mut ripemd160 = Ripemd160::new();
    ripemd160.update(hash);
    let ripe = ripemd160.finalize();
    println!("{:x?}", ripe);
}
