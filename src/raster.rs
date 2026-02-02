use indicatif::{ProgressBar, ProgressStyle};
use crate::color::{write_color, Color, BlendMode};

pub struct Canvas {
    //pub buffer: Vec<Color>,
    pub width: usize,
    pub height: usize,
    pub default_color: Color,
}

// Vec2 built specifically to handle integer positions on a raster (grid)
#[derive(Copy, Clone, Debug)]
pub struct Pos2 {
    pub e: [i32; 2],
}

impl Pos2 {
    pub fn new(x: i32, y: i32) -> Pos2 {
        Pos2 { e: [x, y] }
    }

    pub fn x(&self) -> i32 {
        self.e[0]
    }

    pub fn y(&self) -> i32 {
        self.e[1]
    }
}

// ================= Define Polygon, Polygon types =======================
pub trait Polygon {
    fn draw(&self, canvas: &Canvas, buffer: &mut Vec<Color>);
    fn draw_fill(&self, canvas: &Canvas, buffer: &mut Vec<Color>);
}

pub struct Triangle {
    pub vertices: [(i32, i32); 3],
    pub color: Color,
}

pub struct Circle {
    pub radius: i32,
    pub center: (i32, i32),
    pub color: Color,
}

// Assumed that vertices are ordered in a clockwise manner
pub struct Quad {
    pub vertices: [Pos2; 4],
    pub color: Color,
}

pub struct Square {
    pub center: Pos2,
    pub side_length: i32,
    pub color: Color,
}

// =============== Implement draw functions for polygons =================
impl Polygon for Triangle {
    fn draw(&self, canvas: &Canvas, buffer: &mut Vec<Color>) {
        for i in 0..self.vertices.len() {
            let next_i = if i == self.vertices.len() - 1 { 0 } else { i + 1 };
            draw_line_aa(buffer, canvas, self.vertices[i].0, self.vertices[i].1, self.vertices[next_i].0, self.vertices[next_i].1, &self.color);
        }
    }
    fn draw_fill(&self, canvas: &Canvas, buffer: &mut Vec<Color>) {
        let mut v_list = self.vertices.clone();
        v_list.sort_by_key(|v| v.1);

        // Calculate diff in y for each vertex pair in tri
        let dy_01 = v_list[1].1 - v_list[0].1;
        let dy_02 = v_list[2].1 - v_list[0].1;
        let dy_12 = v_list[2].1 - v_list[1].1;

        if dy_01 == 0 && dy_02 == 0 { return; } // This indicates a flat line, executive decision to not draw this

        // Work on upper part of the triangle
        if dy_01 != 0 {
            let im_01 = (v_list[1].0 as f64 - v_list[0].0 as f64) / dy_01 as f64;
            let im_02 = (v_list[2].0 as f64 - v_list[0].0 as f64) / dy_02 as f64;
            let y0 = v_list[0].1;
            let x0 = v_list[0].0;

            for i in 0..=dy_01 {
                let y = y0 + i;
                let mut left_x = (im_02 * i as f64).round() as i32 + x0;
                let mut right_x = (im_01 * i as f64).round() as i32 + x0;

                if left_x > right_x {
                    (left_x, right_x) = (right_x, left_x);
                }

                for j in left_x..=right_x {
                    draw_pixel(buffer, canvas, j, y, self.color, BlendMode::None);
                }
            }
        }

        // Work on lower part of the triangle
        if dy_12 != 0 {
            let im_12 = (v_list[2].0 as f64 - v_list[1].0 as f64) / dy_12 as f64;
            let im_02 = (v_list[2].0 as f64 - v_list[0].0 as f64) / dy_02 as f64;
            let y0 = v_list[0].1;
            let y1 = v_list[1].1;
            let x0 = v_list[0].0;
            let x1 = v_list[1].0;

            for i in 0..=dy_12 {
                let y = y1 + i;
                let mut left_x = (im_02 * (i + y1 - y0) as f64).round() as i32 + x0;
                let mut right_x = (im_12 * i as f64).round() as i32 + x1;

                if left_x > right_x {
                    (left_x, right_x) = (right_x, left_x);
                }

                for j in left_x..=right_x {
                    draw_pixel(buffer, canvas, j, y, self.color, BlendMode::None);
                }
            }
        }
    }
}

impl Polygon for Circle {
    fn draw(&self, canvas: &Canvas, buffer: &mut Vec<Color>) {}
    fn draw_fill(&self, canvas: &Canvas, buffer: &mut Vec<Color>) {
        let center_x = self.center.0;
        let center_y = self.center.1;
        let r_2 = self.radius * self.radius;

        // Calculate bounding box
        let x_min = self.center.0 - self.radius;
        let x_max = self.center.0 + self.radius;
        let y_min = self.center.1 - self.radius;
        let y_max = self.center.1 + self.radius;

        for j in y_min..y_max {
            for i in x_min..x_max {
                if (i as f64 - center_x as f64).powi(2) + (j as f64 - center_y as f64).powi(2) <= r_2 as f64 {
                    draw_pixel(buffer, canvas, i, j, self.color, BlendMode::Add);
                }
            }
        }
    }
}

impl Polygon for Quad {
    fn draw(&self, canvas: &Canvas, buffer: &mut Vec<Color>) {
        for i in 0..self.vertices.len() {
            let next_i = if i == self.vertices.len() - 1 { 0 } else { i + 1 };
            draw_line_aa(buffer, canvas, self.vertices[i].x(), self.vertices[i].y(), self.vertices[next_i].x(), self.vertices[next_i].y(), &self.color);
        }
    }
    fn draw_fill(&self, canvas: &Canvas, buffer: &mut Vec<Color>) {
        let v_list = self.vertices.clone();

        // Separate the quad into two tris
        let tri1 = Triangle {
            vertices: [(v_list[0].x(), v_list[0].y()), (v_list[1].x(), v_list[1].y()), (v_list[2].x(), v_list[2].y())],
            color: self.color,
        };
        let tri2 = Triangle {
            vertices: [(v_list[2].x(), v_list[2].y()), (v_list[3].x(), v_list[3].y()), (v_list[0].x(), v_list[0].y())],
            color: self.color,
        };

        tri1.draw_fill(canvas, buffer);
        tri2.draw_fill(canvas, buffer);
    }
}

impl Quad {
    // Performs a 2D rotation in the xy plane around the given pivot point, returns a new quad.
    // Angle received as degrees, converted to radians
    pub fn rotate(&self, angle: f64, pivot: Pos2) -> Quad {
        let angle_radians = angle.to_radians();
        let v_list = self.vertices.clone();
        let mut new_v_list = [Pos2 { e: [0, 0]}; 4];
        for (i, v) in v_list.iter().enumerate() {
            let current_x = v.x() as f64 - pivot.x() as f64; // FLAG: Check if this works later
            let current_y = v.y() as f64 - pivot.y() as f64;
            let new_x = current_x * angle_radians.cos() - current_y * angle_radians.sin();
            let new_y = current_x * angle_radians.sin() + current_y * angle_radians.cos();
            new_v_list[i] = Pos2 {e: [new_x as i32 + pivot.x(), new_y as i32 + pivot.y()]};
        }

        Quad { vertices: new_v_list, color: self.color }
    }
}

impl Polygon for Square {
    fn draw(&self, canvas: &Canvas, buffer: &mut Vec<Color>) {
        // Step 1: Calculate position of all 4 corner vertices
        let center = self.center;
        let dist_from_center = self.side_length / 2;
        let pos0 = Pos2::new(center.x() - dist_from_center, center.y() - dist_from_center);
        let pos1 = Pos2::new(center.x() + dist_from_center, center.y() - dist_from_center);
        let pos2 = Pos2::new(center.x() + dist_from_center, center.y() + dist_from_center);
        let pos3 = Pos2::new(center.x() - dist_from_center, center.y() + dist_from_center);

        // Step 2: Create a quad based on the generated positions
        let quad1 = Quad {
            vertices: [pos0, pos1, pos2, pos3],
            color: self.color,
        };

        // Step 3: Draw
        quad1.draw(canvas, buffer);
    }
    fn draw_fill(&self, canvas: &Canvas, buffer: &mut Vec<Color>) {
        // Step 1: Calculate position of all 4 corner vertices
        let center = self.center;
        let dist_from_center = self.side_length / 2;
        let pos0 = Pos2::new(center.x() - dist_from_center, center.y() - dist_from_center);
        let pos1 = Pos2::new(center.x() + dist_from_center, center.y() - dist_from_center);
        let pos2 = Pos2::new(center.x() + dist_from_center, center.y() + dist_from_center);
        let pos3 = Pos2::new(center.x() - dist_from_center, center.y() + dist_from_center);

        // Step 2: Create a quad based on the generated positions
        let quad1 = Quad {
            vertices: [pos0, pos1, pos2, pos3],
            color: self.color,
        };

        // Step 3: Draw
        quad1.draw_fill(canvas, buffer);
    }
}

pub fn output_ppm(c: &Canvas, buffer: &Vec<Color>) {
    // Progress bar settings
    let progress_bar = ProgressBar::new(c.height as u64);
    progress_bar.set_style(ProgressStyle::with_template("{prefix} {bar:80.cyan/blue} {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("##-"));
    progress_bar.set_prefix("Scanlines completed...");

    // Render

    println!("P3\n{} {}\n255", c.width, c.height);

    for j in 0..c.height as u32 {
        progress_bar.inc(1);

        for i in 0..c.width as u32 {
            write_color(&buffer[j as usize * c.width as usize + i as usize]);
        }
    }
}

// Function for drawing anti-aliased lines
// Implements Xiaolin Wu's algorithm, without accounting for overlap cuz it's stupid
pub fn draw_line_aa(buffer: &mut Vec<Color>, c: &Canvas, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: &Color) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    if dx > dy {
        if x0 > x1 {
            (x0, x1) = (x1, x0);
            (y0, y1) = (y1, y0);
        }

        let m = (y1 - y0) as f64 / (x1 - x0) as f64;

        for i in 0..=dx {
            let y = y0 as f64 + (i as f64 * m);
            let ix = x0 + i;
            let iy = y.floor() as i32;
            let dist = y - iy as f64;
            draw_pixel(buffer, c, ix, iy, *color * (1.0-dist), BlendMode::None);
            draw_pixel(buffer, c, ix, iy + 1, *color * dist, BlendMode::None);
        }
    } else {
        if y0 > y1 {
            (x0, x1) = (x1, x0);
            (y0, y1) = (y1, y0);
        }

        let m = (x1 - x0) as f64 / (y1 - y0) as f64;

        for i in 0..=dy {
            let x = x0 as f64 + (i as f64 * m);
            let ix = x.floor() as i32;
            let iy = y0 + i;
            let dist = x - ix as f64;
            draw_pixel(buffer, c, ix, iy, (*color * (1.0-dist)).clone(), BlendMode::None);
            draw_pixel(buffer, c, ix + 1, iy, (*color * dist).clone(), BlendMode::None);
        }
    }
}

pub fn draw_line(buffer: &mut Vec<Color>, c: &Canvas, x0: i32, y0: i32, x1: i32, y1: i32, color: &Color) {
    if (x1 - x0).abs() > (y1 - y0).abs() {
        draw_line_h(buffer, c, x0, y0, x1, y1, color);
    } else {
        draw_line_v(buffer, c, x0, y0, x1, y1, color);
    }
}

fn draw_line_h(buffer: &mut Vec<Color>, c: &Canvas, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: &Color) {
    if x0 > x1 {
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
    }

    let dx = x1 - x0;
    let dy =  y1 - y0;

    let dir = if dy < 0 { -1 } else { 1 };
    let dy =  dy * dir;

    if dx != 0 {
        let mut y = y0;
        let mut p = 2 * dy - dx;

        for i in 0..dx+1 {
            draw_pixel(buffer, &c, x0 + i, y, color.clone(), BlendMode::None);
            if p >= 0 {
                y += dir;
                p = p - 2 * dx;
            }
            p = p + 2 * dy;
        }
    }
}

fn draw_line_v(buffer: &mut Vec<Color>, c: &Canvas, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: &Color) {
    if y0 > y1 {
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
    }

    let dx = x1 - x0;
    let dy =  y1 - y0;

    let dir = if dx < 0 { -1 } else { 1 };
    let dx =  dx * dir;

    if dy != 0 {
        let mut x = x0;
        let mut p = 2 * dx - dy;

        for i in 0..dy+1 {
            draw_pixel(buffer, &c, x, y0 + i, color.clone(), BlendMode::None);
            if p >= 0 {
                x += dir;
                p = p - 2 * dy;
            }
            p = p + 2 * dx;
        }
    }
}

// We use i32 for x,y even though Canvas does not have negative values so that we can draw
// triangles which go outside our Canvas
fn draw_pixel(buffer: &mut Vec<Color>, c: &Canvas, x: i32, y: i32, color: Color, blend_mode: BlendMode) {

    if buffer.is_empty() { return; }
    // Handle out-of-bounds coordinates
    if x < 0 || y < 0 || x >= c.width as i32 || y >= c.height as i32 { return; }

    match blend_mode {
        BlendMode::None => { buffer[y as usize * c.width + x as usize] = color; }
        BlendMode::Add => {
            let current_color = buffer[y as usize * c.width + x as usize];
            buffer[y as usize * c.width + x as usize] = color + current_color;
        }
        BlendMode::Subtract => {}
        BlendMode::Multiply => {}
        BlendMode::AlphaBlend => {}
    }
}