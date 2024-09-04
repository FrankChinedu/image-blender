pub trait BlendOperation {
    fn perform_operation(&self, pixel1: [u8; 3], pixel2: [u8; 3]) -> [u8; 3];
}

pub struct AverageBlend;

impl BlendOperation for AverageBlend {
    fn perform_operation(&self, pixel1: [u8; 3], pixel2: [u8; 3]) -> [u8; 3] {
        [
            pixel1[0] / 2 + pixel2[0] / 2,
            pixel1[1] / 2 + pixel2[1] / 2,
            pixel1[2] / 2 + pixel2[2] / 2,
        ]
    }
}

pub struct MultiplyBlend;

impl BlendOperation for MultiplyBlend {
    fn perform_operation(&self, pixel1: [u8; 3], pixel2: [u8; 3]) -> [u8; 3] {
        [
            ((pixel1[0] as f32 / 255. * pixel2[0] as f32 / 255.) * 255.) as u8,
            ((pixel1[1] as f32 / 255. * pixel2[1] as f32 / 255.) * 255.) as u8,
            ((pixel1[2] as f32 / 255. * pixel2[2] as f32 / 255.) * 255.) as u8,
        ]
    }
}

pub struct LightenBlend;

impl BlendOperation for LightenBlend {
    fn perform_operation(&self, pixel1: [u8; 3], pixel2: [u8; 3]) -> [u8; 3] {
        [
            pixel1[0].max(pixel2[0]),
            pixel1[1].max(pixel2[1]),
            pixel1[2].max(pixel2[2]),
        ]
    }
}

pub struct DarkenBlend;

impl BlendOperation for DarkenBlend {
    fn perform_operation(&self, pixel1: [u8; 3], pixel2: [u8; 3]) -> [u8; 3] {
        [
            pixel1[0].min(pixel2[0]),
            pixel1[1].min(pixel2[1]),
            pixel1[2].min(pixel2[2]),
        ]
    }
}

pub struct ScreenBlend;

impl BlendOperation for ScreenBlend {
    fn perform_operation(&self, pixel1: [u8; 3], pixel2: [u8; 3]) -> [u8; 3] {
        [
            ((1. - ((1. - (pixel1[0] as f32 / 255.)) * (1. - (pixel2[0] as f32 / 255.))))
                * u8::MAX as f32) as u8,
            ((1. - ((1. - (pixel1[1] as f32 / 255.)) * (1. - (pixel2[1] as f32 / 255.))))
                * u8::MAX as f32) as u8,
            ((1. - ((1. - (pixel1[2] as f32 / 255.)) * (1. - (pixel2[2] as f32 / 255.))))
                * u8::MAX as f32) as u8,
        ]
    }
}

pub struct AdditionBlend;

impl BlendOperation for AdditionBlend {
    fn perform_operation(&self, pixel1: [u8; 3], pixel2: [u8; 3]) -> [u8; 3] {
        [
            (pixel1[0] as u16 + pixel2[0] as u16).clamp(0, u8::MAX as u16) as u8,
            (pixel1[1] as u16 + pixel2[1] as u16).clamp(0, u8::MAX as u16) as u8,
            (pixel1[2] as u16 + pixel2[2] as u16).clamp(0, u8::MAX as u16) as u8,
        ]
    }
}

pub struct SubtrationBlend;

impl BlendOperation for SubtrationBlend {
    fn perform_operation(&self, pixel1: [u8; 3], pixel2: [u8; 3]) -> [u8; 3] {
        [
            (pixel1[0] as u16 - pixel2[0] as u16).clamp(0, u8::MAX as u16) as u8,
            (pixel1[1] as u16 - pixel2[1] as u16).clamp(0, u8::MAX as u16) as u8,
            (pixel1[2] as u16 - pixel2[2] as u16).clamp(0, u8::MAX as u16) as u8,
        ]
    }
}
