
// Re-export winit types
pub use winit::event::VirtualKeyCode;
pub use winit::event::MouseButton;
pub use winit::event::ElementState;
pub use winit::dpi::LogicalSize;
pub use winit::dpi::PhysicalSize;
pub use winit::dpi::LogicalPosition;
pub use winit::dpi::PhysicalPosition;

use super::AppControl;
use winit::window::Window;

impl InputState {
    pub const KEYBOARD_BUTTON_COUNT: usize = 255;
    pub const MOUSE_BUTTON_COUNT: usize = 7;
    const MIN_DRAG_DISTANCE : f64 = 2.0;
}

#[derive(Copy, Clone, Debug)]
pub struct MouseDragState {
    pub begin_position: LogicalPosition,
    pub end_position: LogicalPosition,
    pub previous_frame_delta: LogicalPosition,
    pub accumulated_frame_delta: LogicalPosition
}

pub struct InputState {
    window_size: LogicalSize,
    dpi_factor: f64,

    key_is_down: [bool; Self::KEYBOARD_BUTTON_COUNT],
    key_just_down: [bool; Self::KEYBOARD_BUTTON_COUNT],
    key_just_up: [bool; Self::KEYBOARD_BUTTON_COUNT],

    mouse_position: LogicalPosition,
    mouse_button_is_down: [bool; Self::MOUSE_BUTTON_COUNT],
    mouse_button_just_down: [Option<LogicalPosition>; Self::MOUSE_BUTTON_COUNT],
    mouse_button_just_up: [Option<LogicalPosition>; Self::MOUSE_BUTTON_COUNT],

    mouse_button_just_clicked: [Option<LogicalPosition>; Self::MOUSE_BUTTON_COUNT],

    mouse_button_went_down_position: [Option<LogicalPosition>; Self::MOUSE_BUTTON_COUNT],
    mouse_button_went_up_position: [Option<LogicalPosition>; Self::MOUSE_BUTTON_COUNT],

    mouse_drag_in_progress: [Option<MouseDragState>; Self::MOUSE_BUTTON_COUNT],
    mouse_drag_just_finished: [Option<MouseDragState>; Self::MOUSE_BUTTON_COUNT],
}

impl InputState {
    pub fn new(window: &Window) -> InputState {
        return InputState {
            window_size: window.inner_size(),
            dpi_factor: window.hidpi_factor(),
            key_is_down: [false; Self::KEYBOARD_BUTTON_COUNT],
            key_just_down: [false; Self::KEYBOARD_BUTTON_COUNT],
            key_just_up: [false; Self::KEYBOARD_BUTTON_COUNT],
            mouse_position: LogicalPosition::new(0.0, 0.0),
            mouse_button_is_down: [false; Self::MOUSE_BUTTON_COUNT],
            mouse_button_just_down: [None; Self::MOUSE_BUTTON_COUNT],
            mouse_button_just_up: [None; Self::MOUSE_BUTTON_COUNT],
            mouse_button_just_clicked: [None; Self::MOUSE_BUTTON_COUNT],
            mouse_button_went_down_position: [None; Self::MOUSE_BUTTON_COUNT],
            mouse_button_went_up_position: [None; Self::MOUSE_BUTTON_COUNT],
            mouse_drag_in_progress: [None; Self::MOUSE_BUTTON_COUNT],
            mouse_drag_just_finished: [None; Self::MOUSE_BUTTON_COUNT],
        };
    }

    //
    // Accessors
    //
    pub fn window_size(&self) -> LogicalSize {
        self.window_size
    }

    pub fn dpi_factor(&self) -> f64 {
        self.dpi_factor
    }

    pub fn is_key_down(&self, key: VirtualKeyCode) -> bool {
        if let Some(index) = Self::keyboard_button_to_index(key) {
            return self.key_is_down[index];
        } else {
            false
        }
    }

    pub fn is_key_just_down(&self, key: VirtualKeyCode) -> bool {
        if let Some(index) = Self::keyboard_button_to_index(key) {
            return self.key_just_down[index];
        } else {
            false
        }
    }

    pub fn is_key_just_up(&self, key: VirtualKeyCode) -> bool {
        if let Some(index) = Self::keyboard_button_to_index(key) {
            return self.key_just_up[index];
        } else {
            false
        }
    }

    pub fn mouse_position(&self) -> LogicalPosition {
        return self.mouse_position;
    }

    pub fn is_mouse_down(&self, mouse_button: MouseButton) -> bool {
        if let Some(index) = Self::mouse_button_to_index(mouse_button) {
            return self.mouse_button_is_down[index];
        } else {
            false
        }
    }

    pub fn is_mouse_just_down(&self, mouse_button: MouseButton) -> bool {
        if let Some(index) = Self::mouse_button_to_index(mouse_button) {
            return self.mouse_button_just_down[index].is_some();
        } else {
            false
        }
    }

    pub fn mouse_just_down_position(&self, mouse_button: MouseButton) -> Option<LogicalPosition> {
        if let Some(index) = Self::mouse_button_to_index(mouse_button) {
            return self.mouse_button_just_down[index];
        } else {
            None
        }
    }

    pub fn is_mouse_just_up(&self, mouse_button: MouseButton) -> bool {
        if let Some(index) = Self::mouse_button_to_index(mouse_button) {
            return self.mouse_button_just_up[index].is_some();
        } else {
            false
        }
    }

    pub fn mouse_just_up_position(&self, mouse_button: MouseButton) -> Option<LogicalPosition> {
        if let Some(index) = Self::mouse_button_to_index(mouse_button) {
            return self.mouse_button_just_up[index];
        } else {
            None
        }
    }

    pub fn is_mouse_button_just_clicked(&self, mouse_button: MouseButton) -> bool {
        if let Some(index) = Self::mouse_button_to_index(mouse_button) {
            return self.mouse_button_just_clicked[index].is_some();
        } else {
            false
        }
    }

    pub fn mouse_button_just_clicked_position(
        &self,
        mouse_button: MouseButton,
    ) -> Option<LogicalPosition> {
        if let Some(index) = Self::mouse_button_to_index(mouse_button) {
            return self.mouse_button_just_clicked[index];
        } else {
            None
        }
    }

    pub fn mouse_button_went_down_position(&self, mouse_button: MouseButton) -> Option<LogicalPosition> {
        if let Some(index) = Self::mouse_button_to_index(mouse_button) {
            return self.mouse_button_went_down_position[index];
        } else {
            None
        }
    }

    pub fn mouse_button_went_up_position(&self, mouse_button: MouseButton) -> Option<LogicalPosition> {
        if let Some(index) = Self::mouse_button_to_index(mouse_button) {
            return self.mouse_button_went_up_position[index];
        } else {
            None
        }
    }

    pub fn is_mouse_drag_in_progress(&self, mouse_button: MouseButton) -> bool {
        if let Some(index) = Self::mouse_button_to_index(mouse_button) {
            return self.mouse_drag_in_progress[index].is_some();
        } else {
            false
        }
    }

    pub fn mouse_drag_in_progress(&self, mouse_button: MouseButton) -> Option<MouseDragState> {
        if let Some(index) = Self::mouse_button_to_index(mouse_button) {
            return self.mouse_drag_in_progress[index];
        } else {
            None
        }
    }

    pub fn is_mouse_drag_just_finished(&self, mouse_button: MouseButton) -> bool {
        if let Some(index) = Self::mouse_button_to_index(mouse_button) {
            return self.mouse_drag_just_finished[index].is_some();
        } else {
            false
        }
    }

    pub fn mouse_drag_just_finished(&self, mouse_button: MouseButton) -> Option<MouseDragState> {
        if let Some(index) = Self::mouse_button_to_index(mouse_button) {
            return self.mouse_drag_just_finished[index];
        } else {
            None
        }
    }

    //
    // Handlers for significant events
    //
    pub fn end_frame(&mut self) {
        for value in self.key_just_down.iter_mut() {
            *value = false;
        }

        for value in self.key_just_up.iter_mut() {
            *value = false;
        }

        for value in self.mouse_button_just_down.iter_mut() {
            *value = None;
        }

        for value in self.mouse_button_just_up.iter_mut() {
            *value = None;
        }

        for value in self.mouse_button_just_clicked.iter_mut() {
            *value = None;
        }

        for value in self.mouse_drag_just_finished.iter_mut() {
            *value = None;
        }

        for value in self.mouse_drag_in_progress.iter_mut() {
            if let Some(v) = value {
                v.previous_frame_delta = LogicalPosition::new(0.0, 0.0);
            }
        }
    }

    pub fn handle_hidpi_factor_changed(&mut self, dpi_factor: f64) {
        self.dpi_factor = dpi_factor;
    }

    pub fn handle_window_size_changed(&mut self, window_size: LogicalSize) {
        self.window_size = window_size;
    }

    pub fn handle_keyboard_event(
        &mut self,
        keyboard_button: VirtualKeyCode,
        button_state: ElementState
    ) {
        if let Some(kc) = Self::keyboard_button_to_index(keyboard_button) {
            // Assign true if key is down, or false if key is up
            if button_state == ElementState::Pressed {
                if !self.key_is_down[kc] {
                    self.key_just_down[kc] = true;
                }
                self.key_is_down[kc] = true
            } else {
                if self.key_is_down[kc] {
                    self.key_just_up[kc] = true;
                }
                self.key_is_down[kc] = false
            }
        }
    }

    pub fn handle_mouse_button_event(
        &mut self,
        button: MouseButton,
        button_event: ElementState
    ) {
        if let Some(button_index) = Self::mouse_button_to_index(button) {
            assert!(button_index < InputState::MOUSE_BUTTON_COUNT);

            // Update is down/up, just down/up
            match button_event {
                ElementState::Pressed => {
                    self.mouse_button_just_down[button_index] = Some(self.mouse_position);
                    self.mouse_button_is_down[button_index] = true;

                    self.mouse_button_went_down_position[button_index] = Some(self.mouse_position);
                }
                ElementState::Released => {
                    self.mouse_button_just_up[button_index] = Some(self.mouse_position);
                    self.mouse_button_is_down[button_index] = false;

                    self.mouse_button_went_up_position[button_index] = Some(self.mouse_position);

                    match self.mouse_drag_in_progress[button_index] {
                        Some(in_progress) => {
                            let delta = Self::subtract(self.mouse_position, Self::add(in_progress.begin_position, in_progress.accumulated_frame_delta));
                            self.mouse_drag_just_finished[button_index] = Some(MouseDragState {
                                begin_position: in_progress.begin_position,
                                end_position: self.mouse_position,
                                previous_frame_delta: delta,
                                accumulated_frame_delta: Self::add(in_progress.accumulated_frame_delta, delta)
                            });
                        }
                        None => {
                            self.mouse_button_just_clicked[button_index] = Some(self.mouse_position)
                        }
                    }

                    self.mouse_drag_in_progress[button_index] = None;
                }
            }
        }
    }

    pub fn handle_mouse_move_event(&mut self, position: LogicalPosition) {
        //let old_mouse_position = self.mouse_position;

        // Update mouse position
        self.mouse_position = position;

        // Update drag in progress state
        for i in 0..Self::MOUSE_BUTTON_COUNT {
            if self.mouse_button_is_down[i] {
                self.mouse_drag_in_progress[i] = match self.mouse_drag_in_progress[i] {
                    None => {
                        match self.mouse_button_went_down_position[i] {
                            Some(went_down_position) => {
                                let min_drag_distance_met =
                                    Self::distance(went_down_position, self.mouse_position)
                                        > Self::MIN_DRAG_DISTANCE;
                                if min_drag_distance_met {
                                    // We dragged a non-trivial amount, start the drag
                                    Some(MouseDragState {
                                        begin_position: went_down_position,
                                        end_position: self.mouse_position,
                                        previous_frame_delta: Self::subtract(self.mouse_position, went_down_position),
                                        accumulated_frame_delta: Self::subtract(self.mouse_position, went_down_position)
                                    })
                                } else {
                                    // Mouse moved too small an amount to be considered a drag
                                    None
                                }
                            }

                            // We don't know where the mosue went down, so we can't start a drag
                            None => None,
                        }
                    }
                    Some(old_drag_state) => {
                        // We were already dragging, so just update the end position

                        let delta = Self::subtract(self.mouse_position, Self::add(old_drag_state.begin_position, old_drag_state.accumulated_frame_delta));
                        Some(MouseDragState {
                            begin_position: old_drag_state.begin_position,
                            end_position: self.mouse_position,
                            previous_frame_delta: delta,
                            accumulated_frame_delta: Self::add(old_drag_state.accumulated_frame_delta, delta)
                        })
                    }
                };
            }
        }
    }

    pub fn handle_winit_event<T>(
        &mut self,
        app_control: &mut AppControl,
        event: &winit::event::Event<T>,
        _window_target: &winit::event_loop::EventLoopWindowTarget<T>
    ) {
        use winit::event::Event;
        use winit::event::WindowEvent;

        let mut is_close_requested = false;

        match event {
            // Close if the window is killed
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => is_close_requested = true,

            Event::WindowEvent {
                event: WindowEvent::HiDpiFactorChanged(hidpi_factor),
                ..
            } => {
                trace!("dpi scaling factor changed {:?}", hidpi_factor);
                self.handle_hidpi_factor_changed(*hidpi_factor);
                //TODO: fix old mouse positions? Could store as logical and only convert to physical
                // on demand
            }

            Event::WindowEvent {
                event: WindowEvent::Resized(window_size),
                ..
            } => {
                self.handle_window_size_changed(*window_size)
            }

            //Process keyboard input
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                trace!("keyboard input {:?}", input);
                if let Some(vk) = input.virtual_keycode {
                    self.handle_keyboard_event(vk, input.state);
                }
            }

            Event::WindowEvent {
                event:
                WindowEvent::MouseInput {
                    device_id,
                    state,
                    button,
                    modifiers,
                },
                ..
            } => {
                trace!(
                    "mouse button input {:?} {:?} {:?} {:?}",
                    device_id,
                    state,
                    button,
                    modifiers
                );

                self.handle_mouse_button_event(*button, *state);
            }

            Event::WindowEvent {
                event:
                WindowEvent::CursorMoved {
                    device_id,
                    position,
                    modifiers,
                },
                ..
            } => {
                trace!("mouse move input {:?} {:?} {:?}", device_id, position, modifiers);
                self.handle_mouse_move_event(*position);
            }

            // Ignore any other events
            _ => (),
        }

        if is_close_requested {
            trace!("close requested");
            app_control.enqueue_terminate_process();
        }
    }

    //
    // Helper functions
    //
    fn mouse_button_to_index(button: MouseButton) -> Option<usize> {
        let index = match button {
            MouseButton::Left => 0,
            MouseButton::Right => 1,
            MouseButton::Middle => 2,
            MouseButton::Other(x) => (x as usize) + 3
        };

        if index >= Self::MOUSE_BUTTON_COUNT {
            None
        } else {
            Some(index)
        }
    }

    fn keyboard_button_to_index(button: VirtualKeyCode) -> Option<usize> {
        let index = button as usize;
        if index >= Self::KEYBOARD_BUTTON_COUNT {
            None
        } else {
            Some(index)
        }
    }

    fn add(p0: LogicalPosition, p1: LogicalPosition) -> LogicalPosition {
        return LogicalPosition::new(
            p0.x + p1.x,
            p0.y + p1.y
        );
    }

    fn subtract(p0: LogicalPosition, p1: LogicalPosition) -> LogicalPosition {
        return LogicalPosition::new(
            p0.x - p1.x,
            p0.y - p1.y
        );
    }

    fn distance(p0: LogicalPosition, p1: LogicalPosition) -> f64 {
        let x_diff = p1.x - p0.x;
        let y_diff = p1.y - p0.y;

        ((x_diff * x_diff) + (y_diff * y_diff)).sqrt()
    }
}