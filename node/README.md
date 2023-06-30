1. 编译

```sh
cargo build --release 
```

2. 生成 staging 文件

```sh
./target/release/node-template build-spec --chain staging > staging.json
2023-06-30 23:04:26 Building chain spec    
```

3. 生成 raw 文件
```sh
./target/release/node-template build-spec --chain=staging.json --raw > staging-raw.json
2023-06-30 23:05:28 Building chain spec    
2023-06-30 23:05:28 [0] 💸 generated 4 npos voters, 4 from validators and 0 nominators    
2023-06-30 23:05:28 [0] 💸 generated 4 npos targets    
```