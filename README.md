# grep-cli
跟着大佬写grep cli

## 使用
### 区分大小写
```shell
cargo run abc pom.txt
```
### 不区分大小写
```shell
CASE_INSENSITIVE=1 cargo run abc pom.txt
```

### 测试
```shell
cargo test
```