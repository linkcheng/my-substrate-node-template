benchmark
1. 编译
```sh
$ cargo build --release --features runtime-benchmarks

Compiling sc-cli v0.10.0-dev (https://github.com/paritytech/substrate.git?branch=polkadot-v0.9.40#98f2e345)
Compiling node-template-runtime v4.0.0-dev (~/proj/substrate-node-template/runtime)
Compiling frame-benchmarking-cli v4.0.0-dev (https://github.com/paritytech/substrate.git?branch=polkadot-v0.9.40#98f2e345)
Compiling node-template v4.0.0-dev (~/proj/substrate-node-template/node)
Finished release [optimized] target(s) in 5m 02s
```

2. 运行
```
 ./target/release/node-template benchmark pallet --chain dev --execution wasm --wasm-execution compiled --pallet pallet_poe --extrinsic "*" --steps 20 --repeat 10 --json-file=raw.json --output ./pallets/poe/src/weights.rs --template .maintain/frame-weight-template.hbs

2023-06-28 23:01:55 Starting benchmark: pallet_poe::create_claim    
2023-06-28 23:01:55 Starting benchmark: pallet_poe::revoke_claim    
2023-06-28 23:01:55 Starting benchmark: pallet_poe::transfer_claim    
Pallet: "pallet_poe", Extrinsic: "create_claim", Lowest values: [], Highest values: [], Steps: 20, Repeat: 10
Raw Storage Info
========
Storage: PoeModule Proofs (r:1 w:1)
Proof: PoeModule Proofs (max_values: None, max_size: Some(566), added: 3041, mode: MaxEncodedLen)

-- Proof Sizes --

6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes
6 bytes

Median Slopes Analysis
========
-- Extrinsic Time --

Model:
Time ~=       32
    + d    0.007
              µs

Reads = 1 + (0 * d)
Writes = 1 + (0 * d)
Recorded proof Size = 6 + (0 * d)

Min Squares Analysis
========
-- Extrinsic Time --

Data points distribution:
    d   mean µs  sigma µs       %
    0        31         0    0.0%
   26      31.5       0.5    1.5%
   53      32.5       0.5    1.5%
   80     32.83     0.372    1.1%
  107      33.5     1.707    5.0%
  134     41.83      8.61   20.5%
  161      33.5     0.763    2.2%
  188        34     0.816    2.4%
  215        34         0    0.0%
  242     34.16     0.372    1.0%
  269      34.5       0.5    1.4%
  296     34.16     0.372    1.0%
  323     34.33     0.471    1.3%
  350      34.5       0.5    1.4%
  377     34.83     0.372    1.0%
  404     35.16     0.372    1.0%
  431     35.16     0.372    1.0%
  458     35.33     0.471    1.3%
  485     37.66     2.054    5.4%
  512        36     0.577    1.6%

Quality and confidence:
param     error
d         0.001

Model:
Time ~=    32.83
    + d    0.006
              µs

Reads = 1 + (0 * d)
Writes = 1 + (0 * d)
Recorded proof Size = 6 + (0 * d)

Pallet: "pallet_poe", Extrinsic: "revoke_claim", Lowest values: [], Highest values: [], Steps: 20, Repeat: 10
Raw Storage Info
========
Storage: PoeModule Proofs (r:1 w:1)
Proof: PoeModule Proofs (max_values: None, max_size: Some(566), added: 3041, mode: MaxEncodedLen)

-- Proof Sizes --

85 bytes
85 bytes
85 bytes
85 bytes
85 bytes
85 bytes
85 bytes
85 bytes
85 bytes
85 bytes
111 bytes
111 bytes
111 bytes
111 bytes
111 bytes
111 bytes
111 bytes
111 bytes
111 bytes
111 bytes
139 bytes
139 bytes
139 bytes
139 bytes
139 bytes
139 bytes
139 bytes
139 bytes
139 bytes
139 bytes
167 bytes
167 bytes
167 bytes
167 bytes
167 bytes
167 bytes
167 bytes
167 bytes
167 bytes
167 bytes
194 bytes
194 bytes
194 bytes
194 bytes
194 bytes
194 bytes
194 bytes
194 bytes
194 bytes
194 bytes
222 bytes
222 bytes
222 bytes
222 bytes
222 bytes
222 bytes
222 bytes
222 bytes
222 bytes
222 bytes
249 bytes
249 bytes
249 bytes
249 bytes
249 bytes
249 bytes
249 bytes
249 bytes
249 bytes
249 bytes
276 bytes
276 bytes
276 bytes
276 bytes
276 bytes
276 bytes
276 bytes
276 bytes
276 bytes
276 bytes
303 bytes
303 bytes
303 bytes
303 bytes
303 bytes
303 bytes
303 bytes
303 bytes
303 bytes
303 bytes
330 bytes
330 bytes
330 bytes
330 bytes
330 bytes
330 bytes
330 bytes
330 bytes
330 bytes
330 bytes
358 bytes
358 bytes
358 bytes
358 bytes
358 bytes
358 bytes
358 bytes
358 bytes
358 bytes
358 bytes
385 bytes
385 bytes
385 bytes
385 bytes
385 bytes
385 bytes
385 bytes
385 bytes
385 bytes
385 bytes
412 bytes
412 bytes
412 bytes
412 bytes
412 bytes
412 bytes
412 bytes
412 bytes
412 bytes
412 bytes
439 bytes
439 bytes
439 bytes
439 bytes
439 bytes
439 bytes
439 bytes
439 bytes
439 bytes
439 bytes
466 bytes
466 bytes
466 bytes
466 bytes
466 bytes
466 bytes
466 bytes
466 bytes
466 bytes
466 bytes
494 bytes
494 bytes
494 bytes
494 bytes
494 bytes
494 bytes
494 bytes
494 bytes
494 bytes
494 bytes
521 bytes
521 bytes
521 bytes
521 bytes
521 bytes
521 bytes
521 bytes
521 bytes
521 bytes
521 bytes
548 bytes
548 bytes
548 bytes
548 bytes
548 bytes
548 bytes
548 bytes
548 bytes
548 bytes
548 bytes
575 bytes
575 bytes
575 bytes
575 bytes
575 bytes
575 bytes
575 bytes
575 bytes
575 bytes
575 bytes
603 bytes
603 bytes
603 bytes
603 bytes
603 bytes
603 bytes
603 bytes
603 bytes
603 bytes
603 bytes

Median Slopes Analysis
========
-- Extrinsic Time --

Model:
Time ~=    33.16
    + d    0.021
              µs

Reads = 1 + (0 * d)
Writes = 1 + (0 * d)
Recorded proof Size = 86 + (1 * d)

Min Squares Analysis
========
-- Extrinsic Time --

Data points distribution:
    d   mean µs  sigma µs       %
    0        31         0    0.0%
   26     32.66     0.471    1.4%
   53        34         0    0.0%
   80     34.83     0.372    1.0%
  107     35.33     0.471    1.3%
  134     37.33     0.942    2.5%
  161        37     0.577    1.5%
  188     38.66     2.054    5.3%
  215     37.83     0.372    0.9%
  242      38.5       0.5    1.2%
  269     39.33     0.471    1.1%
  296        40         0    0.0%
  323     40.33     0.471    1.1%
  350     40.33     0.471    1.1%
  377     39.83     0.372    0.9%
  404     41.33     0.471    1.1%
  431     42.33     0.471    1.1%
  458     41.66     0.471    1.1%
  485     44.66     1.972    4.4%
  512     43.83     0.372    0.8%

Quality and confidence:
param     error
d             0

Model:
Time ~=    32.93
    + d    0.021
              µs

Reads = 1 + (0 * d)
Writes = 1 + (0 * d)
Recorded proof Size = 86 + (1 * d)

Pallet: "pallet_poe", Extrinsic: "transfer_claim", Lowest values: [], Highest values: [], Steps: 20, Repeat: 10
Raw Storage Info
========
Storage: PoeModule Proofs (r:1 w:1)
Proof: PoeModule Proofs (max_values: None, max_size: Some(566), added: 3041, mode: MaxEncodedLen)

-- Proof Sizes --

85 bytes
85 bytes
85 bytes
85 bytes
85 bytes
85 bytes
85 bytes
85 bytes
85 bytes
85 bytes
111 bytes
111 bytes
111 bytes
111 bytes
111 bytes
111 bytes
111 bytes
111 bytes
111 bytes
111 bytes
139 bytes
139 bytes
139 bytes
139 bytes
139 bytes
139 bytes
139 bytes
139 bytes
139 bytes
139 bytes
167 bytes
167 bytes
167 bytes
167 bytes
167 bytes
167 bytes
167 bytes
167 bytes
167 bytes
167 bytes
194 bytes
194 bytes
194 bytes
194 bytes
194 bytes
194 bytes
194 bytes
194 bytes
194 bytes
194 bytes
222 bytes
222 bytes
222 bytes
222 bytes
222 bytes
222 bytes
222 bytes
222 bytes
222 bytes
222 bytes
249 bytes
249 bytes
249 bytes
249 bytes
249 bytes
249 bytes
249 bytes
249 bytes
249 bytes
249 bytes
276 bytes
276 bytes
276 bytes
276 bytes
276 bytes
276 bytes
276 bytes
276 bytes
276 bytes
276 bytes
303 bytes
303 bytes
303 bytes
303 bytes
303 bytes
303 bytes
303 bytes
303 bytes
303 bytes
303 bytes
330 bytes
330 bytes
330 bytes
330 bytes
330 bytes
330 bytes
330 bytes
330 bytes
330 bytes
330 bytes
358 bytes
358 bytes
358 bytes
358 bytes
358 bytes
358 bytes
358 bytes
358 bytes
358 bytes
358 bytes
385 bytes
385 bytes
385 bytes
385 bytes
385 bytes
385 bytes
385 bytes
385 bytes
385 bytes
385 bytes
412 bytes
412 bytes
412 bytes
412 bytes
412 bytes
412 bytes
412 bytes
412 bytes
412 bytes
412 bytes
439 bytes
439 bytes
439 bytes
439 bytes
439 bytes
439 bytes
439 bytes
439 bytes
439 bytes
439 bytes
466 bytes
466 bytes
466 bytes
466 bytes
466 bytes
466 bytes
466 bytes
466 bytes
466 bytes
466 bytes
494 bytes
494 bytes
494 bytes
494 bytes
494 bytes
494 bytes
494 bytes
494 bytes
494 bytes
494 bytes
521 bytes
521 bytes
521 bytes
521 bytes
521 bytes
521 bytes
521 bytes
521 bytes
521 bytes
521 bytes
548 bytes
548 bytes
548 bytes
548 bytes
548 bytes
548 bytes
548 bytes
548 bytes
548 bytes
548 bytes
575 bytes
575 bytes
575 bytes
575 bytes
575 bytes
575 bytes
575 bytes
575 bytes
575 bytes
575 bytes
603 bytes
603 bytes
603 bytes
603 bytes
603 bytes
603 bytes
603 bytes
603 bytes
603 bytes
603 bytes

Median Slopes Analysis
========
-- Extrinsic Time --

Model:
Time ~=    36.02
    + d    0.022
              µs

Reads = 1 + (0 * d)
Writes = 1 + (0 * d)
Recorded proof Size = 86 + (1 * d)

Min Squares Analysis
========
-- Extrinsic Time --

Data points distribution:
    d   mean µs  sigma µs       %
    0     33.66     1.105    3.2%
   26     35.66     0.471    1.3%
   53     40.16      3.89    9.6%
   80     41.33     3.299    7.9%
  107      38.5     1.118    2.9%
  134      40.5     1.892    4.6%
  161     39.33     0.471    1.1%
  188     40.83     1.067    2.6%
  215     45.16     3.975    8.8%
  242        48     9.291   19.3%
  269        42         0    0.0%
  296     52.33     16.12   30.8%
  323     43.16     0.372    0.8%
  350     45.16     1.067    2.3%
  377     50.33     8.034   15.9%
  404        45         0    0.0%
  431     47.33     2.624    5.5%
  458     46.66     2.054    4.4%
  485        46         0    0.0%
  512     46.66     0.471    1.0%

Quality and confidence:
param     error
d         0.003

Model:
Time ~=    37.51
    + d    0.023
              µs

Reads = 1 + (0 * d)
Writes = 1 + (0 * d)
Recorded proof Size = 86 + (1 * d)

Created file: "./pallets/poe/src/weights.rs"
```

3. 编译poe模块
```
cargo build --release 

Compiling pallet-poe v4.0.0-dev (~/proj/substrate-node-template/pallets/poe)
Finished release [optimized] target(s) in 1.11s
```


单元测试
![10 单元测试.jps](docs/10%20%E5%8D%95%E5%85%83%E6%B5%8B%E8%AF%95.jpg)

常用的宏

```
#[frame_support::pallet] 定义功能模块
#[pallet::config] 定义配置接口
#[pallet::storage] 存储单元
#[pallet::event] 事件
#[pallet::error] 错误信息
#[pallet::call] 可调用函数
#[pallet::hooks] 钩子函数，区块不同时期调用的函数
#[pallet::weight(0)] 消耗资源的权重
construct_runtime! 添加模块到runtime
```

常用的存储数据结构

```
StorageValue 单值
StorageMap 映射
StorageDoubleMap 双键映射
StorageNmap 多键映射
```

创建、转移、撤销存证

1. 启动区块链
![1 启动.png](docs%2F1%20%E5%90%AF%E5%8A%A8.png)
2. 在浏览器查看
![2 浏览器展示.png](docs%2F2%20%E6%B5%8F%E8%A7%88%E5%99%A8%E5%B1%95%E7%A4%BA.png)
3. 第一次查看存证
![3 第一次查询.png](docs%2F3%20%E7%AC%AC%E4%B8%80%E6%AC%A1%E6%9F%A5%E8%AF%A2.png)
4. Alice 创建存证 0x01
![4 alice create claim.png](docs%2F4%20alice%20create%20claim.png)
5. 再次查看存证在 Alice 名下
![5 查看存证在alice名下.png](docs%2F5%20%E6%9F%A5%E7%9C%8B%E5%AD%98%E8%AF%81%E5%9C%A8alice%E5%90%8D%E4%B8%8B.png)
6. 转移存证到 Bob 名下
![6 alice transfer claim to bob.png](docs%2F6%20alice%20transfer%20claim%20to%20bob.png)
7. 再次查看存证在 Bob 名下
![7 查询存证在bob名下.png](docs%2F7%20%E6%9F%A5%E8%AF%A2%E5%AD%98%E8%AF%81%E5%9C%A8bob%E5%90%8D%E4%B8%8B.png)
8. 撤销存证
![8 revoke claim.png](docs%2F8%20revoke%20claim.png)
9. 再次查看存证不存在
![9 再次查询存证不存在.png](docs%2F9%20%E5%86%8D%E6%AC%A1%E6%9F%A5%E8%AF%A2%E5%AD%98%E8%AF%81%E4%B8%8D%E5%AD%98%E5%9C%A8.png)
