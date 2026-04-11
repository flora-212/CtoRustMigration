# Loom Converter - Module Structure

## 概述

`loom_converter` 是一个用于将 Rust 代码转换为 Loom 并发测试的工具包。它已从原单个大文件 (`loom_converter.py`) 重构为多个专用模块。

## 模块结构

```
loom_converter/
├── __init__.py              # 包初始化，导出公共 API
├── __main__.py              # 支持 python -m loom_converter 运行
├── cli.py                   # 命令行接口和主文件转换函数
├── converter.py             # 核心转换逻辑 (main → loom::model)
├── primitives.py            # 并发原语替换函数
├── functions.py             # 函数签名和调用处理
├── state_gen.py             # State 结构体生成
├── statics.py               # 全局静态变量处理
└── utils.py                 # 通用工具函数
```

## 各模块职责

### cli.py
- `convert_file()` - 主转换入口函数
- `add_loom_dependency()` - 向 Cargo.toml 添加 loom 依赖
- `main()` - 命令行程序入口

### converter.py
- `convert_main_to_loom_model()` - 将 main() 转换为 loom::model 测试
- 处理全局静态变量的 State 结构体创建
- 函数签名更新和调用修改
- OnceLock 变量处理

### primitives.py
- `replace_concurrency_primitives()` - 替换 std::sync 为 loom::sync
- `wrap_unsafe_function_calls()` - 在 unsafe 块中包装不安全函数调用
- `wrap_libc_calls()` - 处理 libc 调用
- `clone_globals_in_loops()` - 在循环中克隆全局变量

### functions.py
- `update_function_signature()` - 更新函数签名以接受新参数
- `update_function_calls()` - 修改函数调用以传递参数
- `replace_global_var_access()` - 将全局变量访问替换为 state.var

### state_gen.py
- `generate_state_struct()` - 为全局静态变量生成 State 结构体定义
- `generate_state_initialization()` - 生成 State 初始化代码

### statics.py
- `find_all_global_statics()` - 查找所有全局静态变量
- `find_once_init_statics()` - 查找 OnceLock/Once 类型的变量

### utils.py
- `extract_function_body()` - 提取函数体
- `extract_once_init_closure()` - 提取 OnceLock 初始化闭包
- `detect_array_size_and_element_type()` - 检测数组类型和大小
- `convert_array_to_fixed_array()` - 转换数组初始化形式
- `find_functions_using_var()` - 查找使用特定变量的所有函数

## 使用方法

### 作为库导入

```python
from loom_converter import convert_file

# 转换 Rust 文件
convert_file('input.rs', 'output.rs', example_dir='./example', standalone=True)
```

### 命令行使用

```bash
# 直接运行
python -m loom_converter input.rs output.rs --standalone --example-dir ./example

# 或通过兼容性包装脚本
cd validation/
python loom_converter.py input.rs output.rs --standalone --example-dir ../example
```

## 向后兼容性

在 `validation/` 目录中保留了 `loom_converter.py` 兼容性包装脚本，它会自动导入新的 `loom_converter` 包并重导出所有公共 API。这确保了旧代码能继续正常工作。

## 导入路径

### 新方式 (推荐)
```python
from loom_converter import convert_file
```

### 旧方式 (仍然支持)
```python
from validation.loom_converter import convert_file
```

## 文件转换流程

1. **读取源文件** - 加载输入的 Rust 代码
2. **替换并发原语** - std::sync → loom::sync, std::thread → loom::thread
3. **分析全局变量** - 识别所有静态全局变量
4. **生成 State 结构体** - 将全局变量包装入结构体
5. **更新函数签名** - 添加必要的参数
6. **转换主函数** - main() → #[test] fn test_concurrent_access()
7. **包装为 loom::model** - 创建并发模型测试
8. **清理导入** - 移除不再使用的类型导入

## 扩展

若要添加新功能，建议：
- 工具函数 → `utils.py`
- 变量处理 → `statics.py`
- 转换逻辑 → 新建独立模块或添加到 `converter.py`
- 命令行选项 → `cli.py`

## 相关链接

- Loom: https://github.com/tokio-rs/loom
- 原始单文件: 已拆分为上述模块结构
