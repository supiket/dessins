use crate::{adjustable_dessin::AdjustableDessin, shapes::Shapes};
use nannou::prelude::*;

pub struct DessinWithVariables {
    pub variant: DessinVariant,
    pub variables: DessinVariables,
}

macro_rules! dessin_with_variables {
    ($($variant:ident => $variables:path),* $(,)?) => {
        #[derive(Reflect)]
        pub enum DessinVariables {
            $($variant($variables),)*
        }

        #[derive(PartialEq, Eq, Clone, Copy)]
        pub enum DessinVariant {
            $($variant,)*
        }

        impl DessinVariant {
            pub fn get_variables(&self) -> DessinVariables {
                match self {
                    $(Self::$variant => DessinVariables::$variant(<$variables>::default()),)*
                }
            }
        }

        impl DessinVariables {
            pub fn calculate_shapes(&mut self) -> Shapes {
                match self {
                    $(DessinVariables::$variant(variables) => variables.calculate_shapes(),)*
                }
            }

            pub fn update(&mut self, ctx: &mut egui::Context, time: Time<Virtual>) -> (bool, Option<Color>) {
                match self {
                    $(DessinVariables::$variant(variables) => variables.update_dessin(ctx, time),)*
                }
            }
        }
    };
}

dessin_with_variables! {
    Polygon => crate::dessin_variant::Polygon,
    Star => crate::dessin_variant::Star,
    Composition1 => crate::dessin_variant::Composition1,
    Composition2 => crate::dessin_variant::Composition2,
    Jolygon => crate::dessin_variant::Jolygon,
    RawShape => crate::dessin_variant::RawShape,
    Dragon => crate::dessin_variant::Dragon,
    FractalStar => crate::dessin_variant::FractalStar,
    OrbitalCurve => crate::dessin_variant::curve::Orbital,
    RotatingCurve => crate::dessin_variant::curve::Rotating,
    SpiralCurve => crate::dessin_variant::curve::Spiral,
    LinearBipartite => crate::dessin_variant::linear::Bipartite,
    LinearModulo => crate::dessin_variant::linear::Modulo,
    LinearStick => crate::dessin_variant::linear::Stick,
    RegularSimpleFractal => crate::dessin_variant::simple_fractal::Regular,
    RoundedSimpleFractal => crate::dessin_variant::simple_fractal::Rounded,
    DeformedSimpleFractal => crate::dessin_variant::simple_fractal::Deformed,
}

impl DessinVariant {
    pub const ALL: &'static [(Self, &'static str)] = &[
        (Self::Polygon, "polygon"),
        (Self::Star, "star"),
        (Self::Composition1, "composition 1"),
        (Self::Composition2, "composition 2"),
        (Self::Jolygon, "jolygon"),
        (Self::RawShape, "raw shape"),
        (Self::Dragon, "dragon"),
        (Self::FractalStar, "fractal star"),
        (Self::OrbitalCurve, "orbital curve"),
        (Self::RotatingCurve, "rotating curve"),
        (Self::SpiralCurve, "spiral curve"),
        (Self::LinearBipartite, "linear bipartite"),
        (Self::LinearModulo, "linear modulo"),
        (Self::LinearStick, "linear stick"),
        (Self::RegularSimpleFractal, "regular simple fractal"),
        (Self::RoundedSimpleFractal, "rounded simple fractal"),
        (Self::DeformedSimpleFractal, "deformed simple fractal"),
    ];
}

impl DessinWithVariables {
    pub fn update(&mut self, ctx: &mut egui::Context) -> bool {
        let mut changed = false;

        egui::TopBottomPanel::top("active dessin").show(ctx, |ui| {
            ui.horizontal(|ui| {
                for (variant, name) in DessinVariant::ALL {
                    changed |= ui
                        .selectable_value(&mut self.variant, *variant, *name)
                        .changed();
                }
            });
        });

        if changed {
            self.variables = self.variant.get_variables();
        }

        changed
    }
}
