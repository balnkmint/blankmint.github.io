use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use js_sys::Math;

// 导入console.log宏
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// 游戏常量
const GRID_SIZE: u32 = 20;
const CANVAS_WIDTH: u32 = 600;
const CANVAS_HEIGHT: u32 = 400;
const GRID_WIDTH: u32 = CANVAS_WIDTH / GRID_SIZE;
const GRID_HEIGHT: u32 = CANVAS_HEIGHT / GRID_SIZE;

// 方向枚举
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// 位置结构体
#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn new(x: u32, y: u32) -> Self {
        Position { x, y }
    }
}

// 游戏状态
#[wasm_bindgen]
pub struct Game {
    snake: Vec<Position>,
    direction: Direction,
    food: Position,
    score: u32,
    game_over: bool,
    paused: bool,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        console_log!("Creating new snake game");
        
        let mut game = Game {
            snake: vec![Position::new(10, 10)],
            direction: Direction::Right,
            food: Position::new(15, 15),
            score: 0,
            game_over: false,
            paused: false,
        };
        
        game.generate_food();
        game
    }

    pub fn tick(&mut self) {
        if self.game_over || self.paused {
            return;
        }

        let head = self.snake[0];
        let new_head = match self.direction {
            Direction::Up => Position::new(head.x, if head.y == 0 { GRID_HEIGHT - 1 } else { head.y - 1 }),
            Direction::Down => Position::new(head.x, (head.y + 1) % GRID_HEIGHT),
            Direction::Left => Position::new(if head.x == 0 { GRID_WIDTH - 1 } else { head.x - 1 }, head.y),
            Direction::Right => Position::new((head.x + 1) % GRID_WIDTH, head.y),
        };

        // 检查是否撞到自己
        if self.snake.contains(&new_head) {
            self.game_over = true;
            console_log!("Game Over! Final score: {}", self.score);
            return;
        }

        self.snake.insert(0, new_head);

        // 检查是否吃到食物
        if new_head == self.food {
            self.score += 10;
            self.generate_food();
            console_log!("Score: {}", self.score);
        } else {
            self.snake.pop();
        }
    }

    pub fn set_direction_from_key(&mut self, key: &str) {
        if self.game_over || self.paused {
            return;
        }

        let new_direction = match key {
            "ArrowUp" | "KeyW" => Direction::Up,
            "ArrowDown" | "KeyS" => Direction::Down,
            "ArrowLeft" | "KeyA" => Direction::Left,
            "ArrowRight" | "KeyD" => Direction::Right,
            _ => return,
        };

        // 防止蛇反向移动
        let opposite_direction = match self.direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };

        if new_direction != opposite_direction {
            self.direction = new_direction;
        }
    }

    pub fn toggle_pause(&mut self) {
        if !self.game_over {
            self.paused = !self.paused;
            console_log!("Game {}", if self.paused { "paused" } else { "resumed" });
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        // 清空画布
        ctx.set_fill_style(&"#000000".into());
        ctx.fill_rect(0.0, 0.0, CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);

        // 绘制蛇身
        ctx.set_fill_style(&"#00ff00".into());
        for (i, segment) in self.snake.iter().enumerate() {
            if i == 0 {
                // 绘制猫头（蛇头）
                self.draw_cat_head(ctx, segment);
            } else {
                // 绘制蛇身
                ctx.set_fill_style(&"#00ff00".into());
                ctx.fill_rect(
                    (segment.x * GRID_SIZE) as f64,
                    (segment.y * GRID_SIZE) as f64,
                    GRID_SIZE as f64,
                    GRID_SIZE as f64,
                );
            }
        }

        // 绘制粑粑（食物）
        self.draw_poop(ctx, &self.food);

        // 如果游戏结束，显示游戏结束文本
        if self.game_over {
            ctx.set_fill_style(&"#ffffff".into());
            ctx.set_font("30px Arial");
            ctx.fill_text("Game Over!", 200.0, 200.0).unwrap();
            ctx.fill_text(&format!("Score: {}", self.score), 220.0, 240.0).unwrap();
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn restart(&mut self) {
        self.snake = vec![Position::new(10, 10)];
        self.direction = Direction::Right;
        self.score = 0;
        self.game_over = false;
        self.paused = false;
        self.generate_food();
        console_log!("Game restarted");
    }

    fn draw_cat_head(&self, ctx: &CanvasRenderingContext2d, position: &Position) {
        let x = (position.x * GRID_SIZE) as f64;
        let y = (position.y * GRID_SIZE) as f64;
        let size = GRID_SIZE as f64;

        // 绘制白猫头背景
        ctx.set_fill_style(&"#ffffff".into());
        ctx.fill_rect(x, y, size, size);

        // 绘制猫耳朵外轮廓（三角形形状）
        ctx.set_fill_style(&"#f8f8f8".into());
        // 左耳三角形
        ctx.fill_rect(x + 2.0, y + 0.0, 6.0, 1.0);
        ctx.fill_rect(x + 3.0, y + 1.0, 4.0, 1.0);
        ctx.fill_rect(x + 4.0, y + 2.0, 2.0, 1.0);
        ctx.fill_rect(x + 4.5, y + 3.0, 1.0, 1.0);
        // 右耳三角形
        ctx.fill_rect(x + size - 8.0, y + 0.0, 6.0, 1.0);
        ctx.fill_rect(x + size - 7.0, y + 1.0, 4.0, 1.0);
        ctx.fill_rect(x + size - 6.0, y + 2.0, 2.0, 1.0);
        ctx.fill_rect(x + size - 5.5, y + 3.0, 1.0, 1.0);

        // 绘制耳朵内部（粉色三角形）
        ctx.set_fill_style(&"#ffb6c1".into());
        // 左耳内
        ctx.fill_rect(x + 3.5, y + 1.0, 3.0, 1.0);
        ctx.fill_rect(x + 4.0, y + 2.0, 2.0, 1.0);
        ctx.fill_rect(x + 4.5, y + 3.0, 1.0, 1.0);
        // 右耳内
        ctx.fill_rect(x + size - 6.5, y + 1.0, 3.0, 1.0);
        ctx.fill_rect(x + size - 6.0, y + 2.0, 2.0, 1.0);
        ctx.fill_rect(x + size - 5.5, y + 3.0, 1.0, 1.0);

        // 绘制头顶的一小撮黑毛
        ctx.set_fill_style(&"#333333".into());
        ctx.fill_rect(x + size/2.0 - 1.5, y + 1.0, 3.0, 1.0);
        ctx.fill_rect(x + size/2.0 - 1.0, y + 2.0, 2.0, 1.0);
        ctx.fill_rect(x + size/2.0 - 0.5, y + 3.0, 1.0, 1.0);

        // 绘制大眼睛（金黄色椭圆形）
        ctx.set_fill_style(&"#daa520".into());
        // 左眼
        ctx.fill_rect(x + 3.0, y + 6.0, 5.0, 1.0);
        ctx.fill_rect(x + 2.5, y + 7.0, 6.0, 1.0);
        ctx.fill_rect(x + 3.0, y + 8.0, 5.0, 1.0);
        ctx.fill_rect(x + 3.5, y + 9.0, 4.0, 1.0);
        // 右眼
        ctx.fill_rect(x + size - 8.0, y + 6.0, 5.0, 1.0);
        ctx.fill_rect(x + size - 8.5, y + 7.0, 6.0, 1.0);
        ctx.fill_rect(x + size - 8.0, y + 8.0, 5.0, 1.0);
        ctx.fill_rect(x + size - 7.5, y + 9.0, 4.0, 1.0);

        // 绘制瞳孔（竖直椭圆）
        ctx.set_fill_style(&"#000000".into());
        ctx.fill_rect(x + 5.0, y + 6.5, 1.0, 3.0); // 左瞳孔
        ctx.fill_rect(x + size - 6.0, y + 6.5, 1.0, 3.0); // 右瞳孔

        // 绘制眼睛高光
        ctx.set_fill_style(&"#ffffff".into());
        ctx.fill_rect(x + 4.5, y + 7.0, 1.0, 1.0); // 左眼高光
        ctx.fill_rect(x + size - 6.5, y + 7.0, 1.0, 1.0); // 右眼高光

        // 绘制鼻子（粉红色小三角）
        ctx.set_fill_style(&"#ff69b4".into());
        ctx.fill_rect(x + size/2.0 - 1.0, y + 11.0, 2.0, 1.0);
        ctx.fill_rect(x + size/2.0 - 0.5, y + 12.0, 1.0, 1.0);

        // 绘制嘴巴（W形状）
        ctx.set_fill_style(&"#000000".into());
        ctx.fill_rect(x + size/2.0 - 2.5, y + 13.0, 1.0, 1.0);
        ctx.fill_rect(x + size/2.0 - 1.0, y + 14.0, 1.0, 1.0);
        ctx.fill_rect(x + size/2.0, y + 14.0, 1.0, 1.0);
        ctx.fill_rect(x + size/2.0 + 1.5, y + 13.0, 1.0, 1.0);

        // 绘制胡须（更长更明显）
        ctx.set_fill_style(&"#666666".into());
        // 左侧胡须
        ctx.fill_rect(x + 0.0, y + 10.0, 3.0, 0.5);
        ctx.fill_rect(x + 0.0, y + 11.5, 3.0, 0.5);
        ctx.fill_rect(x + 0.0, y + 13.0, 3.0, 0.5);
        // 右侧胡须
        ctx.fill_rect(x + size - 3.0, y + 10.0, 3.0, 0.5);
        ctx.fill_rect(x + size - 3.0, y + 11.5, 3.0, 0.5);
        ctx.fill_rect(x + size - 3.0, y + 13.0, 3.0, 0.5);
    }

    fn draw_poop(&self, ctx: &CanvasRenderingContext2d, position: &Position) {
        let x = (position.x * GRID_SIZE) as f64;
        let y = (position.y * GRID_SIZE) as f64;
        let size = GRID_SIZE as f64;

        // 绘制粑粑底部（最大的一坨）
        ctx.set_fill_style(&"#8b4513".into());
        ctx.fill_rect(x + 2.0, y + 10.0, size - 4.0, 6.0);
        ctx.fill_rect(x + 1.0, y + 12.0, size - 2.0, 4.0);

        // 绘制粑粑中部（中等大小）
        ctx.set_fill_style(&"#a0522d".into());
        ctx.fill_rect(x + 3.0, y + 6.0, size - 6.0, 6.0);
        ctx.fill_rect(x + 2.0, y + 8.0, size - 4.0, 4.0);

        // 绘制粑粑顶部（最小的一坨）
        ctx.set_fill_style(&"#cd853f".into());
        ctx.fill_rect(x + 4.0, y + 2.0, size - 8.0, 6.0);
        ctx.fill_rect(x + 3.0, y + 4.0, size - 6.0, 4.0);

        // 绘制粑粑的阴影和立体感
        ctx.set_fill_style(&"#654321".into());
        // 底部阴影
        ctx.fill_rect(x + size - 3.0, y + 12.0, 2.0, 4.0);
        ctx.fill_rect(x + 2.0, y + size - 2.0, size - 4.0, 2.0);
        // 中部阴影
        ctx.fill_rect(x + size - 4.0, y + 8.0, 2.0, 4.0);
        // 顶部阴影
        ctx.fill_rect(x + size - 5.0, y + 4.0, 2.0, 4.0);

        // 绘制粑粑顶部的小装饰（热气效果）
        ctx.set_fill_style(&"#ddd".into());
        ctx.fill_rect(x + size/2.0 - 1.0, y + 0.0, 1.0, 2.0);
        ctx.fill_rect(x + size/2.0 + 1.0, y + 1.0, 1.0, 2.0);
        ctx.fill_rect(x + size/2.0 - 2.0, y + 1.0, 1.0, 1.0);

        // 绘制粑粑表面的纹理
        ctx.set_fill_style(&"#5d4037".into());
        ctx.fill_rect(x + 4.0, y + 5.0, 1.0, 1.0);
        ctx.fill_rect(x + 6.0, y + 9.0, 1.0, 1.0);
        ctx.fill_rect(x + 8.0, y + 13.0, 1.0, 1.0);
    }

    fn generate_food(&mut self) {
        loop {
            let x = (Math::random() * GRID_WIDTH as f64) as u32;
            let y = (Math::random() * GRID_HEIGHT as f64) as u32;
            let new_food = Position::new(x, y);
            
            if !self.snake.contains(&new_food) {
                self.food = new_food;
                break;
            }
        }
    }
}
