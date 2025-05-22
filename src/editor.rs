use crate::BitFlipperParams;
use crossbeam::atomic::AtomicCell;
use nih_plug::params::persist::PersistentField;
use nih_plug::prelude::*;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use serde::{Deserialize, Serialize};
use std::num::NonZeroIsize;
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use ui::{Background, BackgroundPipeline, UiElement};
use wgpu::SurfaceTargetUnsafe;

mod ui;

pub struct CustomWgpuWindow {
    gui_context: Arc<dyn GuiContext>,

    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,

    ui_elements: Vec<Box<dyn UiElement>>,
    params: Arc<BitFlipperParams>,

    mouse_pos: (f64, f64),
    mouse_down: bool,
}

impl CustomWgpuWindow {
    fn new(
        window: &mut baseview::Window<'_>,
        gui_context: Arc<dyn GuiContext>,
        params: Arc<BitFlipperParams>,
        scaling_factor: f32,
    ) -> Self {
        let target = baseview_window_to_surface_target(window);

        pollster::block_on(Self::create(target, gui_context, params, scaling_factor))
    }

    async fn create(
        target: SurfaceTargetUnsafe,
        gui_context: Arc<dyn GuiContext>,
        params: Arc<BitFlipperParams>,
        scaling_factor: f32,
    ) -> Self {
        let (unscaled_width, unscaled_height) = params.editor_state.size();
        let width = (unscaled_width as f64 * scaling_factor as f64).round() as u32;
        let height = (unscaled_height as f64 * scaling_factor as f64).round() as u32;

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

        let surface = unsafe { instance.create_surface_unsafe(target) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                // required_features: wgpu::Features::POLYGON_MODE_LINE,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
                ..Default::default()
            })
            .await
            .expect("Failed to create device");

        let surface_config = surface.get_default_config(&adapter, width, height).unwrap();
        surface.configure(&device, &surface_config);

        let bg_pipeline = Arc::new(BackgroundPipeline::new(&device, &queue, &surface_config));
        let background = Background::new(bg_pipeline.clone());

        let ui_elements: Vec<Box<dyn UiElement>> = vec![Box::new(background)];

        Self {
            gui_context,
            device,
            queue,
            surface,
            surface_config,
            params,
            ui_elements,
            mouse_pos: (0.0, 0.0),
            mouse_down: false,
        }
    }
}

impl baseview::WindowHandler for CustomWgpuWindow {
    fn on_frame(&mut self, _window: &mut baseview::Window) {
        // Do rendering here.

        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            for element in &self.ui_elements {
                element.render(&mut rpass);
            }
        }

        self.queue.submit(Some(encoder.finish()));

        frame.present();
    }

    fn on_event(
        &mut self,
        _window: &mut baseview::Window,
        event: baseview::Event,
    ) -> baseview::EventStatus {
        // Use this to set parameter values.
        let _param_setter = ParamSetter::new(self.gui_context.as_ref());

        match &event {
            baseview::Event::Mouse(event) => match event {
                baseview::MouseEvent::ButtonPressed {
                    button: baseview::MouseButton::Left,
                    modifiers: _,
                } => self.mouse_down = true,
                baseview::MouseEvent::ButtonReleased {
                    button: baseview::MouseButton::Left,
                    modifiers: _,
                } => self.mouse_down = false,
                baseview::MouseEvent::CursorMoved {
                    position,
                    modifiers: _,
                } => self.mouse_pos = (position.x, position.y),
                _ => {}
            },
            baseview::Event::Window(baseview::WindowEvent::Resized(window_info)) => {
                self.params.editor_state.size.store((
                    window_info.logical_size().width.round() as u32,
                    window_info.logical_size().height.round() as u32,
                ));

                self.surface_config.width = window_info.physical_size().width;
                self.surface_config.height = window_info.physical_size().height;

                self.surface.configure(&self.device, &self.surface_config);
            }
            _ => {}
        }

        baseview::EventStatus::Captured
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomWgpuEditorState {
    /// The window's size in logical pixels before applying `scale_factor`.
    #[serde(with = "nih_plug::params::persist::serialize_atomic_cell")]
    size: AtomicCell<(u32, u32)>,
    /// Whether the editor's window is currently open.
    #[serde(skip)]
    open: AtomicBool,
}

impl CustomWgpuEditorState {
    pub fn from_size(size: (u32, u32)) -> Arc<Self> {
        Arc::new(Self {
            size: AtomicCell::new(size),
            open: AtomicBool::new(false),
        })
    }

    /// Returns a `(width, height)` pair for the current size of the GUI in logical pixels.
    pub fn size(&self) -> (u32, u32) {
        self.size.load()
    }

    /// Whether the GUI is currently visible.
    // Called `is_open()` instead of `open()` to avoid the ambiguity.
    pub fn is_open(&self) -> bool {
        self.open.load(Ordering::Acquire)
    }
}

impl<'a> PersistentField<'a, CustomWgpuEditorState> for Arc<CustomWgpuEditorState> {
    fn set(&self, new_value: CustomWgpuEditorState) {
        self.size.store(new_value.size.load());
    }

    fn map<F, R>(&self, f: F) -> R
    where
        F: Fn(&CustomWgpuEditorState) -> R,
    {
        f(self)
    }
}

pub struct CustomWgpuEditor {
    params: Arc<BitFlipperParams>,

    /// The scaling factor reported by the host, if any. On macOS this will never be set and we
    /// should use the system scaling factor instead.
    scaling_factor: AtomicCell<Option<f32>>,
}

impl Editor for CustomWgpuEditor {
    fn spawn(
        &self,
        parent: ParentWindowHandle,
        context: Arc<dyn GuiContext>,
    ) -> Box<dyn std::any::Any + Send> {
        let (unscaled_width, unscaled_height) = self.params.editor_state.size();
        let scaling_factor = self.scaling_factor.load();

        let gui_context = Arc::clone(&context);

        let params = Arc::clone(&self.params);

        let window = baseview::Window::open_parented(
            &ParentWindowHandleAdapter(parent),
            baseview::WindowOpenOptions {
                title: String::from("WGPU Window"),
                // Baseview should be doing the DPI scaling for us
                size: baseview::Size::new(unscaled_width as f64, unscaled_height as f64),
                // NOTE: For some reason passing 1.0 here causes the UI to be scaled on macOS but
                //       not the mouse events.
                scale: scaling_factor
                    .map(|factor| baseview::WindowScalePolicy::ScaleFactor(factor as f64))
                    .unwrap_or(baseview::WindowScalePolicy::SystemScaleFactor),

                // NOTE: The OpenGL feature in baseview is not needed here, but rust-analyzer gets
                // confused when some crates do use it and others don't.
                gl_config: None,
            },
            move |window: &mut baseview::Window<'_>| -> CustomWgpuWindow {
                CustomWgpuWindow::new(window, gui_context, params, scaling_factor.unwrap_or(1.0))
            },
        );

        self.params.editor_state.open.store(true, Ordering::Release);
        Box::new(CustomWgpuEditorHandle {
            state: self.params.editor_state.clone(),
            window,
        })
    }

    fn size(&self) -> (u32, u32) {
        self.params.editor_state.size()
    }

    fn set_scale_factor(&self, factor: f32) -> bool {
        // If the editor is currently open then the host must not change the current HiDPI scale as
        // we don't have a way to handle that. Ableton Live does this.
        if self.params.editor_state.is_open() {
            return false;
        }

        self.scaling_factor.store(Some(factor));
        true
    }

    fn param_value_changed(&self, _id: &str, _normalized_value: f32) {
        // As mentioned above, for now we'll always force a redraw to allow meter widgets to work
        // correctly. In the future we can use an `Arc<AtomicBool>` and only force a redraw when
        // that boolean is set.
    }

    fn param_modulation_changed(&self, _id: &str, _modulation_offset: f32) {}

    fn param_values_changed(&self) {
        // Same
    }
}

/// The window handle used for [`CustomWgpuEditor`].
struct CustomWgpuEditorHandle {
    state: Arc<CustomWgpuEditorState>,
    window: baseview::WindowHandle,
}

/// The window handle enum stored within 'WindowHandle' contains raw pointers. Is there a way around
/// having this requirement?
unsafe impl Send for CustomWgpuEditorHandle {}

impl Drop for CustomWgpuEditorHandle {
    fn drop(&mut self) {
        self.state.open.store(false, Ordering::Release);
        // XXX: This should automatically happen when the handle gets dropped, but apparently not
        self.window.close();
    }
}

/// This version of `baseview` uses a different version of `raw_window_handle than NIH-plug, so we
/// need to adapt it ourselves.
struct ParentWindowHandleAdapter(nih_plug::editor::ParentWindowHandle);

unsafe impl HasRawWindowHandle for ParentWindowHandleAdapter {
    fn raw_window_handle(&self) -> RawWindowHandle {
        match self.0 {
            ParentWindowHandle::X11Window(window) => {
                let mut handle = raw_window_handle::XcbWindowHandle::empty();
                handle.window = window;
                RawWindowHandle::Xcb(handle)
            }
            ParentWindowHandle::AppKitNsView(ns_view) => {
                let mut handle = raw_window_handle::AppKitWindowHandle::empty();
                handle.ns_view = ns_view;
                RawWindowHandle::AppKit(handle)
            }
            ParentWindowHandle::Win32Hwnd(hwnd) => {
                let mut handle = raw_window_handle::Win32WindowHandle::empty();
                handle.hwnd = hwnd;
                RawWindowHandle::Win32(handle)
            }
        }
    }
}

/// WGPU uses raw_window_handle v6, but baseview uses raw_window_handle v5, so manually convert it.
fn baseview_window_to_surface_target(window: &baseview::Window<'_>) -> wgpu::SurfaceTargetUnsafe {
    use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

    let raw_display_handle = window.raw_display_handle();
    let raw_window_handle = window.raw_window_handle();

    wgpu::SurfaceTargetUnsafe::RawHandle {
        raw_display_handle: match raw_display_handle {
            raw_window_handle::RawDisplayHandle::AppKit(_) => {
                raw_window_handle_06::RawDisplayHandle::AppKit(
                    raw_window_handle_06::AppKitDisplayHandle::new(),
                )
            }
            raw_window_handle::RawDisplayHandle::Xlib(handle) => {
                raw_window_handle_06::RawDisplayHandle::Xlib(
                    raw_window_handle_06::XlibDisplayHandle::new(
                        NonNull::new(handle.display),
                        handle.screen,
                    ),
                )
            }
            raw_window_handle::RawDisplayHandle::Xcb(handle) => {
                raw_window_handle_06::RawDisplayHandle::Xcb(
                    raw_window_handle_06::XcbDisplayHandle::new(
                        NonNull::new(handle.connection),
                        handle.screen,
                    ),
                )
            }
            raw_window_handle::RawDisplayHandle::Windows(_) => {
                raw_window_handle_06::RawDisplayHandle::Windows(
                    raw_window_handle_06::WindowsDisplayHandle::new(),
                )
            }
            _ => todo!(),
        },
        raw_window_handle: match raw_window_handle {
            raw_window_handle::RawWindowHandle::AppKit(handle) => {
                raw_window_handle_06::RawWindowHandle::AppKit(
                    raw_window_handle_06::AppKitWindowHandle::new(
                        NonNull::new(handle.ns_view).unwrap(),
                    ),
                )
            }
            raw_window_handle::RawWindowHandle::Xlib(handle) => {
                raw_window_handle_06::RawWindowHandle::Xlib(
                    raw_window_handle_06::XlibWindowHandle::new(handle.window),
                )
            }
            raw_window_handle::RawWindowHandle::Xcb(handle) => {
                raw_window_handle_06::RawWindowHandle::Xcb(
                    raw_window_handle_06::XcbWindowHandle::new(
                        NonZeroU32::new(handle.window).unwrap(),
                    ),
                )
            }
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                let mut raw_handle = raw_window_handle_06::Win32WindowHandle::new(
                    NonZeroIsize::new(handle.hwnd as isize).unwrap(),
                );

                raw_handle.hinstance = NonZeroIsize::new(handle.hinstance as isize);

                raw_window_handle_06::RawWindowHandle::Win32(raw_handle)
            }
            _ => todo!(),
        },
    }
}

pub fn create_editor(params: &Arc<BitFlipperParams>) -> Option<Box<dyn Editor>> {
    Some(Box::new(CustomWgpuEditor {
        params: Arc::clone(params),

        // TODO: We can't get the size of the window when baseview does its own scaling, so if the
        //       host does not set a scale factor on Windows or Linux we should just use a factor of
        //       1. That may make the GUI tiny but it also prevents it from getting cut off.
        #[cfg(target_os = "macos")]
        scaling_factor: AtomicCell::new(None),
        #[cfg(not(target_os = "macos"))]
        scaling_factor: AtomicCell::new(Some(1.0)),
    }))
}
