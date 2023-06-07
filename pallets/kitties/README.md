1. 在工程根目录添加 rust-toolchain.toml 文件

```toml
[toolchain]
channel = "nightly-2023-01-01"
components = [ "rustfmt" ]
targets = [ "wasm32-unknown-unknown" ]
profile = "minimal"

```

```
rustup install nightly-2023-01-01
rustup target add wasm32-unknown-unknown --toolchain nightly-2023-01-01
cargo +nightly-2023-01-01 build --release
```

2. 编译 node

```sh
cargo build --release 
```

3. 启动 node

```sh
./target/release/node-template --dev --base-path tmp/kitties
```

![第一次运行](docs/03/%E7%AC%AC%E4%B8%80%E6%AC%A1%E5%90%AF%E5%8A%A8.png)

4. 创建

![第一次创建](docs/03/%E7%AC%AC%E4%B8%80%E6%AC%A1create.png)

5. 链上状态

![第一次查询](docs/03/%E7%AC%AC%E4%B8%80%E6%AC%A1%E6%9F%A5%E8%AF%A2.png)

6. 添加升级代码后重新启动

![第二次运行](docs/03/%E7%AC%AC%E4%BA%8C%E6%AC%A1%E5%90%AF%E5%8A%A8.png)

7. 查看状态

![第二次查询](docs/03/%E7%AC%AC%E4%BA%8C%E6%AC%A1%E6%9F%A5%E8%AF%A2.png)

8. 升级

![升级](docs/03/%E5%8D%87%E7%BA%A7.png)

9. 重新查询

![查询](docs/03/%E5%8D%87%E7%BA%A7%E5%90%8E%E6%9F%A5%E8%AF%A2.png)

10. 创建

![创建](docs/03/%E5%8D%87%E7%BA%A7%E5%90%8E%E5%88%9B%E5%BB%BA.png)

11. 单元测试

![ut](docs/03/unit-test.png)