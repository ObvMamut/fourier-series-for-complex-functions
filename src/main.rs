use ggez::{event, glam::*, graphics::{self, Color, Mesh}, Context, GameResult};
use std::{process};
use ggez::event::EventHandler;
use num::complex::Complex;
use std::io;

mod data {
    const GRID_SIZE: (i16, i16) = (160,110); // 22*12 for tetris screen | 10*22 for title | 32*10 for additional info
    const GRID_CELL_SIZE: (i16, i16) = (16, 16); // 1 grid = 64 px * 64 px

    pub(crate) const SCREEN_SIZE: (f32, f32) = (
        GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
        GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
    );

    pub(crate) const DESIRED_FPS: u32 = 60;

    pub(crate) const SCALE_UP_CONST: f32 = 1.0;
}

struct Fourier_Serie {
    vectors: Vec<Vec<f32>>,
    constants: Vec<Vec<f32>>,
    t: f32,
    speed_factor: i64,
    n: i64,
    list_n: Vec<i64>,
}

impl Fourier_Serie {


    fn init_vectors(constants: Vec<Vec<f32>>, n: i64) -> Vec<Vec<f32>> {

        let mut vectors = vec![];

        // init vector0

        vectors.push(vec![constants[0][0], constants[0][1], 0.0, 0.0]);

        if n != 0 {
            for x in 0..n*2 {
                vectors.push(vec![constants[(x+1) as usize][0], constants[(x+1) as usize][1], vectors[x as usize][0], vectors[x as usize][1]])
            }
        }



        return vectors
    }

    fn f_of_t(t: f32) -> Complex<f32> {

        // square wave

        if t < 0.5 {
            return {Complex::new(1.0, 0.0)}
        } else {
            return {Complex::new(6.0, 0.0)}
        }

    }

    fn calculate_constant(n: i64) -> Vec<f32> {
        let mut integral = Complex::new(0.0, 0.0);
        let mut t = 0.0;

        for tx in 0..100 {

            let pi = std::f32::consts::PI;
            let exponent = -n as f32 * 2.0 * pi * t;
            let ft = Self::f_of_t(t);
            let e = Complex::new(exponent.cos(), exponent.sin());
            let result = ft * e * 0.01;

            integral += result;
            t += 0.01;
        }

        return vec![integral.re * data::SCALE_UP_CONST, integral.im * data::SCALE_UP_CONST]

    }

    fn get_constants(n: i64) -> Vec<Vec<f32>> {
        let mut constants = vec![];

        for x in 0..=n {
            constants.push(Self::calculate_constant(x));

            if x != 0 {
                constants.push(Self::calculate_constant(-x));
            }
        }



        return constants
    }

    pub fn new(n: i64) -> Self {
        Fourier_Serie {
            vectors: Self::init_vectors(Self::get_constants(n), n),
            constants: Self::get_constants(n),
            t: 0.0,
            speed_factor: 0,
            n: n,
            list_n: Self::init_n(n)
        }
    }

    fn init_n(n: i64) -> Vec<i64> {
        let mut list_n = vec![];

        for x in 0..=n {
            list_n.push(x);

            if x != 0 {
                list_n.push(-x);
            }
        }

        return list_n
    }

    fn coordinates_to_screen_coordinates(&mut self, n: i32) -> Vec<f32> {
        let mut new_vector = vec![];

        // x coordinate

        new_vector.push((self.update_pivots(n)[0] * 88.0) + 880.0);

        // y coordinate

        new_vector.push((self.update_pivots(n)[1] * -88.0) + 880.0);


        return new_vector
    }

    fn update_pivots(&mut self, n: i32) -> Vec<f32> {

        let mut v_re = 0.0;
        let mut v_im = 0.0;

        for x in 0..=n {

            v_re += self.vectors[x as usize][0];
            v_im += self.vectors[x as usize][1];

        }

        let mut vn = vec![v_re, v_im];

        return vn

    }

    fn update_vectors(&mut self) {
        // rotate points

        let pi = std::f32::consts::PI;

        for x in 0..self.vectors.len() {

            let c = Complex::new(self.constants[x][0], self.constants[x][1]);
            let exponent = -(self.list_n[x] as i64)  as f32 * 2.0 * pi * self.t;
            let e = Complex::new(exponent.cos(), exponent.sin());
            let result = c * e;

            self.vectors[x as usize][0] = result.re as f32;
            self.vectors[x as usize][1] = result.im as f32;
        }


    }

    fn draw_axis(&mut self, context: &mut Context, canvas: &mut graphics::Canvas) -> Option<i32> {

        // draw real and imaginary axis

        let points_of_real_axis = vec![vec2(0.0, 0.0), vec2(1760.0, 0.0)];  // Line from (10, 50) to (100, 50)
        let axis = graphics::Mesh::new_line(context, &points_of_real_axis, 10.0, Color::BLUE).ok()?;

        canvas.draw(&axis, Vec2::new(0.0, 880.0));

        let points_of_img_axis = vec![vec2(0.0, 0.0), vec2(0.0, 1760.0)];  // Line from (10, 50) to (100, 50)
        let axis = graphics::Mesh::new_line(context, &points_of_img_axis, 10.0, Color::BLUE).ok()?;

        canvas.draw(&axis, Vec2::new(880.0, 0.0));

        return Some(0)
    }

    fn draw_vectors(&mut self, context: &mut Context, canvas: &mut graphics::Canvas) -> Option<i32> {

        // draw points/vectors

        self.draw_point(context, canvas);


        return Some(0);

    }

    fn determine_color(&mut self, x: usize) -> Color {
        if x == self.constants.len() - 1 {
            return Color::RED
        } else {
            return Color::BLUE
        }
    }

    fn draw_point(&mut self, context: &mut Context, canvas: &mut graphics::Canvas) -> Option<i32> {


        for x in 0..self.constants.len() {

            let circle = graphics::Mesh::new_circle(
                context,
                graphics::DrawMode::fill(),
                vec2(0., 0.),
                10.0,
                2.0,
                self.determine_color(x),
            ).ok()?;

            // for n and -n

            canvas.draw(&circle, Vec2::new(self.coordinates_to_screen_coordinates(x as i32)[0], self.coordinates_to_screen_coordinates(x as i32)[1]));

            // draw vector to point

            // let points_of_real_axis = vec![vec2(self.vectors[x as usize][2] * 88.0, self.vectors[x as usize][3] * -88.0), vec2(self.vectors[x as usize][0] * 88.0, self.vectors[x as usize][1] * -88.0)];
            // let axis = graphics::Mesh::new_line(context, &points_of_real_axis, 10.0, Color::BLUE).ok()?;
            //
            // canvas.draw(&axis, Vec2::new(self.vectors[x as usize][2] * 88.0 + 880.0, self.vectors[x as usize][3] * -88.0 + 880.0));

        }

        return Some(0);

    }

    fn connect_vector(&mut self, context: &mut Context, canvas: &mut graphics::Canvas) -> Mesh {

        let origin_of_vector = vec![vec2(self.vectors[0][2], self.vectors[0][3]), vec2(self.vectors[0][0], self.vectors[0][1])];
        let vector = graphics::Mesh::new_line(context, &origin_of_vector, 10.0, Color::BLUE);

        return vector.expect("")
    }
}

impl event::EventHandler<ggez::GameError> for Fourier_Serie {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(data::DESIRED_FPS) {

            self.update_vectors();

            self.speed_factor += 1;


            self.t += 0.01;


            println!("{:?}", self.t);

            if self.t > 10.0 {
                process::exit(1);

            }

        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([1.0, 1.0, 1.0, 1.0]));

        self.draw_axis(ctx, &mut canvas);

        self.draw_point(ctx, &mut canvas);

        canvas.finish(ctx)?;

        ggez::timer::yield_now();
        Ok(())
    }
}

fn get_n() -> i64 {
    // get N

    let mut n_str = String::new();

    println!("How precise should be the series (N) : ");

    io::stdin()
        .read_line(&mut n_str)
        .expect("Failed to read line");

    let n = n_str.trim().parse::<i64>().unwrap();

    return n
}

fn main() -> GameResult {

    let n = get_n();

    let (ctx, events_loop) = ggez::ContextBuilder::new("Fourier Series", "ObvMamut")
        .window_setup(ggez::conf::WindowSetup::default().title("Fourier Series"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(data::SCREEN_SIZE.0, data::SCREEN_SIZE.1))
        .build()?;


    let state = Fourier_Serie::new(n);
    event::run(ctx, events_loop, state)
}