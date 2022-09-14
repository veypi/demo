# rust 加解密

[HOME](https://github.com/veypi/demo)

## [RustCrypto 项目](https://github.com/RustCrypto)

[github](https://github.com/RustCrypto/traits) [doc](https://docs.rs/crypto/latest/crypto/)

Rustcrypto 是一个单独发布的密码库特征集，其中包含许多不同类型的加密算法的traits。

这个包主要是对另外几个库进行re-export。

| 特征crate                                                    | 简介                                                         | Crates.io                                                    | 文档                                                         | 加密crate                                                    |
| ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ |
| [`aead`](https://github.com/RustCrypto/traits/blob/master/aead) | [经过身份验证的加密](https://en.wikipedia.org/wiki/Authenticated_encryption) | [![板条箱.io](https://camo.githubusercontent.com/60cb70d05a5de9d8efeb9272fe84c996d75cd0497d2d40a200df2ea715e32d48/68747470733a2f2f696d672e736869656c64732e696f2f6372617465732f762f616561642e737667)](https://crates.io/crates/aead) | [![文档](https://camo.githubusercontent.com/0209ecbe9184f549399ec69e6b25a1c4f8bc222d6b1fd30359c803ed27066a8f/68747470733a2f2f646f63732e72732f616561642f62616467652e737667)](https://docs.rs/aead) | [aeds](https://github.com/RustCrypto/AEADs)                  |
| [`async‑signature`](https://github.com/RustCrypto/traits/blob/master/signature/async) | [数字签名](https://en.wikipedia.org/wiki/Digital_signature)  | [![板条箱.io](https://camo.githubusercontent.com/1ea41b12f924c9431421568022e5e888902041ba132554155d13256ebc514c5e/68747470733a2f2f696d672e736869656c64732e696f2f6372617465732f762f6173796e632d7369676e61747572652e737667)](https://crates.io/crates/async-signature) | [![文档](https://camo.githubusercontent.com/a63fb16dea42c6c4901b057a8848f7945189d7d579c8d3b05d407bcf095ba372/68747470733a2f2f646f63732e72732f6173796e632d7369676e61747572652f62616467652e737667)](https://docs.rs/async-signature) |                                                              |
| [`cipher`](https://github.com/RustCrypto/traits/blob/master/cipher) | [块](https://en.wikipedia.org/wiki/Block_cipher)和[流密码](https://en.wikipedia.org/wiki/Stream_cipher) | [![板条箱.io](https://camo.githubusercontent.com/8ba85725f512dfa09d4e4769518ad4186e623c40ec2305d3b9fb2345657b72e0/68747470733a2f2f696d672e736869656c64732e696f2f6372617465732f762f6369706865722e737667)](https://crates.io/crates/cipher) | [![文档](https://camo.githubusercontent.com/44006191571e5018846dffda6ae9239e1c6d2b9fc308ef2b61fb3267512200bd/68747470733a2f2f646f63732e72732f6369706865722f62616467652e737667)](https://docs.rs/cipher) | [block](https://github.com/RustCrypto/block-ciphers) [stream](https://github.com/RustCrypto/stream-ciphers) |
| [`crypto‑common`](https://github.com/RustCrypto/traits/blob/master/crypto-common) | 常见的加密特征                                               | [![板条箱.io](https://camo.githubusercontent.com/f94306f4c36feff6ca1dc35cc2d4c4a3db6d7725f5fc1347d20fca8182676b56/68747470733a2f2f696d672e736869656c64732e696f2f6372617465732f762f63727970746f2d636f6d6d6f6e2e737667)](https://crates.io/crates/crypto-common) | [![文档](https://camo.githubusercontent.com/a273ee4f2b4c6c60d7f5add00e6818deba296ed1cbbe4109d5bb31f6dd7f2267/68747470733a2f2f646f63732e72732f63727970746f2d636f6d6d6f6e2f62616467652e737667)](https://docs.rs/crypto-common) |                                                              |
| [`digest`](https://github.com/RustCrypto/traits/blob/master/digest) | [加密哈希函数](https://en.wikipedia.org/wiki/Cryptographic_hash_function) | [![板条箱.io](https://camo.githubusercontent.com/6bcb4fbdd55aa02e6793534a54da65fbd98e2b7cb2ecf27f3226a4a521da076d/68747470733a2f2f696d672e736869656c64732e696f2f6372617465732f762f6469676573742e737667)](https://crates.io/crates/digest) | [![文档](https://camo.githubusercontent.com/8baf5a7557e59e2be62fa5e73bb6f2f9d14bf1678e73f0fa4a6fa6f2ed6a3716/68747470733a2f2f646f63732e72732f6469676573742f62616467652e737667)](https://docs.rs/digest) | [hashes](https://github.com/RustCrypto/hashes)               |
| [`elliptic‑curve`](https://github.com/RustCrypto/traits/blob/master/elliptic-curve) | [椭圆曲线密码学](https://en.wikipedia.org/wiki/Elliptic-curve_cryptography) | [![板条箱.io](https://camo.githubusercontent.com/13ea3ac8e7747fc33172657c5f8c8058c1d0cb15808d1c2d6047f204dadc56af/68747470733a2f2f696d672e736869656c64732e696f2f6372617465732f762f656c6c69707469632d63757276652e737667)](https://crates.io/crates/elliptic-curve) | [![文档](https://camo.githubusercontent.com/384100ff0c4ecae6526918b77b885fbde2edba977ea778e1a4662b51b31b681d/68747470733a2f2f646f63732e72732f656c6c69707469632d63757276652f62616467652e737667)](https://docs.rs/elliptic-curve) |                                                              |
| [`kem`](https://github.com/RustCrypto/traits/blob/master/kem) | [关键封装机制](https://en.wikipedia.org/wiki/Key_encapsulation) | [![板条箱.io](https://camo.githubusercontent.com/2a6959dfb1754cc629bd144c675fc1ebd5af1029f8b33278b47c2843783502de/68747470733a2f2f696d672e736869656c64732e696f2f6372617465732f762f6b656d2e737667)](https://crates.io/crates/kem) | [![文档](https://camo.githubusercontent.com/963dc2fa1d9e8dc6700a17c3d527bb0956e85e1dd771fa511e585587a4d6f6eb/68747470733a2f2f646f63732e72732f6b656d2f62616467652e737667)](https://docs.rs/kem) |                                                              |
| [`password-hash`](https://github.com/RustCrypto/traits/blob/master/password-hash) | [密码哈希](https://en.wikipedia.org/wiki/Cryptographic_hash_function#Password_verification) | [![板条箱.io](https://camo.githubusercontent.com/ce8f527028893e618e3c77968bc2c60f0dc28278bff624016cfd4ae11f214f5a/68747470733a2f2f696d672e736869656c64732e696f2f6372617465732f762f70617373776f72642d686173682e737667)](https://crates.io/crates/password-hash) | [![文档](https://camo.githubusercontent.com/a48a31e4c784ac63bfaa2817769740545edbf8c4b625a4b1df9c920a1a2882bb/68747470733a2f2f646f63732e72732f70617373776f72642d686173682f62616467652e737667)](https://docs.rs/password-hash) | [password-hashes](https://github.com/RustCrypto/password-hashes) |
| [`signature`](https://github.com/RustCrypto/traits/blob/master/signature) | [数字签名](https://en.wikipedia.org/wiki/Digital_signature)  | [![板条箱.io](https://camo.githubusercontent.com/c2a314cece8bb468c584396765cf85a05ef29f48b1443c721961c33eb1b3fc18/68747470733a2f2f696d672e736869656c64732e696f2f6372617465732f762f7369676e61747572652e737667)](https://crates.io/crates/signature) | [![文档](https://camo.githubusercontent.com/032bf00cd019ad4bd50f978e943b375de6948a15fecd8df33cc4cacef547dd41/68747470733a2f2f646f63732e72732f7369676e61747572652f62616467652e737667)](https://docs.rs/signature) | [ecdsa](https://github.com/RustCrypto/signatures/tree/master/ecdsa)  [Ed25519](https://github.com/RustCrypto/signatures/tree/master/ed25519) |
| [`universal‑hash`](https://github.com/RustCrypto/traits/blob/master/universal-hash) | [通用哈希函数](https://en.wikipedia.org/wiki/Universal_hashing) | [![板条箱.io](https://camo.githubusercontent.com/6757e27b572d546b71d7c5b28cdb1dc97d6704ae90d2f3559d43b445a40221d4/68747470733a2f2f696d672e736869656c64732e696f2f6372617465732f762f756e6976657273616c2d686173682e737667)](https://crates.io/crates/universal-hash) | [![文档](https://camo.githubusercontent.com/b7bd351c7467274a85e44840376172118fb66c4fdbbe89b94ef7191c3c91d56e/68747470733a2f2f646f63732e72732f756e6976657273616c2d686173682f62616467652e737667)](https://docs.rs/universal-hash) | [universal-hashes](https://github.com/RustCrypto/universal-hashes) |

在最右侧中包含了实现这些特征的crate，最终使用也是看右侧这些库

#### [aes_gcm](https://github.com/RustCrypto/AEADs/tree/master/aes-gcm)

该算法使用aes加密，并使用counter模式保证算法的完整性。

样例

```bash
make gotag t=rust-encroption-aes_gcm
```



### Padding

在加密中我们经常需要填充块，encrypto中已经写了相应的填充算法，https://github.com/RustCrypto/utils
