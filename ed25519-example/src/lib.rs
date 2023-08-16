extern crate ed25519_dalek;

use ed25519_dalek::{Keypair, Signature, Signer};
use rand::rngs::OsRng;

pub fn ed25519_example() {
    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);

    let message: &[u8] = b"This is a test of the tsunami alert system.";
    let signature: Signature = keypair.sign(message);
    assert!(keypair.verify(message, &signature).is_ok());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ed25519() {
        // 生成随机的秘钥对
        ed25519_example();
    }

    //这个示例使用了ed25519_dalek库，它提供了对Ed25519签名算法的实现。
    //首先，我们使用Keypair::generate函数生成一个随机的秘钥对。然后，我们从秘钥对中获取公钥和私钥。
    //
    //接下来，我们定义要签名的消息，并使用keypair.sign函数对消息进行签名，得到一个签名对象。
    //然后，我们使用公钥的verify方法验证签名，并检查验证结果。
    //
    // 最后，我们打印签名和验证结果。请注意，这只是一个简单的示例，实际使用中可能需要更多的错误处理和其他安全措施。
    #[test]
    fn test_ed25519_dalek() {
        use ed25519_dalek::PublicKey;
        use ed25519_dalek::Verifier;

        // 生成随机的秘钥对
        let mut csprng = OsRng {};
        let keypair: Keypair = Keypair::generate(&mut csprng);

        println!("Generate Random KeyPair: ({:?})", keypair);

        // 从秘钥对中获取公钥和私钥
        let public_key: PublicKey = keypair.public;
        // let secret_key = keypair.secret;

        // 生成要签名的消息
        let message: &[u8] = b"Hello, world!";

        // 对消息进行签名
        let signature: Signature = keypair.sign(message);

        // 验证签名
        let is_verified: bool = public_key.verify(message, &signature).is_ok();

        // 打印签名和验证结果
        println!("Signature: {:?}", signature);
        println!("Verification result: {:?}", is_verified);
    }
}
