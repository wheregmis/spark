//! Application runner and main event loop.

use sikhar_core::{init_wgpu, Color, SurfaceState};
use sikhar_input::{FocusManager, InputEvent, PointerButton};
use sikhar_layout::LayoutTree;
use sikhar_render::{DrawList, Renderer};
use sikhar_text::TextSystem;
use sikhar_widgets::{EventContext, PaintContext, Widget};
use wgpu::{Device, Queue};
use winit::event::WindowEvent;

#[cfg(any(target_os = "macos", target_os = "ios"))]
use sikhar_native_apple::ViewManager;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[cfg(target_arch = "wasm32")]
use web_time::Instant;

/// Application configuration.
pub struct AppConfig {
    /// Window title.
    pub title: String,
    /// Initial window width.
    pub width: u32,
    /// Initial window height.
    pub height: u32,
    /// Background color.
    pub background: Color,
    /// Enable VSync.
    pub vsync: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            title: String::from("Sikhar App"),
            width: 800,
            height: 600,
            background: Color::from_hex(0xF3F4F6),
            vsync: true,
        }
    }
}

/// The main application struct.
pub struct App {
    config: AppConfig,
}

impl App {
    /// Create a new app with default configuration.
    pub fn new() -> Self {
        Self {
            config: AppConfig::default(),
        }
    }

    /// Set the window title.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.config.title = title.into();
        self
    }

    /// Set the initial window size.
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.config.width = width;
        self.config.height = height;
        self
    }

    /// Set the background color.
    pub fn with_background(mut self, color: Color) -> Self {
        self.config.background = color;
        self
    }

    /// Run the application with the given root widget.
    pub fn run<F>(self, build_ui: F) -> !
    where
        F: FnOnce() -> Box<dyn Widget> + 'static,
    {
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        let runner = AppRunner::new(self.config, build_ui);
        let runner_leaked: &'static mut AppRunner<F> = Box::leak(Box::new(runner));
        event_loop.run_app(runner_leaked).unwrap();
        std::process::exit(0);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal application runner that handles the event loop.
struct AppRunner<F: FnOnce() -> Box<dyn Widget>> {
    config: AppConfig,
    build_ui: Option<F>,
    state: Option<AppState>,
}

struct AppState {
    window: &'static dyn winit::window::Window,
    device: Device,
    queue: Queue,
    surface_state: SurfaceState<'static>,
    renderer: Renderer,
    text_system: TextSystem,
    draw_list: DrawList,
    layout_tree: LayoutTree,
    focus_manager: FocusManager,
    root_widget: Box<dyn Widget>,
    start_time: Instant,
    mouse_pos: glam::Vec2,
    scale_factor: f32,
    needs_layout: bool,
    needs_repaint: bool,
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    native_view_manager: Option<ViewManager>,
}

impl<F: FnOnce() -> Box<dyn Widget>> AppRunner<F> {
    fn new(config: AppConfig, build_ui: F) -> Self {
        Self {
            config,
            build_ui: Some(build_ui),
            state: None,
        }
    }

    fn build_layout(&mut self) {
        let state = self.state.as_mut().unwrap();

        // Clear layout tree
        state.layout_tree = LayoutTree::new();

        // Initialize native view manager if needed
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        if state.native_view_manager.is_none() {
            state.native_view_manager = Some(ViewManager::new());
        }

        // Build layout tree from widget tree
        fn add_to_layout(
            widget: &mut dyn Widget,
            tree: &mut LayoutTree,
        ) -> sikhar_layout::WidgetId {
            let style = widget.style();
            let children_ids: Vec<_> = widget
                .children_mut()
                .iter_mut()
                .map(|child| add_to_layout(
                    child.as_mut(),
                    tree,
                ))
                .collect();

            let id = if children_ids.is_empty() {
                tree.new_leaf(style)
            } else {
                tree.new_with_children(style, &children_ids)
            };

            widget.set_id(id);
            id
        }
        
        // Register native widgets after layout tree is built
        // NOTE: This is a limitation - we can't easily detect native widgets from Box<dyn Widget>
        // The registration happens by traversing the widget tree and checking type IDs
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        fn register_native_widgets(
            widget: &mut dyn Widget,
            manager: &mut ViewManager,
        ) {
            use std::any::{Any, TypeId};
            use sikhar_native_apple::{NativeWidgetRegistration, widgets};
            
            let widget_id = widget.id();
            // Get children before we try to downcast (to avoid move issues)
            let children: Vec<*mut dyn Widget> = widget.children_mut().iter_mut().map(|c| c.as_mut() as *mut dyn Widget).collect();
            
            // Try to detect native widgets using is_native() method
            // This is the proper way since we can't downcast Box<dyn Widget>
            if widget.is_native() {
                eprintln!("Found native widget with ID: {:?}", widget_id);
                // Use the register_native method to register the widget
                // This avoids the need for downcasting
                widget.register_native(widget_id, &mut |id, ptr| {
                    use sikhar_native_apple::NativeViewHandle;
                    #[cfg(target_os = "macos")]
                    {
                        let view_handle = NativeViewHandle::AppKit(ptr as *mut objc2::runtime::AnyObject);
                        manager.register_widget(id, view_handle);
                    }
                    #[cfg(target_os = "ios")]
                    {
                        let view_handle = NativeViewHandle::UIKit(ptr as *mut objc2::runtime::AnyObject);
                        manager.register_widget(id, view_handle);
                    }
                });
            }
            
            // Recursively register children
            for child_ptr in children {
                let child: &mut dyn Widget = unsafe { &mut *child_ptr };
                register_native_widgets(child, manager);
            }
        }

        let root_id = add_to_layout(
            state.root_widget.as_mut(),
            &mut state.layout_tree,
        );
        state.layout_tree.set_root(root_id);

        // Compute layout
        // Use surface size - this should be in physical pixels
        // But we need to convert to logical pixels for layout
        let size = state.surface_state.size;
        let logical_width = (size.width as f32) / state.scale_factor;
        let logical_height = (size.height as f32) / state.scale_factor;
        state
            .layout_tree
            .compute_layout(logical_width, logical_height);
        
        // Store logical size for later use
        let window_height_logical = logical_height;

        // Register native widgets and update their layouts
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        if let Some(ref mut manager) = state.native_view_manager {
            // Try to register native widgets
            // Note: This won't work with trait objects - we need a different approach
            // For now, native widgets must be registered manually or through type-specific code
            register_native_widgets(state.root_widget.as_mut(), manager);
            
            use std::collections::HashMap;
            let mut layouts = HashMap::new();
            
            // Collect all layouts for native widgets
            state.layout_tree.traverse(|widget_id, computed, _depth| {
                if manager.get_view(widget_id).is_some() {
                    layouts.insert(widget_id, *computed);
                }
            });
            
            // Update layouts for registered native widgets
            // The layout was computed with logical pixels, so parent_height should be logical too
            manager.update_layouts(
                &layouts,
                window_height_logical,
                state.scale_factor,
            );
        }

        state.needs_layout = false;
        state.needs_repaint = true;
    }

    fn paint(&mut self) {
        let state = self.state.as_mut().unwrap();
        state.draw_list.clear();

        // Get elapsed time for animations
        let elapsed_time = state.start_time.elapsed().as_secs_f32();

        // We need to use raw pointers to pass mutable references through the recursive function
        // This is safe because we control the lifetime and don't alias
        let text_system_ptr = &mut state.text_system as *mut TextSystem;
        let device_ptr = &state.device as *const Device;
        let queue_ptr = &state.queue as *const Queue;

        // Paint widgets (skip native widgets as they render themselves)
        fn paint_widget(
            widget: &dyn Widget,
            layout_tree: &LayoutTree,
            focus: &FocusManager,
            draw_list: &mut DrawList,
            scale_factor: f32,
            text_system_ptr: *mut TextSystem,
            device_ptr: *const Device,
            queue_ptr: *const Queue,
            elapsed_time: f32,
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            native_view_manager: Option<&ViewManager>,
        ) {
            let id = widget.id();
            
            // Skip painting native widgets - they render themselves
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            if let Some(manager) = native_view_manager {
                if manager.get_view(id).is_some() {
                    // This is a native widget, skip GPU painting
                    // Still paint children in case they're not native
                    for child in widget.children() {
                        paint_widget(
                            child.as_ref(),
                            layout_tree,
                            focus,
                            draw_list,
                            scale_factor,
                            text_system_ptr,
                            device_ptr,
                            queue_ptr,
                            elapsed_time,
                            #[cfg(any(target_os = "macos", target_os = "ios"))]
                            Some(manager),
                        );
                    }
                    return;
                }
            }

            if let Some(layout) = layout_tree.get_absolute_layout(id) {
                // SAFETY: We control the lifetime and ensure no aliasing within this function
                let text_system = unsafe { &mut *text_system_ptr };
                let device = unsafe { &*device_ptr };
                let queue = unsafe { &*queue_ptr };

                // Scale layout bounds from logical to physical pixels
                // Layout is computed in logical pixels, but renderer uses physical pixels
                let scaled_layout = sikhar_layout::ComputedLayout::new(
                    sikhar_core::Rect::new(
                        layout.bounds.x * scale_factor,
                        layout.bounds.y * scale_factor,
                        layout.bounds.width * scale_factor,
                        layout.bounds.height * scale_factor,
                    )
                );

                let mut ctx = PaintContext {
                    draw_list,
                    layout: scaled_layout,
                    focus,
                    widget_id: id,
                    scale_factor,
                    text_system,
                    device,
                    queue,
                    elapsed_time,
                };
                widget.paint(&mut ctx);

                // Paint children
                for child in widget.children() {
                    paint_widget(
                        child.as_ref(),
                        layout_tree,
                        focus,
                        ctx.draw_list,
                        scale_factor,
                        text_system_ptr,
                        device_ptr,
                        queue_ptr,
                        elapsed_time,
                        #[cfg(any(target_os = "macos", target_os = "ios"))]
                        native_view_manager,
                    );
                }
            }
        }

        paint_widget(
            state.root_widget.as_ref(),
            &state.layout_tree,
            &state.focus_manager,
            &mut state.draw_list,
            state.scale_factor,
            text_system_ptr,
            device_ptr,
            queue_ptr,
            elapsed_time,
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            state.native_view_manager.as_ref(),
        );

        state.needs_repaint = false;
    }

    fn handle_event(&mut self, event: InputEvent) {
        let state = self.state.as_mut().unwrap();

        // Simple event dispatch - dispatch to all widgets, let them check bounds
        fn dispatch_event(
            widget: &mut dyn Widget,
            layout_tree: &LayoutTree,
            focus_id: Option<sikhar_layout::WidgetId>,
            event: &InputEvent,
        ) -> (sikhar_widgets::EventResponse, Option<sikhar_layout::WidgetId>) {
            let id = widget.id();
            let layout = match layout_tree.get_absolute_layout(id) {
                Some(l) => l,
                None => {
                    return (sikhar_widgets::EventResponse::default(), focus_id);
                }
            };

            // First dispatch to children (bubble up)
            let mut new_focus = focus_id;
            for child in widget.children_mut() {
                let (response, focus) = dispatch_event(child.as_mut(), layout_tree, new_focus, event);
                new_focus = focus;
                if response.handled {
                    return (response, new_focus);
                }
            }

            // Create a temporary focus manager for this dispatch
            let mut temp_focus = FocusManager::new();
            if let Some(fid) = new_focus {
                temp_focus.set_focus(fid);
            }

            let mut ctx = EventContext {
                layout,
                layout_tree,
                focus: &mut temp_focus,
                widget_id: id,
                has_capture: false,
            };

            let response = widget.event(&mut ctx, event);
            
            // Update focus
            if response.request_focus {
                new_focus = Some(id);
            } else if response.release_focus && new_focus == Some(id) {
                new_focus = None;
            }

            (response, new_focus)
        }

        let current_focus = state.focus_manager.focused();
        let (response, new_focus) = dispatch_event(
            state.root_widget.as_mut(),
            &state.layout_tree,
            current_focus,
            &event,
        );

        // Update focus manager
        if let Some(fid) = new_focus {
            state.focus_manager.set_focus(fid);
        } else if current_focus.is_some() && new_focus.is_none() {
            state.focus_manager.clear_focus();
        }

        if response.repaint {
            state.needs_repaint = true;
        }
        if response.relayout {
            state.needs_layout = true;
        }
        
        // Request redraw if we need to repaint or relayout
        if state.needs_repaint || state.needs_layout {
            state.window.request_redraw();
        }
    }
}

impl<F: FnOnce() -> Box<dyn Widget>> winit::application::ApplicationHandler for AppRunner<F> {
    fn can_create_surfaces(&mut self, event_loop: &dyn winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(
                winit::window::WindowAttributes::default()
                    .with_title(&self.config.title)
                    .with_surface_size(winit::dpi::LogicalSize::new(
                        self.config.width,
                        self.config.height,
                    )),
            )
            .expect("create window");

        let window_leaked: &'static mut Box<dyn winit::window::Window> =
            Box::leak(Box::new(window));
        let window: &'static dyn winit::window::Window = &**window_leaked;

        // Initialize wgpu - use pollster on native, web handles this specially
        let (device, queue, surface_state) = pollster::block_on(init_wgpu(window));

        let renderer = Renderer::new(&device, surface_state.config.format);
        let text_system = TextSystem::new(&device);
        let draw_list = DrawList::new();
        let layout_tree = LayoutTree::new();
        let focus_manager = FocusManager::new();

        // Build the UI
        let build_ui = self.build_ui.take().expect("build_ui already called");
        let root_widget = build_ui();

        let scale_factor = window.scale_factor() as f32;

        self.state = Some(AppState {
            window,
            device,
            queue,
            surface_state,
            renderer,
            text_system,
            draw_list,
            layout_tree,
            focus_manager,
            root_widget,
            start_time: Instant::now(),
            mouse_pos: glam::Vec2::ZERO,
            scale_factor,
            needs_layout: true,
            needs_repaint: true,
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            native_view_manager: None,
        });

        // Build initial layout (this registers native widgets)
        self.build_layout();
        
        // Embed native views into the window after layout
        // This must happen after layout so widgets are registered
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        {
            // Get all data we need before any mutable borrows
            let (size, scale_factor, all_layouts) = {
                let state = self.state.as_ref().unwrap();
                let mut layouts = std::collections::HashMap::new();
                state.layout_tree.traverse(|widget_id, computed, _depth| {
                    layouts.insert(widget_id, *computed);
                });
                (state.surface_state.size, state.scale_factor, layouts)
            };
            
            // Now get mutable access to manager
            if let Some(ref mut manager) = self.state.as_mut().unwrap().native_view_manager {
            // Embed native views into window - inline implementation
            use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
            use objc2::runtime::AnyObject;
            use sikhar_native_apple::ffi::appkit::NSView;
            
            #[cfg(target_os = "macos")]
            {
                // Get the raw window handle from winit
                if let Ok(raw_handle) = window.raw_window_handle() {
                    if let RawWindowHandle::AppKit(handle) = raw_handle {
                        unsafe {
                            // Get NSWindow from the handle
                            // The ns_view field contains the content view
                            let content_view_ptr: *mut AnyObject = handle.ns_view.as_ptr() as *mut AnyObject;
                            
                            if !content_view_ptr.is_null() {
                                // Set the content view as the root view in the manager
                                manager.set_root_view(sikhar_native_apple::NativeViewHandle::AppKit(content_view_ptr));
                                
                                // Create content view wrapper
                                let content_view = NSView::from_ptr(content_view_ptr);
                                
                                // Add all registered native views to the content view
                                let view_count = manager.get_all_views().len();
                                for (_widget_id, view_handle) in manager.get_all_views() {
                                    match view_handle {
                                        sikhar_native_apple::NativeViewHandle::AppKit(ptr) => {
                                            // Create NSView wrapper from pointer
                                            let native_view = NSView::from_ptr(*ptr);
                                            content_view.add_subview(&native_view);
                                            native_view.set_visible(true);
                                            native_view.set_wants_layer(true);
                                            // Bring to front to ensure visibility
                                            native_view.bring_to_front();
                                        }
                                        _ => {}
                                    }
                                }
                                
                                eprintln!("Embedded {} native views into window content view", view_count);
                            } else {
                                eprintln!("Warning: Content view is null, cannot embed native views");
                            }
                        }
                    } else {
                        eprintln!("Warning: Window handle is not AppKit type");
                    }
                } else {
                    eprintln!("Warning: Failed to get raw window handle");
                }
            }
            
                // Update layouts again now that views are embedded
                // Filter layouts to only include registered native widgets
                let native_widget_ids: Vec<_> = manager.get_all_views().keys().copied().collect();
                let layouts: std::collections::HashMap<_, _> = all_layouts
                    .into_iter()
                    .filter(|(id, _)| native_widget_ids.contains(id))
                    .collect();
                
                // Convert physical pixels to logical pixels for parent_height
                // The layout was computed with logical pixels, so we need logical height
                let window_height_logical = (size.height as f32) / scale_factor;
                manager.update_layouts(
                    &layouts,
                    window_height_logical,
                    scale_factor,
                );
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &dyn winit::event_loop::ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::SurfaceResized(size) => {
                if let Some(state) = self.state.as_mut() {
                    if size.width > 0 && size.height > 0 {
                        state
                            .surface_state
                            .resize(&state.device, size.width, size.height);
                        state.needs_layout = true;
                    }
                }
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                if let Some(state) = self.state.as_mut() {
                    state.scale_factor = scale_factor as f32;
                    state.needs_layout = true;
                }
            }
            WindowEvent::PointerMoved { position, .. } => {
                // Convert physical pixels to logical pixels for event handling
                let scale_factor = self.state.as_ref().map(|s| s.scale_factor).unwrap_or(1.0);
                let pos = glam::Vec2::new(
                    position.x as f32 / scale_factor,
                    position.y as f32 / scale_factor,
                );
                if let Some(s) = self.state.as_mut() {
                    s.mouse_pos = pos;
                }
                self.handle_event(InputEvent::PointerMove { pos });
            }
            WindowEvent::PointerButton {
                state: btn_state,
                button,
                ..
            } => {
                let pos = self.state.as_ref().map(|s| s.mouse_pos).unwrap_or_default();
                let button = match button {
                    winit::event::ButtonSource::Mouse(mb) => match mb {
                        winit::event::MouseButton::Left => PointerButton::Primary,
                        winit::event::MouseButton::Right => PointerButton::Secondary,
                        winit::event::MouseButton::Middle => PointerButton::Auxiliary,
                        _ => PointerButton::Primary,
                    },
                    _ => PointerButton::Primary,
                };

                match btn_state {
                    winit::event::ElementState::Pressed => {
                        self.handle_event(InputEvent::PointerDown { pos, button });
                    }
                    winit::event::ElementState::Released => {
                        self.handle_event(InputEvent::PointerUp { pos, button });
                    }
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let pos = self.state.as_ref().map(|s| s.mouse_pos).unwrap_or_default();
                let delta = match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => glam::Vec2::new(x, y),
                    winit::event::MouseScrollDelta::PixelDelta(p) => {
                        glam::Vec2::new(p.x as f32 / 20.0, p.y as f32 / 20.0)
                    }
                };
                self.handle_event(InputEvent::Scroll { pos, delta });
            }
            WindowEvent::KeyboardInput { event, .. } => {
                use sikhar_input::{ui_events::keyboard::Code, Key, KeyboardEvent, NamedKey};

                let key = match &event.logical_key {
                    winit::keyboard::Key::Character(c) => Key::Character(c.to_string()),
                    winit::keyboard::Key::Named(named) => {
                        use winit::keyboard::NamedKey as WN;
                        Key::Named(match named {
                            WN::Enter => NamedKey::Enter,
                            WN::Tab => NamedKey::Tab,
                            WN::Backspace => NamedKey::Backspace,
                            WN::Delete => NamedKey::Delete,
                            WN::Escape => NamedKey::Escape,
                            WN::ArrowUp => NamedKey::ArrowUp,
                            WN::ArrowDown => NamedKey::ArrowDown,
                            WN::ArrowLeft => NamedKey::ArrowLeft,
                            WN::ArrowRight => NamedKey::ArrowRight,
                            WN::Home => NamedKey::Home,
                            WN::End => NamedKey::End,
                            WN::PageUp => NamedKey::PageUp,
                            WN::PageDown => NamedKey::PageDown,
                            _ => return,
                        })
                    }
                    _ => return,
                };

                // Use a generic code since we're translating from logical key
                let code = Code::Unidentified;

                let kb_event = if event.state.is_pressed() {
                    KeyboardEvent::key_down(key.clone(), code)
                } else {
                    KeyboardEvent::key_up(key, code)
                };

                if event.state.is_pressed() {
                    self.handle_event(InputEvent::KeyDown { event: kb_event });
                } else {
                    self.handle_event(InputEvent::KeyUp { event: kb_event });
                }

                // Handle text input
                if event.state.is_pressed() && !event.repeat {
                    if let Some(text) = event.text.as_ref() {
                        let text = text.to_string();
                        if !text.is_empty() && text.chars().all(|c| !c.is_control()) {
                            self.handle_event(InputEvent::TextInput { text });
                        }
                    }
                }
            }
            WindowEvent::Focused(focused) => {
                if focused {
                    self.handle_event(InputEvent::FocusGained);
                } else {
                    self.handle_event(InputEvent::FocusLost);
                }
            }
            WindowEvent::RedrawRequested => {
                let state = self.state.as_mut().unwrap();

                if state.needs_layout {
                    self.build_layout();
                }

                let state = self.state.as_mut().unwrap();
                if state.needs_repaint {
                    self.paint();
                }

                let state = self.state.as_mut().unwrap();

                // Update renderer
                let size = state.surface_state.size;
                state
                    .renderer
                    .set_viewport(size.width as f32, size.height as f32, state.scale_factor);
                state
                    .renderer
                    .set_time(state.start_time.elapsed().as_secs_f32());

                // Prepare render
                state.renderer.prepare(
                    &state.device,
                    &state.queue,
                    &state.draw_list,
                    state.text_system.atlas(),
                );

                // Get frame
                let frame = match state.surface_state.surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(_) => {
                        state.surface_state.reconfigure(&state.device);
                        state
                            .surface_state
                            .surface
                            .get_current_texture()
                            .unwrap()
                    }
                };

                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut encoder = state
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("sikhar_encoder"),
                    });

                let bg = self.config.background;
                state.renderer.render(
                    &mut encoder,
                    &view,
                    wgpu::Color {
                        r: bg.r as f64,
                        g: bg.g as f64,
                        b: bg.b as f64,
                        a: bg.a as f64,
                    },
                );

                state.queue.submit(Some(encoder.finish()));
                frame.present();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &dyn winit::event_loop::ActiveEventLoop) {
        // Request redraw for animation
        // In a real app, you'd only request this when needed
    }
}
