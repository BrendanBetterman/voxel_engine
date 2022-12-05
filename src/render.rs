
//extern crate glium;


use glium::glutin::dpi::Position;
#[allow(unused_imports)]
use glium::{glutin, Surface,Display,index::*};
use glium::vertex::VertexBufferAny;
use glutin::{event_loop::EventLoop};
use std::io::Cursor;
use crate::support::create_voxel_chunk;

use super::support::camera::CameraState;
use super::support;
pub struct Renderer{
    //event_loop: EventLoop<()>,
    display: Display,
    diffuse_texture: glium::texture::SrgbTexture2d,
    vertex_buffer: Vec<VertexBufferAny>,
    chunk:Vec<[[[u8;32];32];32]>,
    
}
impl Renderer{
    pub fn new(event_loop:&EventLoop<()>) -> Renderer{
         //= event;//glutin::event_loop::EventLoop::new();
         
        let wb = glutin::window::WindowBuilder::new().with_title("Pigeoneer")
            .with_inner_size(glutin::dpi::LogicalSize::new(1280,720))
            .with_position(glutin::dpi::LogicalPosition::new(320,180));
           
        let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();
        
        let image = image::load(Cursor::new(&include_bytes!("Texture.png")),
                            image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let diffuse_texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();
        
        let mut chunk = Vec::new();
        let mut vex_buff = Vec::new();
        for i in 0..8{
            chunk.push(create_voxel_chunk((32*i)as usize,1));
            vex_buff.push(support::load_voxel_chunk(&display,&chunk[i],32.0* i as f32,1.0));
        }
        //let mut chunk = create_voxel_chunk(0.0,0.0);
        
        //create_voxel_chunk(i as f32 *32.0,0.0);
            
        
        //let vex_buff = support::load_voxel_chunk(&display,&chunk[0],0.0,0.0);
        
        return Renderer{
            //event_loop: event_loop,
            display: display,
            diffuse_texture: diffuse_texture,
            vertex_buffer: vex_buff,
            chunk: chunk,
            
        };
    }
    pub fn update_mesh(&mut self,player_direction: u8){
         //self.vertex_buffer = support::load_voxel_chunk(&self.display,&mut self.chunk[0],0.0,0.0);
        // self.vertex_buffer = support::load_voxel_chunk(&self.display,&mut self.chunk[1],32.0,0.0);
    }
    pub fn render_frame(&mut self,camera:&CameraState){
        
        let program = program!(&self.display,
            140 => {
                vertex: "
                    #version 140
    
                    uniform mat4 persp_matrix;
                    uniform mat4 view_matrix;
                    uniform mat4 rot_x_matrix;
                    uniform mat4 rot_y_matrix;
                    in vec3 position;
                    in vec3 normal;
                    in vec2 texture;
    
                    out vec3 v_position;
                    out vec3 v_normal;
                    out vec2 v_tex_coords;
    
                    void main() {
                        v_tex_coords = texture;
                        v_position = position;
                        v_normal = normal;
                        gl_Position = persp_matrix * (rot_x_matrix*(rot_y_matrix * view_matrix * vec4(v_position * 0.005, 1.0)));
                    }
                ",
    
                fragment: "
                    #version 140
    
                    in vec3 v_normal;
                    in vec2 v_tex_coords;
    
                    out vec4 f_color;
                    
                    uniform sampler2D diffuse_tex;
    
                    const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);
                    
    
                    void main() {
                        vec3 diffuse_color = texture(diffuse_tex, v_tex_coords).rgb;
    
                        float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                        vec3 color = (0.3 + 0.7 * lum) * diffuse_color;//vec3(0.988,0.906,0.384);
                        //gl_FragColor = vec4(1.0,0.0,0.9,1.0);
                        gl_FragColor = vec4(color, 1.0);
                    }
                ",
            },
    
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
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };
        let mut target = self.display.draw();
        //173,225,229 0.68,0.88,0.9
        //252 231 98 0.988,0.906,0.384
        
        target.clear_color_and_depth((0.68,0.88,0.9, 0.0), 1.0);
        for i in 0..self.vertex_buffer.len(){
            target.draw(&self.vertex_buffer[i],
            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            &program, &uniforms, &params).unwrap();
        }
        
        
        target.finish().unwrap();

    }
}
