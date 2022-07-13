# rust 加解密

[HOME](https://github.com/veypi/demo)

## [RustCrypto 项目](https://github.com/RustCrypto)

[github](https://github.com/RustCrypto/traits) [doc](https://docs.rs/crypto/latest/crypto/)

Rustcrypto 是一个单独发布的密码库特征集，其中包含许多不同类型的加密算法的traits。

这个包主要是对另外几个库进行re-export, 在import这个库时会自动管理依赖和兼容管理。

| 包                                                 | 功能             | 描述                                     |
| :------------------------------------------------- | :--------------- | :--------------------------------------- |
| [`aead`](https://docs.rs/aead)                     | `aead`           | 带有关联数据的认证加密（即高级对称加密） |
| [`cipher`](https://docs.rs/cipher)                 | `cipher`         | 块和流密码（即低级对称加密）             |
| [`digest`](https://docs.rs/digest)                 | `digest`         | 加密哈希函数                             |
| [`elliptic_curve`](https://docs.rs/elliptic-curve) | `elliptic-curve` | 椭圆曲线密码学                           |
| [`mac`](https://docs.rs/crypto-mac)                | `mac`            | 消息验证码（即对称消息验证）             |
| [`password_hash`](https://docs.rs/password-hash)   | `password-hash`  | 密码哈希函数                             |
| [`signature`](https://docs.rs/signature)           | `signature`      | 数字签名（即基于公钥的消息认证）         |
| [`universal_hash`](https://docs.rs/universal-hash) | `universal‑hash` | 通用哈希函数（用于构建 MAC）             |
