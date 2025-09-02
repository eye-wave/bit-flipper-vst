use crate::model::FlipModes;
use crate::{BitFlipperParams, BusHandle, UI_SCALE};
use core::{CustomWgpuEditor, baseview_window_to_surface_target};
use crossbeam::atomic::AtomicCell;
use nih_plug::params::persist::PersistentField;
use nih_plug::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use texture::UVSegment::*;
use ui::texture::TextureAtlas;
use ui::*;
use wgpu::SurfaceTargetUnsafe;

pub const VIEW_WIDTH: u16 = 200;
pub const VIEW_HEIGHT: u16 = 200;

mod core;
mod ui;

fn downscale(pos: (f32, f32)) -> (i16, i16) {
    (
        (pos.0 / UI_SCALE as f32) as i16,
        (pos.1 / UI_SCALE as f32) as i16,
    )
}

#[derive(Debug, Default)]
pub struct EventStore {
    mouse_pos: (f32, f32),
    drag_start: (f32, f32),
    dragging_slider: bool,
    mouse_down: bool,
}

pub struct CustomWgpuWindow {
    gui_context: Arc<dyn GuiContext>,

    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,

    postprocess: Postprocess,
    grayscale_view: wgpu::TextureView,

    scene_elements: Vec<Box<dyn UiElement>>,
    bus_handle: BusHandle,

    params: Arc<BitFlipperParams>,
    event_store: EventStore,
}

impl CustomWgpuWindow {
    fn new(
        window: &mut baseview::Window<'_>,
        gui_context: Arc<dyn GuiContext>,
        bus_handle: BusHandle,
        params: Arc<BitFlipperParams>,
        scaling_factor: f32,
    ) -> Self {
        let target = baseview_window_to_surface_target(window);

        pollster::block_on(Self::create(
            target,
            gui_context,
            bus_handle,
            params,
            scaling_factor,
        ))
    }

    async fn create(
        target: SurfaceTargetUnsafe,
        gui_context: Arc<dyn GuiContext>,
        bus_handle: BusHandle,
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
        let tex_format = surface_config.format;

        let texture_atlas = Arc::new(TextureAtlas::new(&device, &queue));

        let bg_pipeline = Arc::new(BackgroundPipeline::new(
            &device,
            tex_format,
            texture_atlas.clone(),
        ));

        let pipe = Arc::new(StaticBoxPipeline::new(
            &device,
            tex_format,
            texture_atlas.clone(),
        ));

        let monitor_pipeline = Arc::new(SharedMonitorPipeline::new(&device, tex_format));
        let slide_pipe = Arc::new(SliderPipeline::new(
            &device,
            tex_format,
            texture_atlas.clone(),
        ));

        let scene_elements: Vec<Box<dyn UiElement>> = vec![
            Box::new(Background::new(bg_pipeline.clone())),
            Box::new(StaticBox::new(&device, &UV_gui_main, (46, 0), None, pipe.clone()).unwrap()),
            Box::new(
                StaticBox::new(&device, &UV_gui_monitors, (18, 154), None, pipe.clone()).unwrap(),
            ),
            Box::new(ModeButtonBuilder::new(&device, pipe.clone()).mode(FlipModes::Xor)),
            Box::new(ModeButtonBuilder::new(&device, pipe.clone()).mode(FlipModes::Or)),
            Box::new(ModeButtonBuilder::new(&device, pipe.clone()).mode(FlipModes::And)),
            Box::new(ModeButtonBuilder::new(&device, pipe.clone()).mode(FlipModes::Not)),
            Box::new(MonitorGroup::new(
                &device,
                [(20, 155), (105, 155)],
                monitor_pipeline.clone(),
            )),
            Box::new(DigitCluster::new(&device, pipe.clone())),
            Box::new(Slider::new(&device, (74, 142), slide_pipe.clone())),
            Box::new(VolumeText::new(&device, (74, 142), pipe.clone()).unwrap()),
        ];

        let grayscale_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Grayscale Render Target"),
            size: wgpu::Extent3d {
                width: VIEW_WIDTH as u32,
                height: VIEW_HEIGHT as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: tex_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let grayscale_view = grayscale_texture.create_view(&Default::default());
        let postprocess = Postprocess::new(&device, &queue, tex_format, &grayscale_view);

        Self {
            gui_context,
            //
            device,
            queue,
            surface,
            surface_config,
            //
            postprocess,
            grayscale_view,
            //
            scene_elements,
            bus_handle,
            //
            params,
            event_store: EventStore::default(),
        }
    }
}

impl baseview::WindowHandler for CustomWgpuWindow {
    fn on_frame(&mut self, _window: &mut baseview::Window) {
        let mut reader = self.bus_handle.lock().unwrap();
        let bus = reader.read();

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

        for element in self.scene_elements.iter_mut() {
            element.prerender(&self.queue, self.params.clone(), bus);
        }

        // --- First pass: render scene to offscreen grayscale texture ---
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Scene Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.grayscale_view,
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

            for element in &self.scene_elements {
                element.render(&mut rpass);
            }
        }

        // --- Second pass: postprocess to screen ---
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Postprocess Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.postprocess.render(&mut rpass);
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
                } => {
                    self.event_store.dragging_slider = false;
                    self.event_store.mouse_down = true;
                    self.event_store.drag_start = self.event_store.mouse_pos;

                    for el in self.scene_elements.iter_mut() {
                        if let Some(btn) = el.as_mut().as_any_mut().downcast_mut::<Button>() {
                            let mouse_pos = self.event_store.mouse_pos;

                            if btn.is_mouse_over(downscale(mouse_pos)) {
                                let setter = ParamSetter::new(&*self.gui_context);
                                let norm = self.params.mode.preview_normalized(btn.get_state());

                                setter.begin_set_parameter(&self.params.mode);
                                setter.set_parameter_normalized(&self.params.mode, norm);
                                setter.end_set_parameter(&self.params.mode);
                            }

                            continue;
                        }

                        if let Some(cluster) = el.as_any_mut().downcast_mut::<DigitCluster>() {
                            for digit in cluster.digits.iter_mut() {
                                if digit.is_mouse_over(downscale(self.event_store.mouse_pos)) {
                                    if let Some(param) = self.params.bits.get_bit_param(digit.id())
                                    {
                                        let setter = ParamSetter::new(&*self.gui_context);
                                        let norm = param.preview_normalized(!param.value());

                                        setter.begin_set_parameter(param);
                                        setter.set_parameter_normalized(param, norm);
                                        setter.end_set_parameter(param);
                                    }

                                    break;
                                }
                            }
                        }

                        if let Some(slider) = el.as_mut().as_any_mut().downcast_mut::<Slider>() {
                            if slider.is_mouse_over(downscale(self.event_store.mouse_pos)) {
                                self.event_store.dragging_slider = true
                            }

                            continue;
                        }
                    }
                }
                baseview::MouseEvent::ButtonReleased {
                    button: baseview::MouseButton::Left,
                    modifiers: _,
                } => self.event_store.mouse_down = false,
                baseview::MouseEvent::CursorMoved {
                    position,
                    modifiers: _,
                } => {
                    self.event_store.mouse_pos = (position.x as f32, position.y as f32);

                    if self.event_store.dragging_slider && self.event_store.mouse_down {
                        let delta = self.event_store.mouse_pos.0 - self.event_store.drag_start.0;
                        self.event_store.drag_start.0 = self.event_store.mouse_pos.0;

                        let slider_width = 59.0;
                        let delta_norm = delta / slider_width;

                        let param = &self.params.pre_gain;
                        let new_norm =
                            (param.preview_normalized(param.value()) + delta_norm).clamp(0.0, 1.0);

                        let setter = ParamSetter::new(&*self.gui_context);

                        setter.begin_set_parameter(param);
                        setter.set_parameter_normalized(param, new_norm);
                        setter.end_set_parameter(param);
                    }
                }
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

pub fn create_editor(
    params: &Arc<BitFlipperParams>,
    bus_handle: &crate::BusHandle,
) -> Option<Box<dyn Editor>> {
    Some(Box::new(CustomWgpuEditor {
        params: Arc::clone(params),
        bus_handle: Arc::clone(bus_handle),

        // TODO: We can't get the size of the window when baseview does its own scaling, so if the
        //       host does not set a scale factor on Windows or Linux we should just use a factor of
        //       1. That may make the GUI tiny but it also prevents it from getting cut off.
        #[cfg(target_os = "macos")]
        scaling_factor: AtomicCell::new(None),
        #[cfg(not(target_os = "macos"))]
        scaling_factor: AtomicCell::new(Some(1.0)),
    }))
}
