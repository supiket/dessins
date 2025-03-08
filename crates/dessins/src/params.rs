use crate::{reflect::ControllableParams, shapes::Shapes};
use nannou::prelude::*;

pub struct DesignController {
    pub selected: DesignVariant,
    pub params: DesignParams,
}

macro_rules! design_variants {
    ($($variant:ident => $params:path),* $(,)?) => {
        #[derive(Reflect)]
        pub enum DesignParams {
            $($variant($params),)*
        }

        #[derive(PartialEq, Eq, Clone, Copy)]
        pub enum DesignVariant {
            $($variant,)*
        }

        impl DesignVariant {
            pub fn get_params(&self) -> DesignParams {
                match self {
                    $(Self::$variant => DesignParams::$variant(<$params>::default()),)*
                }
            }
        }

        impl DesignParams {
            pub fn calculate_shapes(&mut self) -> Shapes {
                match self {
                    $(DesignParams::$variant(params) => params.calculate_shapes(),)*
                }
            }

            pub fn control(&mut self, ctx: &mut egui::Context, time: Time<Virtual>) -> (bool, Option<Color>) {
                match self {
                    $(DesignParams::$variant(params) => params.control_params(ctx, time),)*
                }
            }
        }
    };
}

design_variants! {
    Polygon => crate::chapter_1::Polygon,
    Star => crate::chapter_1::Star,
    Composition1 => crate::chapter_1::Composition1,
    Composition2 => crate::chapter_1::Composition2,
    Jolygon => crate::chapter_1::Jolygon,
    Shape => crate::chapter_2::Params,
    Dragon => crate::chapter_3::Dragon,
    Fractal => crate::chapter_4::Fractal,
    Orbital => crate::chapter_5::Orbital,
    Rotating => crate::chapter_5::Rotating,
    Spiral => crate::chapter_5::Spiral,
    Bipartite => crate::chapter_6::Bipartite,
    LinearModulo => crate::chapter_6::LinearModulo,
    LinearSticks => crate::chapter_6::LinearSticks,
    SimpleFractal => crate::chapter_7::SimpleFractal,
    SimpleRoundedFractal => crate::chapter_7::SimpleRoundedFractal,
    SimpleDeformedFractal => crate::chapter_7::SimpleDeformedFractal,
}

impl DesignVariant {
    pub const ALL: &'static [(Self, &'static str)] = &[
        (Self::Polygon, "polygon"),
        (Self::Star, "star"),
        (Self::Composition1, "composition 1"),
        (Self::Composition2, "composition 2"),
        (Self::Jolygon, "jolygon"),
        (Self::Shape, "shape"),
        (Self::Dragon, "dragon"),
        (Self::Fractal, "fractal"),
        (Self::Orbital, "orbital"),
        (Self::Rotating, "rotating"),
        (Self::Spiral, "spiral"),
        (Self::Bipartite, "bipartite"),
        (Self::LinearModulo, "linear modulo"),
        (Self::LinearSticks, "linear sticks"),
        (Self::SimpleFractal, "simple fractal"),
        (Self::SimpleRoundedFractal, "simple rounded fractal"),
        (Self::SimpleDeformedFractal, "simple deformed fractal"),
    ];
}

impl DesignController {
    pub fn control(&mut self, ctx: &mut egui::Context) -> bool {
        let mut changed = false;

        egui::TopBottomPanel::top("dessins").show(ctx, |ui| {
            ui.horizontal(|ui| {
                for (variant, name) in DesignVariant::ALL {
                    changed |= ui
                        .selectable_value(&mut self.selected, *variant, *name)
                        .changed();
                }
            });
        });

        if changed {
            self.params = self.selected.get_params();
        }

        changed
    }
}
