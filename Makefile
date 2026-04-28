.PHONY: help build build-release test test-core test-cli clean demo demo-all install run

# 默认目标：显示帮助信息
help:
	@echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@echo "🎨 LiteMark 常用命令"
	@echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@echo ""
	@echo "📦 构建命令："
	@echo "  make build          - 编译 debug 版本"
	@echo "  make build-release  - 编译 release 版本（优化）"
	@echo "  make install        - 安装到系统（需要 sudo）"
	@echo ""
	@echo "🧪 测试命令："
	@echo "  make test           - 运行所有测试"
	@echo "  make test-core      - 仅测试 litemark-core"
	@echo "  make test-cli       - 仅测试 litemark-cli"
	@echo ""
	@echo "🖼️  Demo 生成："
	@echo "  make demo           - 生成所有模板的演示图片"
	@echo "  make demo-classic   - 生成 classic 模板演示"
	@echo "  make demo-compact   - 生成 compact 模板演示"
	@echo "  make demo-professional - 生成 professional 模板演示"
	@echo ""
	@echo "🧹 清理命令："
	@echo "  make clean          - 清理构建产物"
	@echo "  make clean-demo     - 清理 demo 输出"
	@echo ""
	@echo "🚀 快捷命令："
	@echo "  make run            - 快速运行 CLI（显示帮助）"
	@echo "  make templates      - 列出所有可用模板"
	@echo ""
	@echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# 构建命令
build:
	@echo "🔨 编译 debug 版本..."
	cargo build --workspace

build-release:
	@echo "🚀 编译 release 版本（优化）..."
	cargo build --workspace --release
	@echo "✅ 构建完成！二进制文件位置："
	@echo "   ./target/release/litemark-cli"

# 测试命令
test:
	@echo "🧪 运行所有测试..."
	cargo test --workspace

test-core:
	@echo "🧪 测试 litemark-core..."
	cargo test -p litemark-core --lib

test-cli:
	@echo "🧪 测试 litemark-cli..."
	cargo test -p litemark-cli --lib

# 测试套件命令
test-suite:
	@echo "🧪 运行完整测试套件..."
	cargo test -p litemark-test-suite

test-unit:
	@echo "🧪 运行单元测试..."
	cargo test -p litemark-test-suite --test unit -- --test-threads=8

test-integration:
	@echo "🧪 运行集成测试..."
	cargo test -p litemark-test-suite --test integration

test-e2e:
	@echo "🧪 运行 E2E 测试..."
	cargo test -p litemark-test-suite --test e2e

# 生成视觉报告
visual-report:
	@echo "📊 生成视觉报告..."
	cargo run -p litemark-test-suite --bin generate-report
	@echo "✅ 报告位置: target/test-reports/latest/index.html"

# 生成测试图片
generate-test-images:
	@echo "🖼️ 生成测试图片..."
	cargo run -p litemark-test-suite --bin generate-test-images

# 健康检查
health-check:
	@echo "🏥 运行健康检查..."
	cargo run -p litemark-test-suite --bin health-check

# 安装到系统
install: build-release
	@echo "📦 安装到系统..."
	cargo install --path litemark-cli
	@echo "✅ 安装完成！现在可以直接使用 'litemark-cli' 命令"

# 清理命令
clean:
	@echo "🧹 清理构建产物..."
	cargo clean

clean-demo:
	@echo "🧹 清理 demo 输出..."
	rm -rf output/demo
	@echo "✅ Demo 输出已清理"

# Demo 生成
DEMO_INPUT := test_images/demos/DSC09787.JPG
DEMO_LOGO := assets/logos/test_logo_peter.png
DEMO_AUTHOR := "Peter Chen"
DEMO_OUTPUT_DIR := output/demo

# 模板列表
TEMPLATES := classic compact professional

demo: build-release
	@echo "🖼️  生成所有模板的演示图片..."
	@mkdir -p $(DEMO_OUTPUT_DIR)
	@for template in $(TEMPLATES); do \
		echo ""; \
		echo "生成 $$template 模板..."; \
		cargo run -p litemark-cli --release -- add \
			-i $(DEMO_INPUT) \
			-o $(DEMO_OUTPUT_DIR)/$$template.jpg \
			-t $$template \
			--logo $(DEMO_LOGO) \
			--author $(DEMO_AUTHOR); \
	done
	@echo ""
	@echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@echo "✅ 所有演示图片已生成到: $(DEMO_OUTPUT_DIR)/"
	@echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@ls -lh $(DEMO_OUTPUT_DIR)/

# 单个模板 demo（动态生成规则）
.PHONY: $(addprefix demo-,$(TEMPLATES))
$(addprefix demo-,$(TEMPLATES)): demo-%: build-release
	@mkdir -p $(DEMO_OUTPUT_DIR)
	@echo "🖼️  生成 $* 模板演示..."
	@cargo run -p litemark-cli --release -- add \
		-i $(DEMO_INPUT) -o $(DEMO_OUTPUT_DIR)/$*.jpg \
		-t $* --logo $(DEMO_LOGO) --author $(DEMO_AUTHOR)
	@echo "✅ 输出: $(DEMO_OUTPUT_DIR)/$*.jpg"

# 快捷命令
run: build-release
	@cargo run -p litemark-cli --release -- --help

templates: build-release
	@echo "📋 可用模板列表："
	@cargo run -p litemark-cli --release -- templates

# 批量处理示例
batch-example: build-release
	@echo "📦 批量处理示例..."
	@mkdir -p output/batch
	@cargo run -p litemark-cli --release -- batch \
		-i test_images \
		-o output/batch \
		-t classic \
		--logo $(DEMO_LOGO) \
		--author $(DEMO_AUTHOR) \
		--concurrency 4
	@echo "✅ 批量处理完成: output/batch/"
