/**
 * Ported from Kansei CameraControls.ts
 * Original: https://github.com/Siroko/kansei/blob/main/src/controls/CameraControls.ts
 * 
 * Controls the camera movement and interaction with mouse and touch events.
 */

use crate::math::Vector3;
use crate::core_engine::Camera;
use std::f32::consts::PI;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, MouseEvent, WheelEvent, TouchEvent, AddEventListenerOptions};

// Internal state that will be shared with event listeners
#[derive(Debug)]
struct CameraControlsState {
    displacement: (f32, f32),
    prev_angles: (f32, f32),
    current_angles: (f32, f32),
    final_radians: (f32, f32),
    down_point: (f32, f32),
    down: bool,
    wheel_delta: f32,
    mouse_x: f32,
    mouse_y: f32,
    _mouse_x: f32,
    _mouse_y: f32,
    enabled: bool,
    offset: Vector3,
    limits: (f32, f32),
    window_width: f32,
    window_height: f32,
}

pub struct CameraControls {
    camera: Camera,
    target: Vector3,
    radius: f32,
    wheel_delta_ease: f32,
    offset_ease: Vector3,
    time: f32,
    state: Rc<RefCell<CameraControlsState>>,
}

impl CameraControls {
    /// Creates a new CameraControls instance and sets up event listeners
    pub fn new(camera: Camera, target: Vector3, radius: f32, canvas_id: &str) -> Result<Self, JsValue> {
        let prev_angles = (0.04, 0.05);
        
        // Get window dimensions
        let window = web_sys::window().ok_or("No window found")?;
        let window_width = window.inner_width()?.as_f64().unwrap_or(800.0) as f32;
        let window_height = window.inner_height()?.as_f64().unwrap_or(600.0) as f32;
        
        let state = Rc::new(RefCell::new(CameraControlsState {
            displacement: (0.0, 0.0),
            prev_angles,
            current_angles: prev_angles,
            final_radians: (prev_angles.0 * (PI * 2.0), prev_angles.1 * (PI * 2.0)),
            down_point: (0.0, 0.0),
            down: false,
            wheel_delta: radius,
            mouse_x: -1.0,
            mouse_y: -1.0,
            _mouse_x: -1.0,
            _mouse_y: -1.0,
            enabled: true,
            offset: Vector3::new(0.0, 0.0, 0.0),
            limits: (0.2, -0.2),
            window_width,
            window_height,
        }));
        
        // Set up event listeners
        Self::setup_events(state.clone(), canvas_id)?;
        
        Ok(Self {
            camera,
            target,
            radius,
            wheel_delta_ease: radius,
            offset_ease: Vector3::new(0.0, 0.0, 0.0),
            time: 0.0,
            state,
        })
    }
    
    /// Set up event listeners for mouse and touch interactions (like Kansei's events() method)
    fn setup_events(state: Rc<RefCell<CameraControlsState>>, canvas_id: &str) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("No window found")?;
        let document = window.document().ok_or("No document found")?;
        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or("Canvas not found")?;
        
        let canvas_target: EventTarget = canvas.clone().into();
        let document_target: EventTarget = document.into();
        
        // Create options for wheel event (non-passive to allow preventDefault)
        let wheel_options = AddEventListenerOptions::new();
        wheel_options.set_passive(false);
        
        // Mouse wheel event (on document, like Kansei)
        {
            let state = state.clone();
            let closure = Closure::wrap(Box::new(move |event: WheelEvent| {
                event.prevent_default();
                let mut s = state.borrow_mut();
                if s.enabled {
                    let delta = event.delta_y() as f32;
                    s.wheel_delta -= delta * 0.1;
                    s._mouse_x = event.page_x() as f32;
                    s._mouse_y = event.page_y() as f32;
                    s.mouse_x = event.page_x() as f32;
                    s.mouse_y = event.page_y() as f32;
                }
            }) as Box<dyn FnMut(_)>);
            
            document_target.add_event_listener_with_callback_and_add_event_listener_options(
                "wheel",
                closure.as_ref().unchecked_ref(),
                &wheel_options
            )?;
            closure.forget();
        }
        
        // Mouse down event
        {
            let state = state.clone();
            let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
                let mut s = state.borrow_mut();
                if s.enabled {
                    s.down = true;
                    s.down_point = (event.page_x() as f32, event.page_y() as f32);
                }
            }) as Box<dyn FnMut(_)>);
            
            canvas_target.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        
        // Mouse up event
        {
            let state = state.clone();
            let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
                let mut s = state.borrow_mut();
                if s.enabled {
                    s.down = false;
                    s.prev_angles = s.current_angles;
                    s._mouse_x = event.page_x() as f32;
                    s._mouse_y = event.page_y() as f32;
                    s.mouse_x = event.page_x() as f32;
                    s.mouse_y = event.page_y() as f32;
                }
            }) as Box<dyn FnMut(_)>);
            
            canvas_target.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        
        // Mouse move event
        {
            let state = state.clone();
            let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
                let mut s = state.borrow_mut();
                if !s.enabled {
                    return;
                }
                
                let page_x = event.page_x() as f32;
                let page_y = event.page_y() as f32;
                
                let normalized_x = page_x / s.window_width - 0.5;
                let normalized_y = page_y / s.window_height - 0.5;
                let scale_offset = -30.0;
                
                s.offset.x = normalized_x * scale_offset;
                s.offset.y = normalized_y * scale_offset;
                
                if s.down {
                    s.displacement.0 = (s.down_point.0 - page_x) / s.window_width;
                    s.displacement.1 = (s.down_point.1 - page_y) / s.window_height;
                    
                    s.current_angles.0 = s.prev_angles.0 + s.displacement.0;
                    s.current_angles.1 = s.prev_angles.1 - s.displacement.1;
                    
                    // Check if outside limits
                    if s.current_angles.1 > s.limits.0 {
                        s.current_angles.1 = s.limits.0;
                        s.prev_angles.1 = s.limits.0;
                        s.down_point.1 = page_y;
                    }
                    
                    if s.current_angles.1 < s.limits.1 {
                        s.current_angles.1 = s.limits.1;
                        s.prev_angles.1 = s.limits.1;
                        s.down_point.1 = page_y;
                    }
                } else {
                    s._mouse_x = page_x;
                    s._mouse_y = page_y;
                }
            }) as Box<dyn FnMut(_)>);
            
            canvas_target.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        
        // Create options for passive touch events
        let touch_options = AddEventListenerOptions::new();
        touch_options.set_passive(true);
        
        // Touch start event
        {
            let state = state.clone();
            let closure = Closure::wrap(Box::new(move |event: TouchEvent| {
                let touches = event.changed_touches();
                if touches.length() > 0 {
                    if let Some(touch) = touches.item(0) {
                        let mut s = state.borrow_mut();
                        if s.enabled {
                            s.down = true;
                            s.down_point = (touch.page_x() as f32, touch.page_y() as f32);
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);
            
            canvas_target.add_event_listener_with_callback_and_add_event_listener_options(
                "touchstart",
                closure.as_ref().unchecked_ref(),
                &touch_options
            )?;
            closure.forget();
        }
        
        // Touch end event
        {
            let state = state.clone();
            let closure = Closure::wrap(Box::new(move |event: TouchEvent| {
                let touches = event.changed_touches();
                if touches.length() > 0 {
                    if let Some(touch) = touches.item(0) {
                        let mut s = state.borrow_mut();
                        if s.enabled {
                            s.down = false;
                            s.prev_angles = s.current_angles;
                            s._mouse_x = touch.page_x() as f32;
                            s._mouse_y = touch.page_y() as f32;
                            s.mouse_x = touch.page_x() as f32;
                            s.mouse_y = touch.page_y() as f32;
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);
            
            canvas_target.add_event_listener_with_callback_and_add_event_listener_options(
                "touchend",
                closure.as_ref().unchecked_ref(),
                &touch_options
            )?;
            closure.forget();
        }
        
        // Touch move event
        {
            let state = state.clone();
            let closure = Closure::wrap(Box::new(move |event: TouchEvent| {
                let touches = event.changed_touches();
                if touches.length() > 0 {
                    if let Some(touch) = touches.item(0) {
                        let mut s = state.borrow_mut();
                        if !s.enabled {
                            return;
                        }
                        
                        let page_x = touch.page_x() as f32;
                        let page_y = touch.page_y() as f32;
                        
                        let normalized_x = page_x / s.window_width - 0.5;
                        let normalized_y = page_y / s.window_height - 0.5;
                        let scale_offset = -30.0;
                        
                        s.offset.x = normalized_x * scale_offset;
                        s.offset.y = normalized_y * scale_offset;
                        
                        if s.down {
                            s.displacement.0 = (s.down_point.0 - page_x) / s.window_width;
                            s.displacement.1 = (s.down_point.1 - page_y) / s.window_height;
                            
                            s.current_angles.0 = s.prev_angles.0 + s.displacement.0;
                            s.current_angles.1 = s.prev_angles.1 - s.displacement.1;
                            
                            // Check if outside limits
                            if s.current_angles.1 > s.limits.0 {
                                s.current_angles.1 = s.limits.0;
                                s.prev_angles.1 = s.limits.0;
                                s.down_point.1 = page_y;
                            }
                            
                            if s.current_angles.1 < s.limits.1 {
                                s.current_angles.1 = s.limits.1;
                                s.prev_angles.1 = s.limits.1;
                                s.down_point.1 = page_y;
                            }
                        } else {
                            s._mouse_x = page_x;
                            s._mouse_y = page_y;
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);
            
            canvas_target.add_event_listener_with_callback_and_add_event_listener_options(
                "touchmove",
                closure.as_ref().unchecked_ref(),
                &touch_options
            )?;
            closure.forget();
        }
        
        log::info!("CameraControls: Event listeners set up successfully");
        Ok(())
    }
    
    /// Update window dimensions (call on resize)
    pub fn set_window_size(&mut self, width: f32, height: f32) {
        let mut state = self.state.borrow_mut();
        state.window_width = width;
        state.window_height = height;
    }

    /// Set the target position for the camera to orbit around
    pub fn set_target(&mut self, target: Vector3) {
        self.target = target;
    }

    /// Set the orbital radius
    pub fn set_radius(&mut self, radius: f32) {
        let mut state = self.state.borrow_mut();
        state.wheel_delta = radius;
    }

    /// Enable or disable the controls
    pub fn set_enabled(&mut self, enabled: bool) {
        let mut state = self.state.borrow_mut();
        state.enabled = enabled;
    }

    /// Update the camera position and orientation based on time and input
    pub fn update(&mut self, delta_time: f32) {
        self.time += delta_time * 0.1;
        
        let mut state = self.state.borrow_mut();
        
        // Interpolate radians in x and y
        state.final_radians.0 += (state.current_angles.0 * PI * 2.0 - state.final_radians.0) / 20.0;
        state.final_radians.1 += (state.current_angles.1 * PI * 2.0 - state.final_radians.1) / 50.0;
        
        self.wheel_delta_ease += (state.wheel_delta - self.wheel_delta_ease) / 10.0;
        self.radius += (state.wheel_delta - self.radius) / 20.0;
        
        // Update offset ease
        self.offset_ease.x += (state.offset.x - self.offset_ease.x) / 10.0;
        self.offset_ease.y += (state.offset.y - self.offset_ease.y) / 10.0;
        self.offset_ease.z += (state.offset.z - self.offset_ease.z) / 10.0;
        
        // Calculate camera position in spherical coordinates
        self.camera.position.x = (self.target.x + self.offset_ease.x) 
            + (state.final_radians.0.sin() * state.final_radians.1.cos() * self.radius);
        self.camera.position.y = (self.target.y + self.offset_ease.y) 
            + (state.final_radians.1.sin() * self.radius);
        self.camera.position.z = (self.target.z + self.offset_ease.z) 
            + (state.final_radians.0.cos() * state.final_radians.1.cos() * self.radius);
        
        // Make camera look at target
        self.camera.look_at(&self.target);
        
        // Smooth mouse position
        state.mouse_x += (state._mouse_x - state.mouse_x) / 10.0;
        state.mouse_y += (state._mouse_y - state.mouse_y) / 10.0;
    }

    /// Get a reference to the camera
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    /// Get a mutable reference to the camera
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    /// Get the current mouse position
    pub fn get_mouse_position(&self) -> (f32, f32) {
        let state = self.state.borrow();
        (state.mouse_x, state.mouse_y)
    }

    /// Get the current target
    pub fn get_target(&self) -> Vector3 {
        self.target
    }

    /// Get the current radius
    pub fn get_radius(&self) -> f32 {
        self.radius
    }
}

