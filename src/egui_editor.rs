use std::sync::Arc;

use nih_plug::prelude::ParamSetter;
use nih_plug_egui::{egui::{Ui, Response, self, Rect, Pos2, Label, Sense, Separator}, widgets};

use crate::{PluginParams, egui_my_widgets::my_slider::{ui_enum_slider, SliderLayout}};



pub fn delay_gui(ui: &mut Ui, setter: &ParamSetter, params: Arc<PluginParams>) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.heading("Maeror's Delay");

        ui.horizontal(|ui| {
            // DELAY TIMING STACK
            egui::Grid::new("parameters_grid")
            .num_columns(3)
            .striped(true)
            .spacing([10.0, 10.0])
            .show(ui, |ui| {

                ui.vertical(|ui| {
                    ui.label("Time");

                    ui.vertical(|ui| {
                        ui.spacing_mut().slider_width = 150.0;

                        ui_enum_slider(
                            ui, 
                            setter, 
                            &params.delay_time, 
                            "Delay Time", 
                            SliderLayout::Horizontal);

                        ui_enum_slider(
                            ui, 
                            setter, 
                            &params.delay_timing, 
                            "Timing", 
                            SliderLayout::Vertical);
                    });
                });
                //ui.add(Separator::vertical(Separator::default()));
                // DELAY FEEDBACK
                ui.vertical(|ui| {
                    ui.label("Feedback");
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Feedback");
                            ui.label("Cutoff");
                            ui.label("Resonance");
                        });
                        ui.vertical(|ui| {
                            let feedback_slider = ui.add(widgets::ParamSlider::for_param(&params.feedback, setter)
                                .with_width(125.0)
                                .without_value());
                            ui.put(feedback_slider.rect, Label::new(format!("{}",&params.feedback)));
    
                            let cutoff_slider = ui.add(widgets::ParamSlider::for_param(&params.cutoff, setter)
                                .with_width(125.0)
                                .without_value());
                            ui.put(cutoff_slider.rect, Label::new(format!("{}",&params.cutoff)));
    
                            let resonance_slider = ui.add(widgets::ParamSlider::for_param(&params.resonance, setter)
                                .with_width(125.0)
                                .without_value());
                            ui.put(resonance_slider.rect, Label::new(format!("{}",&params.resonance)));
    
                        });
                    });
                });
                //ui.add(Separator::vertical(Separator::default()));
                // DELAY MIXING
                ui.vertical(|ui| {
                    ui.label("Mixing");
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Wet");
                            ui.label("Dry");
                        });
                        ui.vertical(|ui| {
                            let wet_slider = ui.add(widgets::ParamSlider::for_param(&params.wet, setter)
                                .with_width(125.0)
                                .without_value());
                            ui.put(wet_slider.rect, Label::new(format!("{}",&params.wet)));

                            let dry_slider = ui.add(widgets::ParamSlider::for_param(&params.dry, setter)
                                .with_width(125.0)
                                .without_value());
                            ui.put(dry_slider.rect, Label::new(format!("{}",&params.dry)));
                        });
                    });
                });
            });
            
        });
    });
}