//! D3D9 Hook with ImGui integration for Rust
//! 
//! This library provides functionality to hook DirectX 9 functions
//! and render ImGui menus in games using the hudhook library.

use hudhook::hooks::dx9::ImguiDx9Hooks;
use hudhook::ImguiRenderLoop;
use hudhook::hudhook;
use imgui::*;
use std::sync::Once;

#[cfg(windows)]
use windows::{
    Win32::System::LibraryLoader::{FreeLibraryAndExitThread, GetModuleHandleW},
    Win32::Foundation::HMODULE,
};

/// 初始化控制台（自动调用，确保只初始化一次）
static INIT_CONSOLE: Once = Once::new();

/// 启用控制台
/// 
/// 分配控制台窗口并启用颜色支持
/// 参考示例：l4d2-internal-base/l4d2-rust/src/imgui_render.rs
fn init_console() {
    INIT_CONSOLE.call_once(|| {
        // 分配控制台
        match hudhook::alloc_console() {
            Ok(_) => {
                // 启用控制台颜色支持
                hudhook::enable_console_colors();
                tracing::info!("✅ 控制台已启用");
            }
            Err(e) => {
                // 如果控制台已经存在，这是正常的
                eprintln!("控制台分配警告: {:?} (可能已经存在)", e);
            }
        }
    });
}

/// Main render loop for ImGui
pub struct D3D9RenderLoop {
    display_menu: bool,
    last_insert_state: bool,
    last_end_state: bool,
}

impl Default for D3D9RenderLoop {
    fn default() -> Self {
        // 初始化控制台
        init_console();
        
        Self {
            display_menu: true,
            last_insert_state: false,
            last_end_state: false,
        }
    }
}

impl ImguiRenderLoop for D3D9RenderLoop {
    fn render(&mut self, ui: &mut Ui) {
        // Handle keyboard input
        self.handle_input(ui);
        
        // Render menu if enabled
        if self.display_menu {
            ui.window("Menu Window Title")
                .size([500.0, 300.0], Condition::Once)
                .build(|| {
                    ui.text("Draw your menu here.");
                    
                    // Example: Add some ImGui widgets
                    if ui.button("Button") {
                        // Handle button click
                    }
                    
                    ui.separator();
                    
                    // Example: Checkbox
                    let mut enabled = true;
                    ui.checkbox("Enable Feature", &mut enabled);
                });
        }
    }
}

impl D3D9RenderLoop {
    /// Handle keyboard input
    fn handle_input(&mut self, ui: &mut Ui) {
        let io = ui.io();
        
        // 使用 ImGui IO 检测 Insert 键
        // 参考 imgui-sys: ImGuiKey_Insert = 521
        let insert_pressed = io.keys_down[imgui::sys::ImGuiKey_Insert as usize];
        
        tracing::info!("insert_pressed: {}", insert_pressed);
        
        // Toggle menu with INSERT key
        // Detect key press (transition from not pressed to pressed)
        if insert_pressed && !self.last_insert_state {
            self.display_menu = !self.display_menu;
        }
        self.last_insert_state = insert_pressed;
        
        // 使用 ImGui IO 检测 End 键
        // 参考 imgui-sys: ImGuiKey_End = 522
        let end_pressed = io.keys_down[imgui::sys::ImGuiKey_End as usize];
        
        // Unload DLL with END key
        // Detect key press (transition from not pressed to pressed)
        if end_pressed && !self.last_end_state {
            tracing::info!("END key pressed, unloading DLL...");
            self.unload_dll();
        }
        self.last_end_state = end_pressed;
    }
    
    /// Unload the DLL from the target process
    #[cfg(windows)]
    fn unload_dll(&self) {
        unsafe {
            // Get the current module handle
            if let Ok(module_handle) = GetModuleHandleW(None) {
                // Extract the raw pointer value as usize to safely pass between threads
                let module_ptr_value = module_handle.0 as usize;
                
                // Create a thread to unload the DLL
                // FreeLibraryAndExitThread will unload the DLL and exit the thread
                std::thread::spawn(move || {
                    // Convert back to raw pointer and then to HMODULE
                    let module_ptr = module_ptr_value as *mut std::ffi::c_void;
                    FreeLibraryAndExitThread(HMODULE(module_ptr), 0);
                });
            } else {
                tracing::error!("Failed to get module handle for unloading");
            }
        }
    }
    
    #[cfg(not(windows))]
    fn unload_dll(&self) {
        tracing::warn!("DLL unloading is only supported on Windows");
    }
}

// Register the hook for DirectX 9
// Note: The macro creates the instance internally using Default::default()
hudhook!(ImguiDx9Hooks, D3D9RenderLoop::default());
