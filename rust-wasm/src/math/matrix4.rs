use super::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Matrix4 {
    pub data: [f32; 16],
}

impl Matrix4 {
    pub fn identity() -> Self {
        Self {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        // Perspective projection for WebGPU (Z range [0, 1])
        // Based on glam's perspective_rh formula
        let tan_half_fov = (fov / 2.0).tan();
        
        Self {
            data: [
                1.0 / (aspect * tan_half_fov), 0.0, 0.0, 0.0,
                0.0, 1.0 / tan_half_fov, 0.0, 0.0,
                0.0, 0.0, far / (near - far), -1.0,
                0.0, 0.0, (near * far) / (near - far), 0.0,
            ],
        }
    }
    
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let rl = 1.0 / (right - left);
        let tb = 1.0 / (top - bottom);
        let fn_inv = 1.0 / (far - near);
        
        Self {
            data: [
                2.0 * rl, 0.0, 0.0, 0.0,
                0.0, 2.0 * tb, 0.0, 0.0,
                0.0, 0.0, fn_inv, 0.0,
                -(right + left) * rl, -(top + bottom) * tb, -near * fn_inv, 1.0,
            ],
        }
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                x, y, z, 1.0,
            ],
        }
    }

    pub fn rotation_x(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, c, s, 0.0,
                0.0, -s, c, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn rotation_y(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            data: [
                c, 0.0, -s, 0.0,
                0.0, 1.0, 0.0, 0.0,
                s, 0.0, c, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn rotation_z(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            data: [
                c, s, 0.0, 0.0,
                -s, c, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: [
                x, 0.0, 0.0, 0.0,
                0.0, y, 0.0, 0.0,
                0.0, 0.0, z, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn multiply(&self, other: &Matrix4) -> Self {
        let mut result = [0.0f32; 16];
        
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i * 4 + j] += self.data[i * 4 + k] * other.data[k * 4 + j];
                }
            }
        }

        Self { data: result }
    }

    pub fn look_at(eye: &Vector3, target: &Vector3, up: &Vector3) -> Self {
        let z = eye.subtract(target).normalize();
        let x = up.cross(&z).normalize();
        let y = z.cross(&x);

        Self {
            data: [
                x.x, y.x, z.x, 0.0,
                x.y, y.y, z.y, 0.0,
                x.z, y.z, z.z, 0.0,
                -x.dot(eye), -y.dot(eye), -z.dot(eye), 1.0,
            ],
        }
    }
}

