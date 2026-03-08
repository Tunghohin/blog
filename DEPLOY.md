# 阿里云 ECS 部署指南

## 前置准备

### 1. ECS 实例配置
- **系统**: Ubuntu 22.04 LTS
- **配置**: 2 核 4G 或以上
- **安全组**: 开放端口 80 (HTTP), 443 (HTTPS), 22 (SSH)

### 2. GitHub Secrets 配置

在 GitHub 仓库 Settings → Secrets and variables → Actions 中添加：

| Secret 名称 | 值 |
|-----------|-----|
| `ECS_HOST` | ECS 公网 IP，如 `1.2.3.4` |
| `ECS_USER` | SSH 用户名，如 `root` |
| `ECS_SSH_KEY` | SSH 私钥（见下方生成步骤）|

### 3. 生成 SSH 密钥（可选）

```bash
# 生成专用密钥
ssh-keygen -t ed25519 -f ~/.ssh/github-ecs -N ""

# 复制公钥到 ECS
ssh-copy-id -i ~/.ssh/github-ecs.pub root@<ECS-IP>

# 添加私钥到 GitHub Secrets
cat ~/.ssh/github-ecs | pbcopy  # macOS
# 或
cat ~/.ssh/github-ecs | xclip -selection clipboard  # Linux
```

---

## 部署流程

### 第一步：ECS 初始化（只需一次）

SSH 登录 ECS：
```bash
ssh root@<ECS-IP>
```

安装运行环境：
```bash
# 更新系统
apt update && apt upgrade -y

# 安装 Nginx
apt install -y nginx

# 创建目录
mkdir -p /opt/blog

# 设置权限
chown -R root:root /opt/blog
chmod -R 755 /opt/blog
```

### 第二步：手动拉取代码（首次）

```bash
cd /opt
git clone https://github.com/你的用户名/你的仓库.git blog
chown -R root:root /opt/blog
```

### 第三步：配置 GitHub Actions（自动部署）

每次 `git push` 后自动：
1. GitHub Actions 编译后端和前端
2. 上传编译产物到 ECS
3. 自动重启服务

查看构建状态：https://github.com/你的用户名/你的仓库/actions

---

## 手动部署（可选）

如果不想用 GitHub Actions，可以手动：

### 本地构建 + 上传

```bash
# 本地执行（需要 Rust + Node.js）
ECS_HOST=1.2.3.4 ./scripts/build-and-deploy.sh

# SSH 到 ECS 执行后续命令
ssh root@1.2.3.4
```

### ECS 上执行

```bash
cd /opt/blog
tar -xzf /tmp/blog-release.tar.gz -C /opt/blog

# 配置 systemd
cp /opt/blog/deploy/blog-api.service /etc/systemd/system/
systemctl daemon-reload
systemctl enable blog-api
systemctl start blog-api

# 配置 Nginx
cp /opt/blog/deploy/nginx.conf /etc/nginx/nginx.conf
systemctl restart nginx
```

---

## 验证部署

```bash
# 检查服务状态
systemctl status blog-api
systemctl status nginx

# 测试 API
curl http://localhost/api/posts

# 注册管理员
curl -X POST http://localhost/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "你的密码"}'
```

访问 `http://<ECS 公网 IP>` 查看博客首页。

---

## 常用命令

### 服务管理
```bash
systemctl start blog-api    # 启动
systemctl stop blog-api     # 停止
systemctl restart blog-api  # 重启
systemctl status blog-api   # 状态
```

### 日志查看
```bash
# API 日志
journalctl -u blog-api -f

# Nginx 日志
tail -f /var/log/nginx/access.log
tail -f /var/log/nginx/error.log
```

### 数据库备份
```bash
# 手动备份
cp /opt/blog/backend/blog.db /opt/blog/backups/blog-$(date +%Y%m%d).db

# 定时备份（每周日凌晨 3 点）
crontab -e
# 添加：0 3 * * 0 cp /opt/blog/backend/blog.db /opt/blog/backups/blog-$(date +\%Y\%m\%d).db
```

---

## 故障排查

### 后端无法启动
```bash
# 检查端口占用
netstat -tlnp | grep 3001

# 查看日志
journalctl -u blog-api -n 50
```

### Nginx 无法访问
```bash
# 检查配置
nginx -t

# 重启 Nginx
systemctl restart nginx

# 检查端口
netstat -tlnp | grep 80
```

### GitHub Actions 失败
- 查看 Actions 日志：https://github.com/你的用户名/你的仓库/actions
- 常见问题：
  - SSH 密钥权限不对 → `chmod 600 ~/.ssh/github-ecs`
  - ECS_HOST 为空 → 检查 Secrets 配置
  - 磁盘空间不足 → `df -h`
