pub fn normalize(q: &mut [f32; 4]) {
    let epsilon = 0.00001;
    let mut d = 0.0;

    for i in 0..4 {
        d += q[i].powi(2);
    }

    if d < epsilon {
        return; // do nothing if it is zero
    }

    let inv_length = 1.0 / d.sqrt();
    q[0] *= inv_length;
    q[1] *= inv_length;
    q[2] *= inv_length;
    q[3] *= inv_length;  
}


pub fn set_matrix(m: &mut [f32; 16], q: &[f32; 4]) {
//     let x2  = x + x;
//     let y2  = y + y;
//     let z2  = z + z;
//     let xx2 = x * x2;
//     let xy2 = x * y2;
//     let xz2 = x * z2;
//     let yy2 = y * y2;
//     let yz2 = y * z2;
//     let zz2 = z * z2;
//     let sx2 = s * x2;
//     let sy2 = s * y2;
//     let sz2 = s * z2;

// m[0] = 1.0 - (yy2 + zz2);
//     m[1] = xy2 + sz2;
//     m[2] = xz2 - sy2;
//     m[3] = 0.0; // column 0;
//     m[4] = xy2 - sz2;
//     m[5] = 1.0 - (xx2 + zz2);
//     m[6] = yz2 + sx2;
//     m[7] = 0.0; // column 1;
//     m[8] = xz2 + sy2;
//     m[9] = yz2 - sx2;
//     m[10] = 1.0 - (xx2 + yy2);
//     m[11] = 0.0; // column 2;
//     m[12] = 0.0;
//     m[13] = 0.0          ;
//     m[14] = 0.0;
//     m[15] = 1.0 // column 3;
}

pub fn slerp(q1: &mut [f32; 4], q2: &[f32; 4], max_angle: f32) {
    
    if max_angle < 0.001 {
		// No rotation allowed. Prevent dividing by 0 later.
        return;
    }

    let mut cosTheta = 0.0;
    for i in 0..4 {
        cosTheta += q1[i] * q2[i];
    }

	// q1 and q2 are already equal.
	// Force q2 just to be sure
    if cosTheta > 0.9999 {
        return;
    }

	// Avoid taking the long path around the sphere
    if cosTheta < 0.0 {
        for i in 0..4 {
            q1[i] = q1[i] * -1.0;
        }
        cosTheta *= -1.0;
    }

    let mut angle = cosTheta.acos();

	// If there is only a 2&deg; difference, and we are allowed 5&deg;,
	// ten we arrive:
    if angle < max_angle {
        return ;
    }

    let ft = max_angle / angle;
    angle = max_angle;

    // res = [0] * 4
    let a = ((1.0 - ft) * angle).cos();
    let b = (ft * angle).sin();
    let _sin = angle.sin();
    for r in 0..4 {
        q1[r] = (a * q1[r] + b * q2[r]) / _sin;
    }
    normalize(q1);
}