use clay_layout::{
        elements::FloatingAttachToElement, fixed, grow, layout::{
        Alignment, 
        LayoutDirection::TopToBottom, 
        Padding,
    }, math::Dimensions, percent, render_commands::RenderCommand, text::TextConfig, Clay, Color, Declaration
};

const WHITE: Color = Color::rgb(255.0, 255.0, 255.0);

const BASE: Color = Color::rgb(252.0, 245.0, 199.0);
const DARK: Color = Color::rgb(36.0, 62.0, 54.0);
const PINK: Color = Color::rgb(239.0, 185.0, 203.0);
const LAVE: Color = Color::rgb(230.0, 173.0, 236.0);
const PURP: Color = Color::rgb(194.0, 135.0, 232.0);

trait CustomStyles {
    fn layout_expand(&mut self) -> Self;
    fn content_background_config(&mut self) -> Self;
}

impl CustomStyles for Declaration{
    fn layout_expand(&mut self) -> Self {
        *self.layout()
            .width(grow!())
            .height(grow!())
            .end()
    }

    fn content_background_config(&mut self) -> Self {
        *self.background_color(Color::rgb(90.0, 90.0, 90.0))
            .corner_radius()
                .all(8.0)
                .end()
    }
}

fn render_sidebar_button(clay: &Clay, text: &str, click_event:bool, selected_page: bool) -> bool {
    let mut clicked = false;

    clay.with_styling(&mut Declaration::new() 
            .layout()
                .padding(Padding::new(16,16,8,8))
                .end()
            .corner_radius()
                .all(5.0)
                .end()
        ,|styling| {
            if clay.hovered() {
                styling.background_color(LAVE);
            }
            if selected_page {
                styling.border()
                    .all_directions(5)
                    .color(PURP)
                    .end();
            }
        },
        || {
            if clay.hovered() {
                clay.text(
                    text, 
                    TextConfig::new()
                        .font_size(16)
                        .color(DARK)
                        .end()
                );
                clicked = click_event;
            }
            else {
                clay.text(
                    text, 
                    TextConfig::new()
                        .font_size(16)
                        .color(PINK)
                        .end()
                );
            }
            
        }
    );

    clicked
}

#[derive(Default)]
pub struct ClayState{
    pub mouse_down_rising_edge: bool,
    pub mouse_position: (f32,f32),
    pub scroll_delta: (f32,f32),
    pub size:(f32,f32),

    pub selected_page: u8
}

pub fn initialize_user_data(user_data: &mut ClayState){
    
}

pub fn create_layout<'a>(clay: &'a mut Clay, user_data: &mut ClayState, time_delta: f32) -> impl Iterator<Item = RenderCommand<'a>>{
    clay.layout_dimensions(user_data.size.into());
    clay.pointer_state(user_data.mouse_position.into(), false);
    clay.update_scroll_containers(false, user_data.scroll_delta.into(), time_delta);

    clay.begin();

    clay.with(&Declaration::new()
        .id(clay.id("outer_container"))
        .layout()
            .width(grow!())
            .height(grow!())
            .padding(Padding::all(8))
            .child_gap(8)
            .end()
        .background_color(BASE)
        , |_|{
            clay.with(&Declaration::new()
                .id(clay.id("sidebar"))
                .layout()
                    //.width(percent!(0.2))
                    //.height(grow!())
                    .direction(TopToBottom)
                    .padding(Padding::all(20))
                    .child_gap(20)
                    .end()
                .background_color(DARK)
                .corner_radius()
                    .all(25.0)
                    .end()
                , |_| {
                    clay.with(&Declaration::new()
                        .layout()
                            .width(grow!())
                            .end()
                        .border()
                            .bottom(5)
                            .end()
                        , |_| {
                            clay.text("Pages:", TextConfig::new()
                                .color(DARK)
                                .font_size(18)
                                .end()
                            );
                        }
                    );

                    let pages = [
                        "Current Date", "Future Date", "Inventory lookup",
                        "Add Items", "Edit Item", "Remove Items",
                        "Web Lookup", "Chat AI", "Weather"
                    ];

                    for i in 0..pages.len() {
                        if render_sidebar_button(clay, pages[i], user_data.mouse_down_rising_edge, user_data.selected_page == i as u8) {
                            user_data.selected_page = i as u8;
                        }
                    }
                }
            );

            clay.with(&Declaration::new()
                .id(clay.id("Main Content"))
                .layout()
                    .direction(TopToBottom)
                    .width(grow!())
                    .height(grow!())
                    .end()
                .border()
                    .all_directions(3)
                    .color(DARK)
                    .end()
                //.background_color(WHITE)
                , |_| {

                    match user_data.selected_page {
                        0 => {
                            clay.with(&Declaration::new()
                                    .id(clay.id("current-date-titlebar"))
                                    .layout()
                                        .width(grow!())
                                        .end()
                                    .border()
                                        .bottom(5)
                                        .color(DARK)
                                        .end()
                                , |_| {
                                    clay.with(&Declaration::new()
                                        .layout()
                                            .width(grow!())
                                            .height(grow!())
                                            .end()
                                        //.background_color(LAVE)
                                        , |_| {}
                                    );

                                    clay.text("Current Date", TextConfig::new()
                                        .font_size(24)
                                        .color(DARK)
                                        .end()
                                    );

                                    clay.with(&Declaration::new()
                                        .layout()
                                            .width(grow!())
                                            .height(grow!())
                                            .end()
                                        //.background_color(LAVE)
                                        , |_| {}
                                    );
                                }
                            );
                        }
                        _ => {}
                    }

                }
            );
        }
    );

    clay.end()
}

use std::{cell::RefCell, rc::Rc};
use crate::ui::ui_renderer::UIState;
pub fn measure_text(text: &str, config: &TextConfig, ui: &mut Rc<RefCell<UIState>>) -> Dimensions {
    ui.borrow_mut().measure_text(text, config.font_size as f32, config.line_height as f32)
}