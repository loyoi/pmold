# Project Mold 项目摸具

在 git 仓库中，添加 guide.toml 文件，克隆 git 仓库后，自动根据指南文件修改仓库里面的文件

## 示例

```toml

# questions:交互式问答
[[questions]]
key = "app_name"
prompt = "请输入应用名称"
type = "string"
default = "My App"

[[questions]]
key = "package_manager"
prompt = "请选择包管理器"
type = { select = { options = ["npm", "pnpm", "bun", "yarn"] } }
default = "bun"

[[files]]
path = "src-tauri/tauri.conf.json"
replacements = [
    { search = '"productName": ".*?"', replace = '"productName": "{{app_name}}"', is_regex = true },
    { search = '"version": ".*?"', replace = '"version": "1.0.0"', is_regex = true },
    { search = '"beforeDevCommand": ".*?"', replace = '"beforeDevCommand": "{{package_manager}} run dev"', is_regex = true },
    { search = '"beforeBuildCommand": ".*?"', replace = '"beforeBuildCommand": "{{package_manager}} run build"', is_regex = true }
]

[[files]]
path = "package.json"
replacements = [
    { search = '"name": ".*?"', replace = '"name": "{{app_name}}"', is_regex = true }
]

```
