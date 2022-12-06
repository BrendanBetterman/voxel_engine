#![allow(dead_code)]
use std::time::{Duration, Instant};
use glium::{self, Display};
use glium::vertex::VertexBufferAny;
use glium::glutin::event_loop::{EventLoop, ControlFlow};
use glium::glutin::event::{Event, StartCause};
use obj;
use rand::Rng;
use noise::{NoiseFn, Perlin, Seedable};
pub mod camera;
pub enum Action {
    Stop,
    Continue,
}
pub enum Blocks{
    Air,
    Dirt,
    Grass,
    Stone,
    Amethyst,
    WormWhole,
    Graphite,
}
pub struct Chunk{
    pub chunk:[[[u8;32];32];32],
    pub pos:[usize;2],
}
impl Chunk{
    pub fn new()->Chunk{
        Chunk{
            chunk:[[[0;32];32];32],
            pos:[0,0],
        }
    }
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
fn get_map(image_x:u32,image_y:u32,flipped:bool)->[[f32;2];4]{
    let (width,height) = (4,4);
    let x_off = 1.0/width as f32;
    let y_off = 1.0/height as f32;
    let x = (image_x) as f32 * x_off;
    let y = (height-image_y) as f32 * y_off;
    let x_buffer:f32;
    if x >=0.01{
        x_buffer = 0.01;
    }else{
        x_buffer = 0.0;
    }
    let y_buffer:f32;
    if y >=0.01{
        y_buffer = 0.01;
    }else{
        y_buffer = 0.0;
    }
    if flipped{
        return [
            [x+x_off-x_buffer,y      -y_buffer],
            [x      +x_buffer,y      -y_buffer],
            [x      +x_buffer,y-y_off+y_buffer],
            [x+x_off-x_buffer,y-y_off+y_buffer]
            ];
    }else{
        return [
            [x+x_off-x_buffer,y      -y_buffer],
            [x+x_off-x_buffer,y-y_off+y_buffer],
            [x      +x_buffer,y-y_off+y_buffer],
            [x      +x_buffer,y      -y_buffer]
            ];
    }
   
}
fn get_texture_map_by_id(id:Blocks)->[[[f32;2];4];6]{
    return match id{
        Blocks::Grass=>[
            get_map(1, 0,false),//west
            get_map(1, 0,false),//east
            get_map(1, 0,true),//front
            get_map(1, 0,true),//back
            get_map(0, 0,false),//top
            get_map(0, 1,false)],//bottom
        Blocks::Graphite=>[
            get_map(3, 1,false),//west
            get_map(3, 1,false),//east
            get_map(3, 1,true),//front
            get_map(3, 1,true),//back
            get_map(3, 1,false),//top
            get_map(3, 1,false)],//bottom
        Blocks::Amethyst=>[
            get_map(0, 2,false),//west
            get_map(0, 2,false),//east
            get_map(0, 2,true),//front
            get_map(0, 2,true),//back
            get_map(0, 2,false),//top
            get_map(0, 2,false)],//bottom
        Blocks::Dirt=>[
            get_map(0, 1,false),//west
            get_map(0, 1,false),//east
            get_map(0, 1,true),//front
            get_map(0, 1,true),//back
            get_map(0, 1,false),//top
            get_map(0, 1,false)],//bottom
        Blocks::Stone=>[
            get_map(1, 2,false),//west
            get_map(1, 2,false),//east
            get_map(1, 2,true),//front
            get_map(1, 2,true),//back
            get_map(1, 2,false),//top
            get_map(1, 2,false)],//bottom
        Blocks::WormWhole=>[
            get_map(3, 2,false),//west
            get_map(3, 2,false),//east
            get_map(3, 2,true),//front
            get_map(3, 2,true),//back
            get_map(2, 2,false),//top
            get_map(2, 2,false)],//bottom
        _=>[
            get_map(0, 1,false),//west
            get_map(0, 1,false),//east
            get_map(0, 1,true),//front
            get_map(0, 1,true),//back
            get_map(0, 1,false),//top
            get_map(0, 1,false)],//bottom
        /* 
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
            */
    }
}
fn id_to_blocks(id:u8)->Blocks{
    match id{
       
        1=>Blocks::Grass,
        2=>Blocks::Graphite,
        3=>Blocks::Amethyst,  
        4=>Blocks::Dirt,
        5=>Blocks::Stone, 
        6=>Blocks::WormWhole,
        _=> Blocks::Air,
    }
}
fn side(id:u8,side:u8,offset:[f32;3], vert:&mut Vec<Vertex>){
    let norm:[f32;3];
    
    let scale = 10.0;
    //have texture map be 1 or 0 and plus and multiple each for the differnt faces.
    //let mut face=[[0.0 as f32;3];4];
    let face;
    let texturemap;
    //let order = [0,1,2,0,2,3];
    let mut order = [0,1,2,0,2,3];
    if side%2 ==0{
        order = [1,0,2,0,3,2];
    }

    let texture_mapping = get_texture_map_by_id(id_to_blocks(id));
    
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
                //bottom
                texturemap = texture_mapping[5];
                norm = [0.0,-1.0,0.0];
                face = [
                    [(offset[0]    )*scale,(offset[1]-1.0)*scale,(offset[2]    )*scale],
                    [(offset[0]+1.0)*scale,(offset[1]-1.0)*scale,(offset[2]    )*scale],
                    [(offset[0]+1.0)*scale,(offset[1]-1.0)*scale,(offset[2]-1.0)*scale],
                    [(offset[0]    )*scale,(offset[1]-1.0)*scale,(offset[2]-1.0)*scale]];},
            _=>{
                //top
                norm = [0.0,1.0,0.0];
                texturemap = texture_mapping[4];
                face = [
                    [(offset[0]    )*scale,(offset[1]    )*scale,(offset[2]    )*scale],
                    [(offset[0]+1.0)*scale,(offset[1]    )*scale,(offset[2]    )*scale],
                    [(offset[0]+1.0)*scale,(offset[1]    )*scale,(offset[2]-1.0)*scale],
                    [(offset[0]    )*scale,(offset[1]    )*scale,(offset[2]-1.0)*scale]];},
            
        };
        
        for i in order{
            let point = [face[i][0],face[i][1],face[i][2]];
            let text = texturemap[i];
            vert.push(Vertex{position: point,normal:norm,texture:text,});
        }
    
        for i in order{
            let point = [face[i][0],face[i][1],face[i][2]];
            let text = texturemap[i];
            vert.push(Vertex{position: point,normal:norm,texture:text,});
        }
        
    
}
fn worm(chunk: &mut Chunk,pos: &mut [i32;2]){
    let CHUNKSIZE:i32 = chunk.chunk.len() as i32;
    let mut ra =rand::thread_rng();
    let height = ra.gen_range(1..5);
    for y in height..height+ra.gen_range(1..20){
        for i in (-2)..2{
            for u in (-2)..2{
                if pos[0] + i > 0 && pos[0] + i < CHUNKSIZE && pos[1] + u > 0 && pos[1] + u < CHUNKSIZE{
                    for y2 in y..y+2{
                        chunk.chunk[ (pos[0] + i) as usize][y2][(pos[1]+u)as usize] = 0;
                    }
                }
            }
        }
        let movex = ra.gen_range(-2..2) +pos[0];
        let movez = ra.gen_range(-2..2) + pos[1];
        if movex > 0 && movex < CHUNKSIZE{
            pos[0] = movex;
        }
        if movez > 0 && movez < CHUNKSIZE{
            pos[1] = movez;
        }

    }
    
}
pub fn create_voxel_chunk(chunk_x:usize,chunk_z:usize,seed:u32)-> Chunk{
    const CHUNKSIZE:usize  = 32;
    let mut chunk = Chunk::new();
    chunk.pos = [chunk_x,chunk_z];
    let mut ra =rand::thread_rng();
    /*for _i in 0..100{
        chunk[ra.gen_range(0..CHUNKSIZE)][ra.gen_range(0..CHUNKSIZE)][ra.gen_range(0..CHUNKSIZE)] =ra.gen_range(1..3);
    }*/
    let perlin = Perlin::new(seed);
    let smoothness = 0.035;
    let slope = 7.5;
    let baseheight = 3.0;
    //world
    
    for x in 0..CHUNKSIZE{
        for z in 0..CHUNKSIZE{
            let height = ((perlin.get([(x + chunk_x)as f64 * smoothness,(z + chunk_z)as f64* smoothness,1.0])+baseheight)*slope)as usize;
            for y in 0..height{
                if y < height -3{
                    let orespawn = ra.gen_range(0..10);
                    if orespawn == 9{
                        chunk.chunk[x][y][z] = 3;
                    }else if orespawn == 7{
                        chunk.chunk[x][y][z] = 6;
                    }else{
                        chunk.chunk[x][y][z] = 5;
                    }
                }else if y< height -1 {
                    chunk.chunk[x][y][z] = 4;
                }else{
                    chunk.chunk[x][y][z] = 1;
                }
            }

        } 
    }
    //cave
    for _i in 0..ra.gen_range(5..20){
        let mut pos = [ra.gen_range(0..CHUNKSIZE as i32),ra.gen_range(0..CHUNKSIZE as i32)];
        worm(&mut chunk,&mut pos);
    }
    
    return chunk;
}
pub fn load_voxel_chunk<const Size:usize>(display: &Display,chunk: &[[[u8;Size];Size];Size],chunk_x:f32,chunk_z:f32)->VertexBufferAny{
    implement_vertex!(Vertex, position, normal, texture);
    let mut vertex_data: Vec<Vertex> = Vec::new();
    //Algo Only check East, South and the top if its air check the next and set the side to the next type.
    for x in 0..chunk.len()-1{ //-1 saves checking checking for edge case
        for y in 0..chunk[0].len()-1{
            for z in 0..chunk[0][0].len()-1{
                if chunk[x][y][z] != 0{ //if not air
                    if chunk[x+1][y][z] == 0{//if east is air generate face of this type
                        side(chunk[x][y][z],1,[x as f32+chunk_x,y as f32,z as f32+chunk_z],&mut vertex_data);
                    }
                    if chunk[x][y][z+1] == 0{//south
                        side(chunk[x][y][z],2,[x as f32+chunk_x,y as f32,z as f32+chunk_z],&mut vertex_data);
                    }
                    if chunk[x][y+1][z] == 0{//top
                        side(chunk[x][y][z],5,[x as f32+chunk_x,y as f32,z as f32+chunk_z],&mut vertex_data);
                    }
                } else {//if air
                    if chunk[x+1][y][z] != 0{//east
                        side(chunk[x+1][y][z],0,[x as f32+1.0+chunk_x,y as f32,z as f32+chunk_z],&mut vertex_data);
                    }
                    if chunk[x][y][z+1] != 0{//south
                        side(chunk[x][y][z+1],3,[x as f32+chunk_x,y as f32,z as f32+1.0+chunk_z],&mut vertex_data);
                    }
                    if chunk[x][y+1][z] !=0{//bottom of above
                        side(chunk[x][y+1][z],4,[x as f32+chunk_x,y as f32 +1.0,z as f32+chunk_z],&mut vertex_data);
                    }
                }
            }
        }
    }
    //east and south face of chunk
    for z in 0..chunk.len(){
        for y in 0..chunk[0].len()-1{
            if chunk[chunk[0][0].len()-1][y][z] != 0{ //if not air
                side(chunk[chunk[0][0].len()-1][y][z],1,[(chunk[0][0].len()) as f32-1.0+chunk_x,y as f32,z as f32+chunk_z],&mut vertex_data);
            }
            if chunk[0][y][z] != 0{ //if not air
                side(chunk[0][y][z],0,[0.0+chunk_x,y as f32,z as f32+chunk_z],&mut vertex_data);
            }
            if chunk[z][y][chunk[0][0].len()-1] != 0{ //if not air
                side(chunk[z][y][chunk[0][0].len()-1],2,[z as f32+chunk_x,y as f32,(chunk[0][0].len()) as f32-1.0+chunk_z],&mut vertex_data);
            }
            if chunk[z][y][0] != 0{ //if not air
                side(chunk[z][y][0],3,[z as f32+chunk_x,y as f32,0.0+chunk_z],&mut vertex_data);
            }
        }
    }
    //tops and inner side of blocks on edge
    for x in 0..chunk.len()-1{
        for y in 0..chunk.len()-1{
            if chunk[x][y][chunk.len()-1] != 0{ //if not air
                if chunk[x+1][y][chunk.len()-1] == 0{//if east is air generate face of this type
                    side(chunk[x][y][chunk.len()-1],1,[x as f32+chunk_x,y as f32,(chunk.len()-1) as f32+chunk_z],&mut vertex_data);
                }
                if chunk[x][y+1][chunk.len()-1] == 0{//top
                    side(chunk[x][y][chunk.len()-1],5,[x as f32+chunk_x,y as f32,(chunk.len()-1) as f32+chunk_z],&mut vertex_data);
                }
            } else {//if air
                if chunk[x+1][y][chunk.len()-1] != 0{//east
                    side(chunk[x+1][y][chunk.len()-1],0,[x as f32+1.0+chunk_x,y as f32,(chunk.len()-1) as f32+chunk_z],&mut vertex_data);
                }
                if chunk[x][y+1][chunk.len()-1] !=0{//bottom of above
                    side(chunk[x][y+1][chunk.len()-1],4,[x as f32+chunk_x,y as f32 +1.0,(chunk.len()-1) as f32+chunk_z],&mut vertex_data);
                }
            }
            if chunk[chunk.len()-1][y][x] != 0{ //if not air
                if chunk[chunk.len()-1][y][x+1] == 0{
                    side(chunk[chunk.len()-1][y][x],2,[(chunk.len()-1) as f32+chunk_x,y as f32,x as f32+chunk_z],&mut vertex_data);
                }
                if chunk[chunk.len()-1][y+1][x] == 0{//top
                    side(chunk[chunk.len()-1][y][x],5,[(chunk.len()-1) as f32+chunk_x,y as f32,x as f32+chunk_z],&mut vertex_data);
                }
            } else {//if air
                if chunk[chunk.len()-1][y][x+1] != 0{
                    side(chunk[chunk.len()-1][y][x+1],3,[(chunk.len()-1) as f32+chunk_x,y as f32,x as f32+1.0+chunk_z],&mut vertex_data);
                }
                if chunk[chunk.len()-1][y+1][x+1] !=0{//bottom of above
                    side(chunk[chunk.len()-1][y+1][x],4,[(chunk.len()-1) as f32+chunk_x,y as f32 +1.0,x as f32+chunk_z],&mut vertex_data);
                }
            }
        }
    }
    //Last collumn
    for y in 0..chunk.len()-1{
        if chunk[chunk.len()-1][y][chunk.len()-1] != 0{ 
            if chunk[chunk.len()-1][y+1][chunk.len()-1] == 0{//top
                side(chunk[chunk.len()-1][y][chunk.len()-1],5,[(chunk.len()-1) as f32+chunk_x,y as f32,(chunk.len()-1) as f32+chunk_z],&mut vertex_data);
            }
        }else{
            if chunk[chunk.len()-1][y+1][chunk.len()-1] !=0{//bottom of above
                side(chunk[chunk.len()-1][y+1][chunk.len()-1],4,[(chunk.len()-1) as f32+chunk_x,y as f32 +1.0,(chunk.len()-1) as f32+chunk_z],&mut vertex_data);
            }
        }
    }
    
    return glium::vertex::VertexBuffer::new(display, &vertex_data).unwrap().into();
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
