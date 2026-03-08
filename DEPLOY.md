# 阿里云 ECS 部署指南

## 前置准备

### 1. ECS 实例配置
- **系统**: Ubuntu 22.04 LTS
- **配置**: 2 核 4G 或以上
- **安全组**: 开放端口 80 (HTTP), 443 (HTTPS), 22 (SSH)

---

## 第一步：生成 SSH 密钥

在**本地电脑**执行：

```bash
# 生成专用密钥（不设置密码）
ssh-keygen -t ed25519 -f ~/.ssh/github-ecs -N "" -C "github-actions-to-ecs"

# 查看生成的文件
ls -la ~/.ssh/github-ecs*
```

输出：
- `~/.ssh/github-ecs` — 私钥（给 GitHub Actions 用）
- `~/.ssh/github-ecs.pub` — 公钥（给 ECS 用）

---

## 第二步：配置 ECS（复制公钥）

```bash
# 方式 1：自动复制（推荐）
ssh-copy-id -i ~/.ssh/github-ecs.pub root@<ECS-IP>

# 方式 2：手动复制
# 在本地执行：
cat ~/.ssh/github-ecs.pub
# 复制输出内容，然后 SSH 到 ECS 执行：
echo "<粘贴公钥内容>" >> ~/.ssh/authorized_keys
chmod 600 ~/.ssh/authorized_keys
```

测试密钥登录：
```bash
ssh -i ~/.ssh/github-ecs root@<ECS-IP>
```

---

## 第三步：配置 GitHub Secrets

1. 打开 GitHub 仓库页面
2. 点击 **Settings** → **Secrets and variables** → **Actions**
3. 点击 **New repository secret**
4. 添加以下 3 个 Secrets：

| Secret 名称 | 值 | 获取方式 |
|-----------|-----|---------|
| `ECS_HOST` | ECS 公网 IP | 阿里云控制台查看 |
| `ECS_USER` | `root` | 固定值 |
| `ECS_SSH_KEY` | 私钥内容 | 见下方 |

### 获取私钥内容

**macOS:**
```bash
cat ~/.ssh/github-ecs | pbcopy
echo "私钥已复制到剪贴板"
```

**Linux:**
```bash
cat ~/.ssh/github-ecs | xclip -selection clipboard
echo "私钥已复制到剪贴板"
```

**Windows (Git Bash):**
```bash
cat ~/.ssh/github-ecs | clip
echo "私钥已复制到剪贴板"
```

然后粘贴到 GitHub 的 Value 输入框中。

![添加 GitHub Secret](https://docs.github.com/assets/cb-72088/mw-1440/images/help/repository/actions-secret-new.webp)

---

## 第四步：ECS 初始化（只需一次）

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

---

## 第五步：拉取代码（首次）

```bash
cd /opt
git clone https://github.com/你的用户名/你的仓库.git blog
chown -R root:root /opt/blog
```

---

## 第六步：触发自动部署

```bash
# 本地执行
git add .
git commit -m "trigger deploy"
git push
```

然后打开 GitHub Actions 页面查看部署进度：
https://github.com/你的用户名/你的仓库/actions

部署完成后，访问 `http://<ECS 公网 IP>` 查看博客首页。

---

## 验证部署

```bash
# SSH 到 ECS
ssh root@<ECS-IP>

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
# 添加：0 3 * 0 cp /opt/blog/backend/blog.db /opt/blog/backups/blog-$(date +\%Y\%m\%d).db
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

**查看 Actions 日志：**
https://github.com/你的用户名/你的仓库/actions

**常见问题：**

| 错误 | 原因 | 解决方法 |
|-----|------|---------|
| `Permission denied (publickey)` | SSH 密钥配置错误 | 检查公钥是否正确复制到 ECS |
| `Connection timed out` | ECS_HOST 错误或安全组未开放 | 检查 IP 和安全组规则 |
| `No space left on device` | 磁盘空间不足 | `df -h` 检查，清理空间 |
| `systemctl: command not found` | 系统不是 Ubuntu/Debian | 改用其他初始化方式 |

**快速测试 SSH 连接：**
```bash
# 在本地执行
ssh -i ~/.ssh/github-ecs root@<ECS-IP> "echo SSH works"
```

---

## 快速脚本（可选）

一键完成 SSH 密钥配置：

```bash
# 本地执行
./scripts/setup-ssh.sh 1.2.3.4 root
```

脚本会自动：
1. 生成 SSH 密钥对
2. 复制公钥到 ECS
3. 测试连接
4. 显示需要添加到 GitHub 的私钥内容
