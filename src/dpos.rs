
pub fn dbox_cross(dx:i16, dy:i16, sw:i16, sh:i16) -> bool {

    dx <= sw && dx >= -sw &&
    dy >= -sh && dy <= sh
}

pub fn dpos_cross(dx:i16, dy:i16, agent_size:i16) -> bool {

    dx.abs() <= agent_size && 
    dy.abs() <= agent_size
}

pub fn dpos_at_front(dir:u8, dx:i16, dy:i16) -> bool {
        
    match dir {
        1 => dx >= 0 && dy <= 0,
        2 => dx >= dy.abs(),
        3 => dx >= 0 && dy >= 0,
        4 => dy >= dx.abs(),
        5 => dx <= 0 && dy >= 0,
        6 => dx <= -dy.abs(),
        7 => dx <= 0 && dy <= 0,
        _ => dy <= -dx.abs(),
    }
}

pub fn dpos_cross_dirs(dirs:&mut [bool;8], dx:i16, dy:i16) {

    if dx >= 0 {

        if dy >= dx.abs() {

            dirs[2] = true;
            dirs[3] = true;
            dirs[4] = true;
            dirs[5] = true;
            return;                
        }

        if dy >= 0 {
            dirs[1] = true;
            dirs[2] = true;
            dirs[3] = true;
            dirs[4] = true;
            return;
        }

        if dy < -dx.abs() {
            dirs[7] = true;
            dirs[0] = true;
            dirs[1] = true;
            dirs[2] = true;
            return;
        }
        
        dirs[0] = true;
        dirs[1] = true;
        dirs[2] = true;
        dirs[3] = true;
        return;
    }

    if dy >= dx.abs() {
        dirs[3] = true;
        dirs[4] = true;
        dirs[5] = true;
        dirs[6] = true;
        return;                
    }

    if dy >= 0 {
        dirs[4] = true;
        dirs[5] = true;
        dirs[6] = true;
        dirs[7] = true;
        return;
    }

    if dy < -dx.abs() {
        dirs[6] = true;
        dirs[7] = true;
        dirs[0] = true;
        dirs[1] = true;
        return;
    }
    
    dirs[5] = true;
    dirs[6] = true;
    dirs[7] = true;
    dirs[0] = true;
    return;
}