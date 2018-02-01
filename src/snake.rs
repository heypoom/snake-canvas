extern crate stdweb;

use stdweb::web;

pub struct Point(u32, u32);

pub struct Game {
  px: u32,
  py: u32,

  gs: u32,
  tc: u32,

  ax: u32,
  ay: u32,

  xv: u32,
  yv: u32,

  trail: Vec<Point>,
  tail: u32
}

impl Game {
  pub fn new() -> Game {
    Game {
      px: 10,
      py: 10,

      gs: 20,
      tc: 20,

      ax: 15,
      ay: 15,

      xv: 0,
      yv: 0,

      trail: vec![],
      tail: 5
    }
  }

  pub fn update(&mut self) -> () {
    let window = web::window();
    let document = web::document();
    let canvas = document.query_selector("canvas").unwrap();

    let width = js! {return @{canvas}.width};
    let height = js! {return @{canvas}.height};

    let ctx = js! {
      return @{canvas}.getContext("2d");
    };

    window.request_animation_frame(move |raf| {
      self.px += self.xv;
      self.py += self.yv;

      if self.px < 0 {
        self.px = self.tc - 1;
      }

      if self.px > self.tc - 1 {
        self.px = 0;
      }

      if self.py < 0 {
        self.py = self.tc - 1
      }

      if self.py > self.tc - 1 {
        self.py = 0
      }

      js! {
        @{ctx}.fillStyle = "white";
        @{ctx}.fillRect(0, 0, @{width}, @{height});

        @{ctx}.fillStyle = "darkorange";
        @{ctx}.filter = "drop-shadow(red 0px 0px 12px)";
      }

      // Draw the snake's trail
      for block in self.trail {
        js! {
          ctx.fillRect(
            @{block.0 * self.gs},
            @{block.1 * self.gs},
            @{self.gs - 2},
            @{self.gs - 2}
          );
        }

        if block.0 == self.px && block.1 == self.py {
          self.tail = 5;
        }
      }

      self.trail.push(Point(self.px, self.py));

      while self.trail.len() as u32 > self.tail {
        self.trail.remove(0);
      }

      // TODO: Add RNG
      let rand_x = 3;
      let rand_y = 3;

      if self.ax == self.px && self.ay == self.py {
        self.tail += 1;
        self.ax = rand_x * self.tc;
        self.ay = rand_y * self.tc;
      }

      js! {
        ctx = @{ctx};

        ctx.fillStyle = "red";
        ctx.shadowColor = "rgba(0, 0, 0, 0.18)";
        ctx.shadowBlur = 4;
        ctx.shadowOffsetX = 1;
        ctx.shadowOffsetY = 1;
        ctx.filter = "drop-shadow(red 0px 0px 12px)";

        ctx.fillRect(
          @{self.ax * self.gs},
          @{self.ay * self.gs},
          @{self.gs - 2},
          @{self.gs - 2},
        );
      };

      self.update();
    });
  }
}

