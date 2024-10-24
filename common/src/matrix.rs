use std::f32::consts::PI;

use crate::vector;

pub fn identity(m: &mut [f32; 16]) {
    m[0] = 1.0;
    m[1] = 0.0;
    m[2] = 0.0;
    m[3] = 0.0;
    m[4] = 0.0;
    m[5] = 1.0;
    m[6] = 0.0;
    m[7] = 0.0;
    m[8] = 0.0;
    m[9] = 0.0;
    m[10] = 1.0;
    m[11] = 0.0;
    m[12] = 0.0;
    m[13] = 0.0;
    m[14] = 0.0;
    m[15] = 1.0;
}

pub fn transpose(m: &mut [f32; 16]) {
    m.swap(1, 4);
    m.swap(2, 8);
    m.swap(3, 12);
    m.swap(6, 9);
    m.swap(7, 13);
    m.swap(11, 14);
}

pub fn translate(m: &mut [f32; 16], v: &[f32; 3]) {
    m[12] = v[0] * m[0] + v[1] * m[4] + v[2] * m[8] + m[12];
    m[13] = v[0] * m[1] + v[1] * m[5] + v[2] * m[9] + m[13];
    m[14] = v[0] * m[2] + v[1] * m[6] + v[2] * m[10] + m[14];
}

pub fn scale(m: &mut [f32; 16], v: &[f32; 3]) {
    m[0] = v[0] * m[0];
    m[1] = v[0] * m[1];
    m[2] = v[0] * m[2];
    m[3] = v[0] * m[3];

    m[4] = v[1] * m[4];
    m[5] = v[1] * m[5];
    m[6] = v[1] * m[6];
    m[7] = v[1] * m[7];

    m[8] = v[2] * m[8];
    m[9] = v[2] * m[9];
    m[10] = v[2] * m[10];
    m[11] = v[2] * m[11];
}

pub fn mul_assign(m: &mut [f32; 16], o: &[f32; 3]) {
    let tm0 = o[0] * m[0] + o[1] * m[4] + o[2] * m[8] + o[3] * m[12];
    let tm1 = o[0] * m[1] + o[1] * m[5] + o[2] * m[9] + o[3] * m[13];
    let tm2 = o[0] * m[2] + o[1] * m[6] + o[2] * m[10] + o[3] * m[14];
    let tm3 = o[0] * m[3] + o[1] * m[7] + o[2] * m[11] + o[3] * m[15];

    let tm4 = o[4] * m[0] + o[5] * m[4] + o[6] * m[8] + o[7] * m[12];
    let tm5 = o[4] * m[1] + o[5] * m[5] + o[6] * m[9] + o[7] * m[13];
    let tm6 = o[4] * m[2] + o[5] * m[6] + o[6] * m[10] + o[7] * m[14];
    let tm7 = o[4] * m[3] + o[5] * m[7] + o[6] * m[11] + o[7] * m[15];

    let tm8 = o[8] * m[0] + o[9] * m[4] + o[10] * m[8] + o[11] * m[12];
    let tm9 = o[8] * m[1] + o[9] * m[5] + o[10] * m[9] + o[11] * m[13];
    let tm10 = o[8] * m[2] + o[9] * m[6] + o[10] * m[10] + o[11] * m[14];
    let tm11 = o[8] * m[3] + o[9] * m[7] + o[10] * m[11] + o[11] * m[15];

    let tm12 = o[12] * m[0] + o[13] * m[4] + o[14] * m[8] + o[15] * m[12];
    let tm13 = o[12] * m[1] + o[13] * m[5] + o[14] * m[9] + o[15] * m[13];
    let tm14 = o[12] * m[2] + o[13] * m[6] + o[14] * m[10] + o[15] * m[14];
    let tm15 = o[12] * m[3] + o[13] * m[7] + o[14] * m[11] + o[15] * m[15];

    m[0] = tm0;
    m[1] = tm1;
    m[2] = tm2;
    m[3] = tm3;
    m[4] = tm4;
    m[5] = tm5;
    m[6] = tm6;
    m[7] = tm7;
    m[8] = tm8;
    m[9] = tm9;
    m[10] = tm10;
    m[11] = tm11;
    m[12] = tm12;
    m[13] = tm13;
    m[14] = tm14;
    m[15] = tm15;
}

pub fn rotate_by_vector(m: &mut [f32; 16], a: f32, n: &[f32; 3]) {
    let (sin_a, cos_a) = a.sin_cos();

    let _0 = cos_a + n[0] * n[0] * (1.0 - cos_a);
    let _4 = n[0] * n[1] * (1.0 - cos_a) - n[2] * sin_a;
    let _8 = n[0] * n[2] * (1.0 - cos_a) + n[1] * sin_a;

    let _1 = n[0] * n[1] * (1.0 - cos_a) + n[2] * sin_a;
    let _5 = cos_a + n[1] * n[1] * (1.0 - cos_a);
    let _9 = n[1] * n[2] * (1.0 - cos_a) - n[0] * sin_a;

    let _2 = n[0] * n[2] * (1.0 - cos_a) - n[1] * sin_a;
    let _6 = n[1] * n[2] * (1.0 - cos_a) + n[0] * sin_a;
    let _10 = cos_a + n[2] * n[2] * (1.0 - cos_a);

    let tm0 = _0 * m[0] + _1 * m[4] + _2 * m[8];
    let tm1 = _0 * m[1] + _1 * m[5] + _2 * m[9];
    let tm2 = _0 * m[2] + _1 * m[6] + _2 * m[10];
    let tm3 = _0 * m[3] + _1 * m[7] + _2 * m[11];

    let tm4 = _4 * m[0] + _5 * m[4] + _6 * m[8];
    let tm5 = _4 * m[1] + _5 * m[5] + _6 * m[9];
    let tm6 = _4 * m[2] + _5 * m[6] + _6 * m[10];
    let tm7 = _4 * m[3] + _5 * m[7] + _6 * m[11];

    let tm8 = _8 * m[0] + _9 * m[4] + _10 * m[8];
    let tm9 = _8 * m[1] + _9 * m[5] + _10 * m[9];
    let tm10 = _8 * m[2] + _9 * m[6] + _10 * m[10];
    let tm11 = _8 * m[3] + _9 * m[7] + _10 * m[11];

    m[0] = tm0;
    m[1] = tm1;
    m[2] = tm2;
    m[3] = tm3;
    m[4] = tm4;
    m[5] = tm5;
    m[6] = tm6;
    m[7] = tm7;
    m[8] = tm8;
    m[9] = tm9;
    m[10] = tm10;
    m[11] = tm11;
}

pub fn rotate_by_quaternion(m: &mut [f32; 16], q: &[f32; 4]) {
    let x2 = q[0] + q[0];
    let y2 = q[1] + q[1];
    let z2 = q[2] + q[2];
    let xx2 = q[0] * x2;
    let xy2 = q[0] * y2;
    let xz2 = q[0] * z2;
    let yy2 = q[1] * y2;
    let yz2 = q[1] * z2;
    let zz2 = q[2] * z2;
    let sx2 = q[3] * x2;
    let sy2 = q[3] * y2;
    let sz2 = q[3] * z2;

    let _0 = 1.0 - (yy2 + zz2);
    let _1 = xy2 + sz2;
    let _2 = xz2 - sy2;
    let _3 = 0; // column 0;
    let _4 = xy2 - sz2;
    let _5 = 1.0 - (xx2 + zz2);
    let _6 = yz2 + sx2;
    let _7 = 0; // column 1;
    let _8 = xz2 + sy2;
    let _9 = yz2 - sx2;
    let _10 = 1.0 - (xx2 + yy2);
    let _11 = 0; // column 2;
    let _12 = 0;
    let _13 = 0;
    let _14 = 0;
    let _15 = 1; // column 3;

    let tm0 = _0 * m[0] + _1 * m[4] + _2 * m[8];
    let tm1 = _0 * m[1] + _1 * m[5] + _2 * m[9];
    let tm2 = _0 * m[2] + _1 * m[6] + _2 * m[10];
    let tm3 = _0 * m[3] + _1 * m[7] + _2 * m[11];

    let tm4 = _4 * m[0] + _5 * m[4] + _6 * m[8];
    let tm5 = _4 * m[1] + _5 * m[5] + _6 * m[9];
    let tm6 = _4 * m[2] + _5 * m[6] + _6 * m[10];
    let tm7 = _4 * m[3] + _5 * m[7] + _6 * m[11];

    let tm8 = _8 * m[0] + _9 * m[4] + _10 * m[8];
    let tm9 = _8 * m[1] + _9 * m[5] + _10 * m[9];
    let tm10 = _8 * m[2] + _9 * m[6] + _10 * m[10];
    let tm11 = _8 * m[3] + _9 * m[7] + _10 * m[11];

    m[0] = tm0;
    m[1] = tm1;
    m[2] = tm2;
    m[3] = tm3;
    m[4] = tm4;
    m[5] = tm5;
    m[6] = tm6;
    m[7] = tm7;
    m[8] = tm8;
    m[9] = tm9;
    m[10] = tm10;
    m[11] = tm11;
}

pub fn determinant(m: &[f32; 16]) -> f32 {
    let erg = m[12] * m[9] * m[6] * m[3] - m[8] * m[13] * m[6] * m[3] - m[12] * m[5] * m[10] * m[3]
        + m[4] * m[13] * m[10] * m[3]
        + m[8] * m[5] * m[14] * m[3]
        - m[4] * m[9] * m[14] * m[3]
        - m[12] * m[9] * m[2] * m[7]
        + m[8] * m[13] * m[2] * m[7]
        + m[12] * m[1] * m[10] * m[7]
        - m[0] * m[13] * m[10] * m[7]
        - m[8] * m[1] * m[14] * m[7]
        + m[0] * m[9] * m[14] * m[7]
        + m[12] * m[5] * m[2] * m[11]
        - m[4] * m[13] * m[2] * m[11]
        - m[12] * m[1] * m[6] * m[11]
        + m[0] * m[13] * m[6] * m[11]
        + m[4] * m[1] * m[14] * m[11]
        - m[0] * m[5] * m[14] * m[11]
        - m[8] * m[5] * m[2] * m[15]
        + m[4] * m[9] * m[2] * m[15]
        + m[8] * m[1] * m[6] * m[15]
        - m[0] * m[9] * m[6] * m[15]
        - m[4] * m[1] * m[10] * m[15]
        + m[0] * m[5] * m[10] * m[15];

    erg
}

pub fn inverse(m: &[f32; 16], res: &mut [f32; 16]) {
    let d = determinant(m);

    if d == 0.0 {
        res.copy_from_slice(m);
        return;
    }

    res[0] = (-m[13] * m[10] * m[7] + m[9] * m[14] * m[7] + m[13] * m[6] * m[11]
        - m[5] * m[14] * m[11]
        - m[9] * m[6] * m[15]
        + m[5] * m[10] * m[15])
        / d;
    res[4] = (m[12] * m[10] * m[7] - m[8] * m[14] * m[7] - m[12] * m[6] * m[11]
        + m[4] * m[14] * m[11]
        + m[8] * m[6] * m[15]
        - m[4] * m[10] * m[15])
        / d;
    res[8] = (-m[12] * m[9] * m[7] + m[8] * m[13] * m[7] + m[12] * m[5] * m[11]
        - m[4] * m[13] * m[11]
        - m[8] * m[5] * m[15]
        + m[4] * m[9] * m[15])
        / d;
    res[12] = (m[12] * m[9] * m[6] - m[8] * m[13] * m[6] - m[12] * m[5] * m[10]
        + m[4] * m[13] * m[10]
        + m[8] * m[5] * m[14]
        - m[4] * m[9] * m[14])
        / d;
    res[1] = (m[13] * m[10] * m[3] - m[9] * m[14] * m[3] - m[13] * m[2] * m[11]
        + m[1] * m[14] * m[11]
        + m[9] * m[2] * m[15]
        - m[1] * m[10] * m[15])
        / d;
    res[5] = (-m[12] * m[10] * m[3] + m[8] * m[14] * m[3] + m[12] * m[2] * m[11]
        - m[0] * m[14] * m[11]
        - m[8] * m[2] * m[15]
        + m[0] * m[10] * m[15])
        / d;
    res[9] = (m[12] * m[9] * m[3] - m[8] * m[13] * m[3] - m[12] * m[1] * m[11]
        + m[0] * m[13] * m[11]
        + m[8] * m[1] * m[15]
        - m[0] * m[9] * m[15])
        / d;
    res[13] = (-m[12] * m[9] * m[2] + m[8] * m[13] * m[2] + m[12] * m[1] * m[10]
        - m[0] * m[13] * m[10]
        - m[8] * m[1] * m[14]
        + m[0] * m[9] * m[14])
        / d;
    res[2] = (-m[13] * m[6] * m[3] + m[5] * m[14] * m[3] + m[13] * m[2] * m[7]
        - m[1] * m[14] * m[7]
        - m[5] * m[2] * m[15]
        + m[1] * m[6] * m[15])
        / d;
    res[6] = (m[12] * m[6] * m[3] - m[4] * m[14] * m[3] - m[12] * m[2] * m[7]
        + m[0] * m[14] * m[7]
        + m[4] * m[2] * m[15]
        - m[0] * m[6] * m[15])
        / d;
    res[10] = (-m[12] * m[5] * m[3] + m[4] * m[13] * m[3] + m[12] * m[1] * m[7]
        - m[0] * m[13] * m[7]
        - m[4] * m[1] * m[15]
        + m[0] * m[5] * m[15])
        / d;
    res[14] = (m[12] * m[5] * m[2] - m[4] * m[13] * m[2] - m[12] * m[1] * m[6]
        + m[0] * m[13] * m[6]
        + m[4] * m[1] * m[14]
        - m[0] * m[5] * m[14])
        / d;
    res[3] = (m[9] * m[6] * m[3] - m[5] * m[10] * m[3] - m[9] * m[2] * m[7]
        + m[1] * m[10] * m[7]
        + m[5] * m[2] * m[11]
        - m[1] * m[6] * m[11])
        / d;
    res[7] = (-m[8] * m[6] * m[3] + m[4] * m[10] * m[3] + m[8] * m[2] * m[7]
        - m[0] * m[10] * m[7]
        - m[4] * m[2] * m[11]
        + m[0] * m[6] * m[11])
        / d;
    res[11] = (m[8] * m[5] * m[3] - m[4] * m[9] * m[3] - m[8] * m[1] * m[7]
        + m[0] * m[9] * m[7]
        + m[4] * m[1] * m[11]
        - m[0] * m[5] * m[11])
        / d;
    res[15] = (-m[8] * m[5] * m[2] + m[4] * m[9] * m[2] + m[8] * m[1] * m[6]
        - m[0] * m[9] * m[6]
        - m[4] * m[1] * m[10]
        + m[0] * m[5] * m[10])
        / d;
}

pub fn look_at(m: &mut [f32; 16], lookat: &[f32; 3], up: &[f32; 3]) {
    let mut f = [m[12] - lookat[0], m[13] - lookat[1], m[14] - lookat[2]];

    vector::normalize(&mut f);

    let mut s = vector::cross(&f, up);
    vector::normalize(&mut s);

    let mut u = vector::cross(&s, &f);
    vector::normalize(&mut u);

    m[0] = s[0];
    m[1] = s[1];
    m[2] = s[2];
    m[4] = u[0];
    m[5] = u[1];
    m[6] = u[2];
    m[8] = -f[0];
    m[9] = -f[1];
    m[10] = -f[2];
}

pub fn look_dir(m: &mut [f32; 16], dir: &[f32; 3], up: &[f32; 3]) {
    let mut s = vector::cross(dir, up);
    vector::normalize(&mut s);

    let mut u = vector::cross(&s, dir);
    vector::normalize(&mut u);

    m[0] = s[0];
    m[1] = s[1];
    m[2] = s[2];
    m[4] = u[0];
    m[5] = u[1];
    m[6] = u[2];
    m[8] = -dir[0];
    m[9] = -dir[1];
    m[10] = -dir[2];
}

pub fn rotate_x(m: &mut [f32; 16], v: f32) {
    let (sin_a, cos_a) = v.sin_cos();

    let _0 = cos_a + (1.0 - cos_a);
    let _5 = cos_a;
    let _6 = sin_a;
    let _9 = -sin_a;
    let _10 = cos_a;

    let tm_0 = _0 * m[0];
    let tm_1 = _0 * m[1];
    let tm_2 = _0 * m[2];
    let tm_3 = _0 * m[3];

    let tm_4 = _5 * m[4] + _6 * m[8];
    let tm_5 = _5 * m[5] + _6 * m[9];
    let tm_6 = _5 * m[6] + _6 * m[10];
    let tm_7 = _5 * m[7] + _6 * m[11];

    let tm_8 = _9 * m[4] + _10 * m[8];
    let tm_9 = _9 * m[5] + _10 * m[9];
    let tm_10 = _9 * m[6] + _10 * m[10];
    let tm_11 = _9 * m[7] + _10 * m[11];

    m[0] = tm_0;
    m[1] = tm_1;
    m[2] = tm_2;
    m[3] = tm_3;
    m[4] = tm_4;
    m[5] = tm_5;
    m[6] = tm_6;
    m[7] = tm_7;
    m[8] = tm_8;
    m[9] = tm_9;
    m[10] = tm_10;
    m[11] = tm_11;
}

pub fn rotate_y(m: &mut [f32; 16], v: f32) {
    let (sin_a, cos_a) = v.sin_cos();

    let _0 = cos_a;
    let _8 = sin_a;
    let _2 = -sin_a;
    let _10 = cos_a;

    let tm_0 = _0 * m[0] + _2 * m[8];
    let tm_1 = _0 * m[1] + _2 * m[9];
    let tm_2 = _0 * m[2] + _2 * m[10];
    let tm_3 = _0 * m[3] + _2 * m[11];

    let tm_8 = _8 * m[0] + _10 * m[8];
    let tm_9 = _8 * m[1] + _10 * m[9];
    let tm_10 = _8 * m[2] + _10 * m[10];
    let tm_11 = _8 * m[3] + _10 * m[11];

    m[0] = tm_0;
    m[1] = tm_1;
    m[2] = tm_2;
    m[3] = tm_3;
    m[8] = tm_8;
    m[9] = tm_9;
    m[10] = tm_10;
    m[11] = tm_11;
}

pub fn rotate_z(m: &mut [f32; 16], v: f32) {
    let (sin_a, cos_a) = v.sin_cos();

    let _0 = cos_a;
    let _4 = -sin_a;
    let _1 = sin_a;
    let _5 = cos_a;

    let tm_0 = _0 * m[0] + _1 * m[4];
    let tm_1 = _0 * m[1] + _1 * m[5];
    let tm_2 = _0 * m[2] + _1 * m[6];
    let tm_3 = _0 * m[3] + _1 * m[7];

    let tm_4 = _4 * m[0] + _5 * m[4];
    let tm_5 = _4 * m[1] + _5 * m[5];
    let tm_6 = _4 * m[2] + _5 * m[6];
    let tm_7 = _4 * m[3] + _5 * m[7];

    m[0] = tm_0;
    m[1] = tm_1;
    m[2] = tm_2;
    m[3] = tm_3;
    m[4] = tm_4;
    m[5] = tm_5;
    m[6] = tm_6;
    m[7] = tm_7;
}

pub fn get_rotation(m: &mut [f32; 16])-> [f32; 4] {
    let tr = m[0] + m[5] + m[10];
    let mut s = 0.0;
    let mut qw = 0.0;
    let mut qx = 0.0;
    let mut qy = 0.0;
    let mut qz = 0.0;

    if tr > 0.0 {
        s = (tr + 1.0).sqrt() * 2.0; // S=4*qw
        qw = 0.25 * s;
        qx = (m[6] - m[9]) / s;
        qy = (m[8] - m[2]) / s;
        qz = (m[1] - m[4]) / s;
    }
    else if (m[0] > m[5]) & (m[0] > m[10]) {
        s = (1.0 + m[0] - m[5] - m[10]).sqrt() * 2.0; // S=4*qx
        qw = (m[6] - m[9]) / s;
        qx = 0.25 * s;
        qy = (m[4] + m[1]) / s;
        qz = (m[8] + m[2]) / s;
    }
    else if m[5] > m[10] {
    s = (1.0 + m[5] - m[0] - m[10]).sqrt() * 2.0; // S=4*qy
        qw = (m[8] - m[2]) / s;
        qx = (m[4] + m[1]) / s;
        qy = 0.25 * s;
        qz = (m[9] + m[6]) / s;
    }
    else {
        s = (1.0 + m[10] - m[0] - m[5]).sqrt() * 2.0; // S=4*qz
        qw = (m[1] - m[4]) / s;
        qx = (m[8] + m[2]) / s;
        qy = (m[9] + m[6]) / s;
        qz = 0.25 * s;
    }

    [qx, qy, qz, qw]
}

pub fn get_angle(m: &[f32; 16]) -> [f32; 3] {
    // let EPSILON = 0.00001;

    // find yaw (around y-axis) first
    // NOTE: asin() returns -90~+90, so correct the angle range -180~+180
    // using z value of forward vector
    let mut yaw = m[2].asin() - PI;
    if m[0] < 0.0 {
        if yaw >= 0.0 {
            yaw = PI - yaw;
        }
        else {
            yaw =-PI - yaw;
        }
    }

    // find roll (around z-axis) and pitch (around x-axis)
    // if forward vector is (1,0,0) or (-1,0,0), then m[0]=m[4]=m[9]=m[10]=0
    // if m[0] > -EPSILON and m[0] < EPSILON:
    let roll  = 0.0;  //@@ assume roll=0
    let pitch = m[5].atan2(m[4]);
    // else:
    //     roll = atan2(m[1], m[0])
    //     pitch = atan2(m[10], m[6])

    [pitch, yaw, roll]
}