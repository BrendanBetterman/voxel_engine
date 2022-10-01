
//extern crate glium;


#[allow(unused_imports)]
use glium::{glutin, Surface,Display,index::*};
use glium::vertex::VertexBufferAny;
use glutin::{event_loop::EventLoop};
use std::io::Cursor;
use super::support::camera::CameraState;


pub fn render(camera: &CameraState,vertex_buffer2:&glium::vertex::VertexBufferAny){
    
            110 => {
                vertex: "
                    #version 110
    
                    uniform mat4 persp_matrix;
                    uniform mat4 view_matrix;
                    uniform mat4 rot_x_matrix;
                    uniform mat4 rot_y_matrix;
    
                    attribute vec3 position;
                    attribute vec3 normal;
                    varying vec3 v_position;
                    varying vec3 v_normal;
    
                    void main() {
                        v_position = position;
                        v_normal = normal;
                        gl_Position = persp_matrix * (rot_x_matrix * (rot_y_matrix * view_matrix * vec4(v_position * 0.005, 1.0)));
                    }
                ",
    
                fragment: "
                    #version 110
    
                    varying vec3 v_normal;
    
                    const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);
    
                    void main() {
                        float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                        vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
                        gl_FragColor = vec4(1.0,0.0,0.9,1.0);
                        //gl_FragColor = vec4(color, 1.0);
                    }
                ",
            },
    
            100 => {
                vertex: "
                    #version 100
    
                    uniform lowp mat4 persp_matrix;
                    uniform lowp mat4 view_matrix;
                    uniform lowp mat4 rot_x_matrix;
                    uniform lowp mat4 rot_y_matrix;
                    attribute lowp vec3 position;
                    attribute lowp vec3 normal;
                    varying lowp vec3 v_position;
                    varying lowp vec3 v_normal;
    
                    void main() {
                        v_position = position;
                        v_normal = normal;
                        gl_Position = persp_matrix * (rot_x_matrix* ( rot_y_matrix * view_matrix * vec4(v_position * 0.005, 1.0)));
                    }
                ",
    
                fragment: "
                    #version 100
                    varying lowp vec3 v_normal;
    
                    const lowp vec3 LIGHT = vec3(-0.2, 0.8, 0.1);
    
                    void main() {
                        lowp float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                        lowp vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
                        gl_FragColor = vec4(1.0,0.0,0.9,1.0);
                        //gl_FragColor = vec4(color, 1.0);
                    }
                ",
            },
        ).unwrap();
         // building the uniforms
         let uniforms = uniform! {
            persp_matrix: camera.get_perspective(),
            view_matrix: camera.get_view(),
            //new
            diffuse_tex: &self.diffuse_texture,
            rot_x_matrix: camera.get_rot_x(),
            rot_y_matrix: camera.get_rot_y(),
            // get objects rotation
           
        };
        // draw parameters
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        let mut target = self.display.draw();
        //173,225,229 0.68,0.88,0.9
        //252 231 98 0.988,0.906,0.384
        target.clear_color_and_depth((0.68,0.88,0.9, 0.0), 1.0);
        target.draw(&self.vertex_buffer,
            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            &program, &uniforms, &params).unwrap();
        target.finish().unwrap();

    }
}
