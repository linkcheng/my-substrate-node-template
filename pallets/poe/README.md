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
