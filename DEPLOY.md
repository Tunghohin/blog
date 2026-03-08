# 阿里云 ECS 部署指南

## 前置准备

- **系统**: Ubuntu 22.04 LTS
- **配置**: 2 核 4G 或以上
- **安全组**: 开放端口 80 (HTTP), 443 (HTTPS), 22 (SSH)

---

## 第一步：生成 SSH 密钥

在本地电脑执行：

```bash
ssh-keygen -t ed25519 -f ~/.ssh/github-ecs -N "" -C "github-actions-to-ecs"
```

生成两个文件：
- `~/.ssh/github-ecs` — 私钥（给 GitHub Actions）
- `~/.ssh/github-ecs.pub` — 公钥（给 ECS）

---

## 第二步：配置 ECS SSH 密钥

```bash
ssh-copy-id -i ~/.ssh/github-ecs.pub root@<ECS-IP>

# 测试登录
ssh -i ~/.ssh/github-ecs root@<ECS-IP>
```

---

## 第三步：配置 GitHub Secrets

进入 GitHub 仓库 **Settings → Secrets and variables → Actions**，添加：

| Secret 名称 | 值 |
|-----------|-----|
| `ECS_HOST` | ECS 公网 IP |
| `ECS_USER` | `root` |
| `ECS_SSH_KEY` | `cat ~/.ssh/github-ecs` 的输出 |
| `JWT_SECRET` | 随机密钥，`openssl rand -hex 32` 生成 |

---

## 第四步：ECS 初始化

SSH 登录 ECS 后执行：

```bash
apt update && apt upgrade -y
apt install -y nginx
mkdir -p /opt/blog /opt/blog/logs
```

---

## 第五步：触发自动部署

```bash
git add .
git commit -m "trigger deploy"
git push
```

查看部署进度：`https://github.com/你的用户名/你的仓库/actions`

---

## 第六步：初始化管理员账号

首次部署后，SSH 到 ECS：

```bash
cd /opt/blog/backend

# 停止服务
systemctl stop blog-api

# 创建管理员
cargo run -- init-db

# 按提示输入用户名和密码

# 重启服务
systemctl start blog-api
```

---

## 验证部署

```bash
# 检查服务状态
systemctl status blog-api

# 测试 API
curl http://localhost/api/posts
```

---

## 常用命令

```bash
# 服务管理
systemctl {start|stop|restart|status} blog-api

# 查看日志
journalctl -u blog-api -f
tail -f /opt/blog/logs/api.log

# 数据库备份
cp /opt/blog/backend/blog.db /opt/blog/backups/blog-$(date +%Y%m%d).db
```

---

## 故障排查

| 错误 | 解决方法 |
|-----|---------|
| `Permission denied (publickey)` | 检查公钥是否正确复制到 ECS |
| `Connection timed out` | 检查 ECS_HOST 和安全组规则 |
| 后端无法启动 | `journalctl -u blog-api -n 50` 查看日志 |
| Nginx 无法访问 | `nginx -t` 检查配置，检查端口 80 占用 |
