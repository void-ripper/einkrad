pub fn cross(w: &[f32; 3], v: &[f32; 3]) -> [f32; 3] {
    [
        w[1] * v[2] - w[2] * v[1],
        w[2] * v[0] - w[0] * v[2],
        w[0] * v[1] - w[1] * v[0],
    ]
}

pub fn normalize(w: &mut [f32; 3]) {
    let inv_sum = 1.0 / (w[0] * w[0] + w[1] * w[1] + w[2] * w[2]).sqrt();

    w[0] *= inv_sum;
    w[1] *= inv_sum;
    w[2] *= inv_sum;
}

pub fn angle(w: &[f32; 3], v: &[f32; 3]) -> f32 {
    let d = w[0] * v[0] + w[1] * v[1] + w[2] * v[2];
    let a = (w[0] * w[0] + w[1] * w[1] + w[2] * w[2]).sqrt();
    let b = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    (d / (a * b)).acos()
    // const a = w[1] * v[2] - w[2] * v[1]
    // const b = w[2] * v[0] - w[0] * v[2]
    // const c = w[0] * v[1] - w[1] * v[0]
    // return Math.acos(d / Math.sqrt(a * a + b * b + c * c));
}

pub fn distance(a: &[f32; 3], b: &[f32; 3]) -> f32 {
    let mut e = 0.0;
    for i in 0..3 {
        let x = a[i] - b[i];
        e += x * x;
    }

    e.sqrt()
}

// def distance_cvec(a, v):
//     e = 0.0

//     d = a[0] - v.x
//     e += d * d
//     d = a[1] - v.y
//     e += d * d
//     d = a[2] - v.z
//     e += d * d

//     return sqrt(e)
