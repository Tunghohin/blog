.PHONY: help backend frontend start stop clean dev build deploy-setup deploy-start deploy-stop deploy-status deploy-logs

# 默认目标
help:
	@echo "个人博客系统 - Makefile 命令"
	@echo ""
	@echo "=== 开发命令 ==="
	@echo "  make backend      - 启动后端服务 (端口 3001)"
	@echo "  make frontend     - 启动前端服务 (端口 5174)"
	@echo "  make start        - 同时启动前后端服务"
	@echo "  make stop         - 停止所有服务"
	@echo "  make dev          - 启动开发环境 (同时启动前后端)"
	@echo "  make clean        - 清理构建文件"
	@echo ""
	@echo "=== 构建命令 ==="
	@echo "  make build        - 构建项目 (后端 + 前端)"
	@echo "  make build-backend    - 只构建后端"
	@echo "  make build-frontend   - 只构建前端"
	@echo ""
	@echo "=== 生产部署 ==="
	@echo "  make deploy-setup     - 配置 systemd 服务 (需要 sudo)"
	@echo "  make deploy-start     - 启动生产服务"
	@echo "  make deploy-stop      - 停止生产服务"
	@echo "  make deploy-status    - 查看服务状态"
	@echo "  make deploy-logs      - 查看服务日志"
	@echo ""

# 启动后端
backend:
	@echo "启动后端服务..."
	@cd backend && cargo run --release 2>&1 &
	@echo "后端已启动：http://localhost:3001"

# 启动前端
frontend:
	@echo "启动前端服务..."
	@cd frontend && npm run dev 2>&1 &
	@echo "前端已启动：http://localhost:5174"

# 同时启动前后端
start: backend
	@sleep 2
	@$(MAKE) frontend
	@echo ""
	@echo "=========================================="
	@echo "服务已启动!"
	@echo "  前端：http://localhost:5174"
	@echo "  后端：http://localhost:3001"
	@echo "=========================================="

# 开发模式（后台运行）
dev:
	@echo "启动开发环境..."
	@mkdir -p logs
	@cd backend && nohup cargo run --release > ../logs/backend.log 2>&1 & echo $$! > ../logs/backend.pid
	@cd frontend && nohup npm run dev > ../logs/frontend.log 2>&1 & echo $$! > ../logs/frontend.pid
	@sleep 3
	@echo "开发环境已启动!"
	@echo "  前端：http://localhost:5174"
	@echo "  后端：http://localhost:3001"
	@echo "  日志：logs/backend.log, logs/frontend.log"

# 停止所有服务
stop:
	@echo "停止所有服务..."
	@pkill -f "blog-backend" || true
	@pkill -f "vite" || true
	@pkill -f "node.*dev" || true
	@rm -f logs/*.pid 2>/dev/null || true
	@echo "所有服务已停止"

# 重启服务
restart: stop start

# 构建项目
build: build-backend build-frontend

build-backend:
	@echo "构建后端..."
	@cd backend && cargo build --release

build-frontend:
	@echo "构建前端..."
	@cd frontend && npm run build

# 清理
clean:
	@echo "清理构建文件..."
	@cd backend && cargo clean
	@cd frontend && rm -rf node_modules dist
	@rm -rf logs/*.log logs/*.pid 2>/dev/null || true
	@rm -f backend/blog.db 2>/dev/null || true
	@echo "清理完成"

# 查看日志
logs-backend:
	@tail -f logs/backend.log 2>/dev/null || echo "日志文件不存在"

logs-frontend:
	@tail -f logs/frontend.log 2>/dev/null || echo "日志文件不存在"

# 检查服务状态
status:
	@echo "服务状态:"
	@ps aux | grep -E "blog-backend|vite|node.*dev" | grep -v grep || echo "没有运行的服务"

# 初始化数据库（删除旧数据库）
init-db:
	@echo "初始化数据库..."
	@rm -f backend/blog.db
	@echo "数据库已重置"

# ==================== 生产部署 (systemd) ====================

deploy-build:
	@echo "构建生产版本..."
	@cd frontend && npm ci && npm run build
	@cd backend && cargo build --release
	@echo "构建完成!"

deploy-setup: deploy-build
	@echo "配置 systemd 服务..."
	@sudo mkdir -p /opt/blog/{bin,data,logs}
	@sudo cp backend/target/release/blog-backend /opt/blog/bin/
	@sudo cp -r frontend/dist /opt/blog/html
	@sudo cp deploy/blog-api.service /etc/systemd/system/
	@sudo systemctl daemon-reload
	@echo "systemd 服务已配置!"
	@echo "运行 'make deploy-start' 启动服务"

deploy-start:
	@echo "启动生产服务..."
	@sudo systemctl enable blog-api
	@sudo systemctl start blog-api
	@echo "服务已启动!"

deploy-stop:
	@echo "停止生产服务..."
	@sudo systemctl stop blog-api

deploy-status:
	@sudo systemctl status blog-api

deploy-logs:
	@sudo journalctl -u blog-api -f
