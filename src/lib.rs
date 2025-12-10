//! D3D9 Hook with ImGui integration for Rust
//! 
//! This library provides functionality to hook DirectX 9 functions
//! and render ImGui menus in games using the hudhook library.

use hudhook::hooks::dx9::ImguiDx9Hooks;
use hudhook::ImguiRenderLoop;
use hudhook::hudhook;
use imgui::*;

/// Main render loop for ImGui
pub struct D3D9RenderLoop {
    display_menu: bool,
    last_insert_state: bool,
}

impl Default for D3D9RenderLoop {
    fn default() -> Self {
        Self {
            display_menu: true,
            last_insert_state: false,
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
        
        // Note: Unloading with END key would require additional setup
        // as hudhook manages the lifecycle differently
    }
}

// Register the hook for DirectX 9
// Note: The macro creates the instance internally using Default::default()
hudhook!(ImguiDx9Hooks, D3D9RenderLoop::default());
