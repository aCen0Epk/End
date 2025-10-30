# End
Develop backend API services using the Rust

## 项目简介

一个基于 Axum 的 Rust 后端服务，提供“计数器”与“计数记录”能力，采用 JWT 做接口鉴权，使用 SQLite 与 sqlx 进行数据持久化。项目内置请求追踪与 `x-request-id` 透传，适合作为小型服务或微信小程序后端示例。

## 技术栈

- Axum 0.7（Web 框架）
- sqlx 0.7（SQLite 驱动，异步 ORM）
- Tokio（异步运行时）
- JSON Web Token（`jsonwebtoken`）
- tower-http（Trace、Request-Id）
- dotenvy（环境变量）

## 目录结构

```
src/
  main.rs                # 路由注册与服务启动（端口 3000）
  api/
    jwt.rs               # JWT 生成与解析、中间件提取用户 UID
    user.rs              # 登录换取 JWT
    counter.rs           # 计数器 CRUD、置顶
    counter_record.rs    # 计数记录新增、查询
  db.rs                  # 数据库连接与数据模型（sqlx::FromRow）
migrations/              # SQLite 建表脚本
```

## 环境变量

在项目根目录创建 `.env`（本仓库已存在示例，请按需修改）：

```
DATABASE_URL=sqlite:data.db
JWT_SECRET=请替换为足够随机的密钥

# 微信登录相关（用于 jscode2session）
APP_ID=你的微信小程序AppId
APP_SECRET=你的微信小程序AppSecret
```

说明：
- `JWT_SECRET` 用于对 JWT 编解码（见 `src/api/jwt.rs`）。
- `DATABASE_URL` 使用 sqlx 的 SQLite 连接串，默认使用仓库中的 `data.db` 文件。
- `APP_ID` 与 `APP_SECRET` 用于调用微信 `jscode2session` 接口。

## 启动与运行

1) 安装 Rust 与 Cargo（稳定版），并确保已安装 SQLite（可选）。

2) 准备数据库：
- 方式A：直接使用仓库自带 `data.db`（即 `.env` 如上）。
- 方式B：从迁移脚本初始化（需要安装 `sqlx-cli`）：
  - 安装：`cargo install sqlx-cli --no-default-features --features native-tls,sqlite`
  - 运行迁移：`sqlx database create && sqlx migrate run`

3) 启动服务：

```
cargo run
```

默认监听：`127.0.0.1:3000`

## 身份认证

- 登录接口会返回 `access_token`（Bearer Token）。
- 业务接口需要在请求头添加：`Authorization: Bearer <token>`。
- Token 载荷 `sub` 为用户 ID，默认过期时间为 15 天（见 `Claims::new`）。

## API 文档

所有路由均以 `/api/wx_counter` 为前缀。

### 1. 用户登录

- 路径：`POST /api/wx_counter/login`
- 说明：使用微信 `code` 交换 openid，创建/获取用户并签发 JWT。
- 请求体：

```json
{ "code": "wx.login 后获得的临时 code" }
```

- 响应：

```json
{ "access_token": "<jwt>", "token_type": "Bearer" }
```

### 2. 计数器

- 获取列表：`GET /api/wx_counter/counters`（需鉴权）
  - 响应：`Counter[]`，按 `sequence desc` 排序。

- 新增：`POST /api/wx_counter/counters`（需鉴权）
  - 请求体：

```json
{ "name": "示例计数器", "value": 0, "step": 1, "input_step": false }
```

  - 响应：`{}`

- 查看单个：`GET /api/wx_counter/counters/:id`（需鉴权）
  - 响应：`Counter`

- 更新：`PUT /api/wx_counter/counters/:id`（需鉴权）
  - 请求体：

```json
{ "name": "新名称", "step": 2, "input_step": false }
```

  - 响应：`{}`

- 删除：`DELETE /api/wx_counter/counters/:id`（需鉴权）
  - 说明：同时删除该计数器下的所有计数记录。
  - 响应：`{}`

- 置顶：`POST /api/wx_counter/counters/:id/top`（需鉴权）
  - 说明：将 `sequence` 调整到当前用户下的最大值+1。
  - 响应：`{}`

计数器模型（部分字段）：

```json
{
  "id": 1,
  "user_id": 1,
  "name": "示例",
  "value": 10,
  "step": 1,
  "input_step": false,
  "sequence": 100,
  "created_at": "2025-10-21 06:41:46",
  "updated_at": "2025-10-21 06:41:46"
}
```

### 3. 计数记录

- 新增记录：`POST /api/wx_counter/counter_records`（需鉴权）
  - 请求体：

```json
{ "counter_id": 1, "step": 1 }
```

  - 说明：会在 `counter_records` 插入一条记录，并将对应计数器的 `value` 累加到 `end`，同时更新计数器当前值。
  - 响应：`{}`

- 查询记录列表：`GET /api/wx_counter/counter_records/:counter_id`（需鉴权）
  - 响应：`CounterRecord[]`，按 `id desc` 排序。

计数记录模型（部分字段）：

```json
{
  "id": 1,
  "counter_id": 1,
  "step": 1,
  "begin": 10,
  "end": 11,
  "created_at": "2025-10-21 06:41:46",
  "updated_at": "2025-10-21 06:41:46"
}
```

## 数据库结构

迁移脚本见 `migrations/`：
- `users(id, openid, session_key, created_at, updated_at)`
- `counters(id, user_id, name, value, step, input_step, sequence, created_at, updated_at)`
- `counter_records(id, counter_id, step, begin, end, created_at, updated_at)`

索引：
- `users_openid_index`（唯一）
- `counters_user_id_index (user_id asc, sequence desc)`
- `counter_records_counter_id_index (counter_id)`

## 本地调试要点

- 所有需要鉴权的接口必须携带请求头：`Authorization: Bearer <token>`。
- 服务固定监听 `127.0.0.1:3000`，如需对外暴露可在 `src/main.rs` 调整绑定地址。
- 默认启用请求追踪日志，包含 method、uri 与 `x-request-id`。



