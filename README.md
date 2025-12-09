# D3D9 Hook ImGui (Rust)

这是一个用 Rust 实现的 DirectX 9 Hook 项目，使用 `hudhook` 库实现与 C++ 版本相同的功能。

## 功能特性

- ✅ Hook DirectX 9 的渲染函数
- ✅ 使用 ImGui 绘制菜单界面
- ✅ 窗口大小调整支持
- ✅ 全屏切换支持
- ✅ 键盘输入处理（INSERT 切换菜单）
- ✅ 自动处理设备重置

## 项目结构

```
d3d9-hook-imgui-rs/
├── Cargo.toml          # 项目配置和依赖
├── README.md           # 本文件
└── src/
    └── lib.rs          # 主实现文件
```

## 依赖说明

- `hudhook` - 用于在游戏中创建 ImGui 覆盖层的库，支持 DirectX 9/11/12 和 OpenGL
- `imgui` - Rust 的 ImGui 绑定

## 编译

```bash
cargo build --release
```

编译后会生成 `d3d9_hook_imgui_rs.dll`。

## 使用方法

### 方法 1: 使用 hudhook 注入器

创建一个注入器程序（`src/bin/injector.rs`）：

```rust
use hudhook::inject::Process;

fn main() {
    let dll_path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("d3d9_hook_imgui_rs.dll");
    
    Process::by_name("YourGame.exe")
        .unwrap()
        .inject(dll_path)
        .unwrap();
    
    println!("DLL injected successfully!");
}
```

### 方法 2: 手动注入

使用任何 DLL 注入工具（如 Extreme Injector、Xenos Injector 等）将生成的 DLL 注入到目标进程。

## 键盘快捷键

- `INSERT` - 切换菜单显示/隐藏

## 自定义菜单

在 `src/lib.rs` 的 `render` 方法中添加你的 ImGui 代码：

```rust
impl ImguiRenderLoop for D3D9RenderLoop {
    fn render(&mut self, ui: &mut Ui) {
        if self.display_menu {
            ui.window("My Menu")
                .size([400.0, 300.0], Condition::Once)
                .build(|| {
                    // 添加你的 ImGui 控件
                    ui.text("Hello from Rust!");
                    
                    if ui.button("Click me") {
                        // 处理按钮点击
                    }
                });
        }
    }
}
```

## 与 C++ 版本的对比

| 功能 | C++ 版本 | Rust 版本 |
|------|---------|----------|
| Hook 机制 | MS Detours | hudhook |
| DirectX 9 | ✅ 完整支持 | ✅ 完整支持 |
| ImGui | ✅ 完整集成 | ✅ 完整集成 |
| 窗口处理 | ✅ 完整支持 | ✅ 自动处理 |
| 内存安全 | ⚠️ 手动管理 | ✅ 编译器保证 |
| 代码复杂度 | 中等 | 低（使用库） |

## 优势

1. **更安全**: Rust 的内存安全保证
2. **更简单**: `hudhook` 库处理了大部分底层细节
3. **跨平台**: 支持 Windows 和 Wine/Proton
4. **易维护**: 代码更简洁，易于理解和修改

## 注意事项

1. **目标进程**: 确保目标游戏使用 DirectX 9
2. **权限**: 注入 DLL 可能需要管理员权限
3. **防作弊**: 某些游戏可能有反作弊系统，使用需谨慎
4. **测试**: 建议在测试环境中充分测试

## 开发建议

1. 使用 `cargo check` 检查编译错误
2. 使用 `cargo clippy` 检查代码质量
3. 使用 `cargo fmt` 格式化代码
4. 查看 [hudhook 文档](https://docs.rs/hudhook/) 了解更多功能

## 许可证

与原始 C++ 项目保持一致。

## 参考资源

- [hudhook GitHub](https://github.com/veeenu/hudhook)
- [hudhook 文档](https://docs.rs/hudhook/)
- [imgui-rs 文档](https://docs.rs/imgui/)
