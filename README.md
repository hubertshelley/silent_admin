# Silent Admin

## 数据迁移说明

```shell
# 生成迁移文件
sea-orm-cli migrate generate ....

# 根据数据库结构生成实体文件
sea-orm-cli generate entity -o entity/src -l --with-serde both --serde-skip-hidden-column --model-extra-attributes 'serde(rename_all = "camelCase")'
```

### 开发环境数据库迁移

```shell
# 迁移数据库
sea-orm-cli migrate up
```

### 生产环境数据库迁移

```shell
# 直接运行二进制程序
./bpm_core
```
