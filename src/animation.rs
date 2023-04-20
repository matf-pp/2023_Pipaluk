
#[derive(Clone)]
pub struct Animation {
    pub path: Vec<(i32, i32)>,
    pub sprites: Vec<String>,
    pub current: usize,
}

impl Animation {
    pub fn init(points: Vec<(i32, i32)>, sprites: Vec<String>, duration: i32) -> Self {
        println!("Animation started");
        let mut path: Vec<(i32, i32)> = vec![];
        for i in 0..points.len()-1 {
            let (pcx, pcy) = points[i];   // curr point
            let (pnx, pny) = points[i+1]; // next point
            for j in 0..duration {
                let x = pcx + (pnx-pcx)*j/duration;
                let y = pcy + (pny-pcy)*j/duration;
                path.push((x, y));
            }
        }
        path.push(points[points.len()-1]);
        Self {
            path: path,
            sprites: sprites,
            current: 0
        }
    }

    pub fn next_frame(&mut self) -> ((i32, i32), String, bool, Option<bool>) {
        self.current += 1;
        let mut flipped: Option<bool> = None;
        if !self.finished() {
            flipped = Some(self.path[self.current-1].0-self.path[self.current].0 > 0);
        }
        (self.path[self.current-1], self.sprites[self.current%self.sprites.len()].clone(), self.finished(), flipped)
    }

    pub fn finished(&self) -> bool {
        self.current == self.path.len()
    }
}