use nih_plug::prelude::{Param, FloatParam, ParamSetter, EnumParam, Enum};
use nih_plug_egui::{egui::{self, Label, Layout}, widgets};

pub enum SliderLayout {
    Horizontal,
    Vertical,
}

pub fn ui_float_slider(
    ui: &mut egui::Ui, 
    setter: &ParamSetter, 
    param: &FloatParam, 
    label: &str, 
    layout: SliderLayout
) {
    match layout {
        SliderLayout::Horizontal => {
            ui.horizontal_centered(|ui| {
                ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.label(label);
                    let slider = 
                        ui.add(widgets::ParamSlider::for_param(param, setter)
                        .without_value());
                    ui.put(slider.rect, Label::new(format!("{}",&param)));
                });
            });
        },
        SliderLayout::Vertical => {
            ui.vertical_centered(|ui| {
                ui.with_layout(Layout::top_down_justified(egui::Align::Center), |ui| {
                    ui.label(label);
                    let slider = 
                        ui.add(widgets::ParamSlider::for_param(param, setter)
                        .without_value());
                    ui.put(slider.rect, Label::new(format!("{}",&param)));
                });
            });
        }
    }
}

pub fn ui_enum_slider<T: Enum + PartialEq>(
    ui: &mut egui::Ui, 
    setter: &ParamSetter, 
    param: &EnumParam<T>, 
    label: &str, 
    layout: SliderLayout
) {
    match layout {
        SliderLayout::Horizontal => {
            ui.horizontal(|ui| {
                let lab = ui.label(label);
                let slider = 
                    ui.add(widgets::ParamSlider::for_param(param, setter)
                    .without_value());
                ui.put(slider.rect, Label::new(format!("{}",&param)));
                // let mut label_rect = slider.rect;
                // ui.put(label_rect, Label::new(format!("{}",label)));
            });
        },
        SliderLayout::Vertical => {
            ui.vertical(|ui| {
                ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                    ui.label(label);
                    let slider = 
                        ui.add(widgets::ParamSlider::for_param(param, setter)
                        .without_value());
                    ui.put(slider.rect, Label::new(format!("{}",&param)));
                });
            });
        }
    }
}