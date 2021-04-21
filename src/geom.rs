pub use cgmath::prelude::*;
pub type Vec3 = cgmath::Vector3<f32>;
pub type Pos3 = cgmath::Point3<f32>;
pub type Mat4 = cgmath::Matrix4<f32>;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Sphere {
    pub c: Pos3,
    pub r: f32,
    pub m: f32,
}

impl Sphere {
    pub fn new(c: Pos3, r: f32) -> Self{
        Sphere{
            c,
            r,
            m: (4.0 / 3.0) * 3.14 * r.powi(3)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Plane {
    pub n: Vec3,
    pub d: f32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Box {
    pub pos: Pos3,
    pub dims: Vec3, // (w, l, h)
    // pub rotation: Vec3, // (p, y, r)
}

impl Box {
    pub fn new(pos: Pos3, dims: Vec3) -> Self{
        Box{
            pos,
            dims,
            // rotation
        }
    }

}

/// Are s1 and s2 touching?
#[allow(dead_code)]
pub fn touching_sphere_sphere(s1: &Sphere, s2: &Sphere) -> bool {
    // Is the (squared) distance between the centers less than the
    // (squared) sum of the radii?
    s2.c.distance2(s1.c) <= (s1.r + s2.r).powi(2)
}
/// What's the offset I'd need to push s1 and s2 out of each other?
#[allow(dead_code)]
pub fn disp_sphere_sphere(s1: &Sphere, s2: &Sphere) -> Option<Vec3> {
    let offset = s2.c - s1.c;
    let distance = offset.magnitude();
    if distance < s1.r + s2.r {
        // Make sure we don't divide by 0
        let distance = if distance == 0.0 { 1.0 } else { distance };
        // How much combined radius is "left over"?
        let disp_mag = (s1.r + s2.r) - distance;
        // Normalize offset and multiply by the amount to push
        Some(offset * (disp_mag / distance))
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn touching_sphere_plane(s: &Sphere, p: &Plane) -> bool {
    // Find the distance of the sphere's center to the plane
    (s.c.dot(p.n) - p.d).abs() <= s.r
}
#[allow(dead_code)]
pub fn disp_sphere_plane(s: &Sphere, p: &Plane) -> Option<Vec3> {
    // Find the distance of the sphere's center to the plane
    let dist = s.c.dot(p.n) - p.d;
    if dist.abs() <= s.r {
        // If we offset from the sphere position opposite the normal,
        // we'll end up hitting the plane at `dist` units away.  So
        // the displacement is just the plane's normal * dist.
        Some(p.n * (s.r - dist))
    } else {
        None
    }
}

// pub fn touching_sphere_box(s: &Sphere, b: &Box) -> bool {
    
//     // Closest point on box to circle center
//     let x = clamp(s.c.x, b.pos.x, b.pos.x + b.dims.0);
//     let y = clamp(s.c.y, b.pos.y, b.pos.y + b.dims.1);
//     let z = clamp(s.c.z, b.pos.z, b.pos.z + b.dims.2);

//     // Check if point is in circle
//     let distance = Pos3(x, y, z).distance(s.c);
//     distance < s.r;
// }

// pub fn disp_sphere_box(s: &Sphere, b: &Box) -> Option<Vec3> {
//     // Closest point on box to circle center
//     let x = clamp(s.c.x, b.pos.x, b.pos.x + b.dims.0);
//     let y = clamp(s.c.y, b.pos.y, b.pos.y + b.dims.1);
//     let z = clamp(s.c.z, b.pos.z, b.pos.z + b.dims.2);

//     distance = Vec3(x, y, z) - s.c.to_vec();

// }
