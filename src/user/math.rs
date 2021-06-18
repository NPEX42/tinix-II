use core::ops::*;

pub fn lerpf(v0 : f32, v1 : f32,  t : f32) -> f32 {
    return v0 + t * (v1 - v0);
}

pub fn lerpd(v0 : f64, v1 : f64,  t : f64) -> f64 {
    return v0 + t * (v1 - v0);
}

pub fn lerpi(v0 : isize, v1 : isize,  t : f32) -> isize {
    return (v0 as f32 + t * (v1 - v0) as f32) as isize;
}

pub fn lerpu(v0 : usize, v1 : usize,  t : f32) -> usize {
    return (v0 as f32 + t * (v1 - v0) as f32) as usize;
}

pub fn lerp_p2i(v0 : Point2i, v1 : Point2i, t : f32) -> Point2i {
    let x = lerpi(v0.x, v1.x, t);
    let y = lerpi(v0.y, v1.y, t);

    Point2i::new(x,y)
}

pub fn map_01(range : Range<isize>, value : isize) -> f32 {
    (value as f32 - range.start as f32) as f32 * 1 as f32 / (range.end as f32 - range.start as f32) as f32
}

pub fn mini(a : isize, b : isize) -> isize {
    if a < b {a} else {b}
}

pub fn minu(a : usize, b : usize) -> usize {
    if a < b {a} else {b}
}

pub fn maxi(a : isize, b : isize) -> isize {
    if a < b {b} else {a}
}

pub fn maxu(a : usize, b : usize) -> usize {
    if a < b {b} else {a}
}


pub struct Point2i {
    x : isize,
    y : isize,
}

impl Point2i {
    pub fn new(x : isize, y : isize) -> Self {
        Self {
        x,
        y
        }
    }

    pub fn tuple_xy(&self) -> (isize, isize) {
        (self.x, self.y)
    }
}



macro_rules! implement_add {
    ($tpe : ty) => {
        impl core::ops::Add for $tpe {
            type Output = $tpe;

            fn add(self, rhs : $tpe) -> $tpe {
                let x = rhs.x + self.x;
                let y = rhs.y + self.y;

                <$tpe>::new(x,y)
            }
        }
    }
}

macro_rules! implement_sub {
    ($tpe : ty) => {
        impl core::ops::Sub for $tpe {
            type Output = $tpe;

            fn sub(self, rhs : $tpe) -> $tpe {
                let x = rhs.x - self.x;
                let y = rhs.y - self.y;

                <$tpe>::new(x,y)
            }
        }
    }
}

macro_rules! implement_div {
    ($tpe : ty) => {
        impl core::ops::Div for $tpe {
            type Output = $tpe;

            fn div(self, rhs : $tpe) -> $tpe {
                let x = rhs.x / self.x;
                let y = rhs.y / self.y;

                <$tpe>::new(x,y)
            }
        }
    }
}

macro_rules! implement_mul {
    ($tpe : ty) => {
        impl core::ops::Mul for $tpe {
            type Output = $tpe;

            fn mul(self, rhs : $tpe) -> $tpe {
                let x = rhs.x * self.x;
                let y = rhs.y * self.y;

                <$tpe>::new(x,y)
            }
        }
    }
}

implement_add!(Point2i);
implement_sub!(Point2i);
implement_div!(Point2i);
implement_mul!(Point2i);