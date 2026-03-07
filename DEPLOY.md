# 阿里云 ECS 部署指南

## 前置准备

1. **购买 ECS 实例**
   - 推荐配置：2 核 4G 或以上
   - 系统：Ubuntu 22.04 LTS 或 Debian 12

2. **配置安全组**
   - 开放端口：80 (HTTP), 443 (HTTPS), 22 (SSH)

---

## 方案一：Docker 部署（推荐）

### 1. 安装 Docker

```bash
# SSH 登录服务器
ssh root@<your-ecs-ip>

# 安装 Docker
curl -fsSL https://get.docker.com | bash -s docker

# 安装 docker-compose
apt install docker-compose -y

# 验证安装
docker --version
docker-compose --version
```

### 2. 上传代码

```bash
# 方法 1: git clone
git clone <your-repo> /opt/blog
cd /opt/blog

# 方法 2: scp 上传
scp -r ./* root@<your-ecs-ip>:/opt/blog
```

### 3. 构建并启动

```bash
cd /opt/blog

# 构建镜像
make docker-build

# 启动服务
make docker-up

# 查看日志
make docker-logs
```

### 4. 停止/重启

```bash
make docker-down      # 停止
make docker-restart   # 重启
```

---

## 方案二：systemd 部署

### 1. 安装依赖

```bash
# 更新源
apt update

# 安装 Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
apt install -y nodejs

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 安装 Nginx
apt install -y nginx
```

### 2. 上传并构建

```bash
cd /opt/blog

# 构建
make deploy-build

# 配置 systemd
make deploy-setup
```

### 3. 启动服务

```bash
make deploy-start

# 查看状态
make deploy-status

# 查看日志
make deploy-logs
```

---

## 域名和 HTTPS（可选）

### 1. 域名解析

在阿里云 DNS 控制台添加 A 记录：
- 主机记录：`blog` 或 `@`
- 记录值：`<ECS 公网 IP>`

### 2. 申请 SSL 证书

```bash
# 安装 certbot
apt install certbot python3-certbot-nginx -y

# 申请证书
certbot --nginx -d blog.yourdomain.com
```

### 3. 自动续期

```bash
# 添加定时任务
crontab -e
# 添加：0 3 1 * * certbot renew --quiet
```

---

## 数据库备份

```bash
# 手动备份
cp /opt/blog/data/blog.db /opt/blog/backup/blog-$(date +%Y%m%d).db

# 定时备份（cron）
0 2 * * * cp /opt/blog/data/blog.db /opt/blog/backup/blog-$(date +\%Y\%m\%d).db
```

---

## 常用命令

```bash
# Docker 方案
docker-compose ps          # 查看状态
docker-compose logs -f     # 查看日志
docker-compose restart     # 重启

# systemd 方案
systemctl status blog      # 查看状态
journalctl -u blog -f      # 查看日志
systemctl restart blog     # 重启
```

---

## 故障排查

### 后端无法启动
```bash
# 检查端口占用
netstat -tlnp | grep 3001

# 查看日志
tail -f /opt/blog/logs/api.error.log
```

### 前端无法访问
```bash
# 检查 Nginx 配置
nginx -t

# 检查端口
netstat -tlnp | grep 80
```

### 数据库问题
```bash
# 检查数据库文件
ls -la /opt/blog/data/

# 重置数据库（谨慎操作）
rm /opt/blog/data/blog.db
# 重启服务会自动创建
```
