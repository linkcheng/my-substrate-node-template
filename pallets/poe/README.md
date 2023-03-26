
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
StorageValue 单指
StorageMap 映射
StorageDoubleMap 双键映射
StorageNmap 多键映射
```
