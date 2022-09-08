![image](./success.png)

### build teacher-service

- cargo build --bin teacher-service --release

### build webapp

- cargo build --bin svr --release

### 启动 teacher-service

#### 临时设置环境变量，我这里是 mac

- export DATABASE_URL=postgres://postgres:123456@127.0.0.1:5432/postgres

- 如果没有 postgres 就用 docker 快速体验下

- docker run --name=postgres -p 5432:5432 -e POSTGRES_PASSWORD=123456 postgres:11

- 建表 sql 在根目录 db.sql 里面，PG 我没怎么用过，导出来的不知道对不对，自己测试吧

- ./teacher-service 就能启动了 这里代码里写死是 3000 端口

### 启动 webapp

- export HOST_PORT=127.0.0.1:8082
- ./svr

### build wasm

- cd wasm-client
- wasm-pack build --release

- cd www
- npm run build 构建完在 www/dist 目录下

### 将 dist 放到 web 服务器下，这里使用 http-server

- npm install -g http-server 没装的就安装下
- http-server ./dist -p 8081

### 注意，上面 8081 端口需要使用 127.0.0.1，访问，我在跨域哪里配置了 http 开头的都行，至于 localhost 为什么不行，有空再研究下
