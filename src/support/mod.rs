#![allow(dead_code)]
use std::time::{Duration, Instant};
use glium::{self, Display};
use glium::vertex::VertexBufferAny;
use glium::glutin::event_loop::{EventLoop, ControlFlow};
use glium::glutin::event::{Event, StartCause};
use obj;
use rand::Rng;
pub mod camera;
pub enum Action {
    Stop,
    Continue,
}

pub fn start_loop<F>(event_loop: EventLoop<()>, mut callback: F)->! where F: 'static + FnMut(&Vec<Event<'_, ()>>) -> Action {
    let mut events_buffer = Vec::new();
    let mut next_frame_time = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        let run_callback = match event.to_static() {
            Some(Event::NewEvents(cause)) => {
                match cause {
                    StartCause::ResumeTimeReached { .. } | StartCause::Init => {
                        true
                    },
                    _ => false
                }
            },
            Some(event) => {
                events_buffer.push(event);
                false
            }
            None => {
                // Ignore this event.
                false
            },
        };

        let action = if run_callback {
            let action = callback(&events_buffer);
            next_frame_time = Instant::now() + Duration::from_nanos(0);//16666667
            // TODO: Add back the old accumulator loop in some way

            events_buffer.clear();
            action
        } else {
            Action::Continue
        };

        match action {
            Action::Continue => {
                *control_flow = ControlFlow::WaitUntil(next_frame_time);
            },
            Action::Stop => *control_flow = ControlFlow::Exit
        }
    })
}
#[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
        normal: [f32; 3],
        texture: [f32; 2],
    }
fn get_texture_map_by_id(id:u32)->[[[f32;2];4];6]{
    return match id{
        1=> [[[0.5,1.0],[0.5,0.5],[1.0,0.5],[1.0,1.0]],
        [[0.5,1.0],[0.5,0.5],[1.0,0.5],[1.0,1.0]],
        [[1.0,1.0],[0.5,1.0],[0.5,0.5],[1.0,0.5]],
        [[1.0,1.0],[0.5,1.0],[0.5,0.5],[1.0,0.5]],
        [[0.0,1.0],[0.5,1.0],[0.5,0.5],[0.0,0.5]],
        [[0.0,0.5],[0.5,0.5],[0.5,0.0],[0.0,0.0]]],
        _=>[
            [[0.0,0.5],[0.5,0.5],[0.5,0.0],[0.0,0.0]],
            [[0.0,0.5],[0.5,0.5],[0.5,0.0],[0.0,0.0]],
            [[0.0,0.5],[0.5,0.5],[0.5,0.0],[0.0,0.0]],
            [[0.0,0.5],[0.5,0.5],[0.5,0.0],[0.0,0.0]],
            [[0.0,0.5],[0.5,0.5],[0.5,0.0],[0.0,0.0]],
            [[0.0,0.5],[0.5,0.5],[0.5,0.0],[0.0,0.0]]],
    }
}
fn side(id:u32,side:u8,offset:[f32;3], vert:&mut Vec<Vertex>){
    let norm:[f32;3];
    
    let scale = 10.0;
    //have texture map be 1 or 0 and plus and multiple each for the differnt faces.
    //let mut face=[[0.0 as f32;3];4];
    let face;
    let texturemap;
    let order = [0,1,2,0,2,3];
    
    let texture_mapping = get_texture_map_by_id(id);
    match side{
        0=>{
            norm = [-1.0,0.0,0.0];
            texturemap = texture_mapping[0];
            face = [
                [(offset[0])*scale,(offset[1]    )*scale,(offset[2]    )*scale],
                [(offset[0])*scale,(offset[1]-1.0)*scale,(offset[2]    )*scale],
                [(offset[0])*scale,(offset[1]-1.0)*scale,(offset[2]-1.0)*scale],
                [(offset[0])*scale,(offset[1]    )*scale,(offset[2]-1.0)*scale]];},
        1=>{
            norm = [1.0,0.0,0.0];
            texturemap = texture_mapping[1];
            face = [
                [(offset[0]+1.0)*scale,(offset[1]    )*scale,(offset[2]    )*scale],
                [(offset[0]+1.0)*scale,(offset[1]-1.0)*scale,(offset[2]    )*scale],
                [(offset[0]+1.0)*scale,(offset[1]-1.0)*scale,(offset[2]-1.0)*scale],
                [(offset[0]+1.0)*scale,(offset[1]    )*scale,(offset[2]-1.0)*scale]];},
        2=>{
            norm = [0.0,0.0,1.0];
            texturemap = texture_mapping[2];
            face = [
                [(offset[0]    )*scale,(offset[1]    )*scale,(offset[2])*scale],
                [(offset[0]+1.0)*scale,(offset[1]    )*scale,(offset[2])*scale],
                [(offset[0]+1.0)*scale,(offset[1]-1.0)*scale,(offset[2])*scale],
                [(offset[0]    )*scale,(offset[1]-1.0)*scale,(offset[2])*scale]];},
        3=>{
            norm = [0.0,0.0,-1.0];
            texturemap = texture_mapping[3];
            face = [
                [(offset[0]    )*scale,(offset[1]    )*scale,(offset[2]-1.0)*scale],
                [(offset[0]+1.0)*scale,(offset[1]    )*scale,(offset[2]-1.0)*scale],
                [(offset[0]+1.0)*scale,(offset[1]-1.0)*scale,(offset[2]-1.0)*scale],
                [(offset[0]    )*scale,(offset[1]-1.0)*scale,(offset[2]-1.0)*scale]];},
        4=>{
            //top
            norm = [0.0,1.0,0.0];
            texturemap = texture_mapping[4];
            face = [
                [(offset[0]    )*scale,(offset[1]    )*scale,(offset[2]    )*scale],
                [(offset[0]+1.0)*scale,(offset[1]    )*scale,(offset[2]    )*scale],
                [(offset[0]+1.0)*scale,(offset[1]    )*scale,(offset[2]-1.0)*scale],
                [(offset[0]    )*scale,(offset[1]    )*scale,(offset[2]-1.0)*scale]];},
        _=>{
            texturemap = texture_mapping[5];
            norm = [0.0,-1.0,0.0];
            face = [
                [(offset[0]    )*scale,(offset[1]-1.0)*scale,(offset[2]    )*scale],
                [(offset[0]+1.0)*scale,(offset[1]-1.0)*scale,(offset[2]    )*scale],
                [(offset[0]+1.0)*scale,(offset[1]-1.0)*scale,(offset[2]-1.0)*scale],
                [(offset[0]    )*scale,(offset[1]-1.0)*scale,(offset[2]-1.0)*scale]];},
    };
    for i in order{
        let point = [face[i][0],face[i][1],face[i][2]];
        let text = texturemap[i];
        vert.push(Vertex{position: point,normal:norm,texture:text,});
    }

}
pub fn load_voxel_chunk(display: &Display)->VertexBufferAny{
    implement_vertex!(Vertex, position, normal, texture);
    let mut vertex_data: Vec<Vertex> = Vec::new();
    let mut chunk = [[[0;32];32];32];
    
   let mut ra =rand::thread_rng();
    for _i in 0..1640{
        chunk[ra.gen_range(0..32)][ra.gen_range(0..32)][ra.gen_range(0..32)] =ra.gen_range(1..3);
    }
    
    //codes 0 = west, 1 = east, 2 = front, 3 = back, 4 = top, 5 = bottom
    for x in 0..chunk.len(){
        for y in 0..chunk[0].len(){
            for z in 0..chunk[0][0].len(){
                if chunk[x][y][z] > 0{
                    if x as i64 -1 >= 0{
                        if chunk[x-1][y][z] == 0{
                            //west side
                            side(chunk[x][y][z],0,[x as f32,y as f32,z as f32],&mut vertex_data);
                        }
                    }else{
                        //fail
                        side(chunk[x][y][z],0,[x as f32,y as f32,z as f32],&mut vertex_data);
                    }
                    if x as i64 +1 <= chunk.len() as i64-1{
                        if chunk[x+1][y][z] == 0{
                            side(chunk[x][y][z],1,[x as f32,y as f32,z as f32],&mut vertex_data);
                        }
                    }else{
                        //fail
                        side(chunk[x][y][z],1,[x as f32,y as f32,z as f32],&mut vertex_data);
                    }
                    //top bottom
                    if y as i64 +1 <= chunk[0].len() as i64-1{
                        if chunk[x][y+1][z] == 0{
                            side(chunk[x][y][z],4,[x as f32,y as f32,z as f32],&mut vertex_data);
                        }
                    }else{
                        side(chunk[x][y][z],4,[x as f32,y as f32,z as f32],&mut vertex_data);
                    }
                    if y as i64 -1 >= 0{
                        if chunk[x][y-1][z] == 0{
                            side(chunk[x][y][z],5,[x as f32,y as f32,z as f32],&mut vertex_data);
                        }
                    }else{
                        side(chunk[x][y][z],5,[x as f32,y as f32,z as f32],&mut vertex_data);
                    }
                    //front back
                    if z as i64 +1 <= chunk[0][0].len() as i64-1{
                        if chunk[x][y][z+1] == 0{
                            //west side
                            side(chunk[x][y][z],2,[x as f32,y as f32,z as f32],&mut vertex_data);
                        }
                    }else{
                        //fail
                        side(chunk[x][y][z],2,[x as f32,y as f32,z as f32],&mut vertex_data);
                    }
                    if z as i64 -1 >= 0{
                        if chunk[x][y][z-1] == 0{
                            //west side
                            side(chunk[x][y][z],3,[x as f32,y as f32,z as f32],&mut vertex_data);
                        }
                    }else{
                        //fail
                        side(chunk[x][y][z],3,[x as f32,y as f32,z as f32],&mut vertex_data);
                    }

                }
            }
        }
    }

    glium::vertex::VertexBuffer::new(display, &vertex_data).unwrap().into()

}
/// Returns a vertex buffer that should be rendered as `TrianglesList`.
pub fn load_wavefront(display: &Display, data: &[u8]) -> VertexBufferAny {
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
        normal: [f32; 3],
        texture: [f32; 2],
    }

    implement_vertex!(Vertex, position, normal, texture);

    let mut data = ::std::io::BufReader::new(data);
    let data = obj::ObjData::load_buf(&mut data).unwrap();

    let mut vertex_data = Vec::new();

    for object in data.objects.iter() {
        for polygon in object.groups.iter().flat_map(|g| g.polys.iter()) {
            match polygon {
                obj::SimplePolygon(indices) => {
                    for v in indices.iter() {
                        let position = data.position[v.0];
                        let texture = v.1.map(|index| data.texture[index]);
                        let normal = v.2.map(|index| data.normal[index]);

                        let texture = texture.unwrap_or([0.0, 0.0]);
                        let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                        vertex_data.push(Vertex {
                            position,
                            normal,
                            texture,
                        })
                    }
                },
            }
        }
    }

    glium::vertex::VertexBuffer::new(display, &vertex_data).unwrap().into()
}
