use crate::draw::draw_block;
use piston_window::{types::Color, Context, G2d};
use std::collections::LinkedList;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction
{
	Up,
	Down,
	Left,
	Right
}

impl Direction
{
	pub fn opposite(&self) -> Direction
	{
		match *self
		{
			Direction::Down => Direction::Up,
			Direction::Up => Direction::Down,
			Direction::Left => Direction::Right,
			Direction::Right => Direction::Left
		}
	}
}

#[derive(Debug, Clone)]
struct Block
{
	x: i32,
	y: i32
}

pub struct Snake
{
	direction: Direction,
	body:      LinkedList<Block>,
	tail:      Option<Block>
}

impl Snake
{
	pub fn new(x: i32, y: i32) -> Snake
	{
		let mut body: LinkedList<Block> = LinkedList::new();

		body.push_back(Block { x: x + 2, y });
		body.push_back(Block { x: x + 1, y });
		body.push_back(Block { x, y });

		Snake {
			direction: Direction::Right,
			body:      body,
			tail:      None
		}
	}

	pub fn draw(&self, context: &Context, graphics: &mut G2d)
	{
		for block in &self.body
		{
			draw_block(SNAKE_COLOR, block.x, block.y, context, graphics)
		}
	}

	pub fn head_position(&self) -> (i32, i32)
	{
		let head_block = self.body.front().unwrap();

		(head_block.x, head_block.y)
	}

	pub fn move_forward(&mut self, direction: Option<Direction>)
	{
		match direction
		{
			Some(direction) => self.direction = direction,
			None => ()
		}

		let (last_x, last_y): (i32, i32) = self.head_position();

		let new_block = match self.direction
		{
			Direction::Up =>
			{
				Block {
					x: last_x,
					y: last_y - 1
				}
			}
			Direction::Down =>
			{
				Block {
					x: last_x,
					y: last_y + 1
				}
			}
			Direction::Left =>
			{
				Block {
					x: last_x - 1,
					y: last_y
				}
			}
			Direction::Right =>
			{
				Block {
					x: last_x + 1,
					y: last_y
				}
			}
		};

		self.body.push_front(new_block);
		let removed_body = self.body.pop_back().unwrap();
		self.tail = Some(removed_body);
	}

	pub fn head_direction(&self) -> Direction { self.direction }

	pub fn next_head(&self, direction: Option<Direction>) -> (i32, i32)
	{
		let (head_x, head_y): (i32, i32) = self.head_position();

		let mut moving_direction = self.direction;

		match direction
		{
			Some(direction) => moving_direction = direction,
			None => ()
		}

		match moving_direction
		{
			Direction::Up => (head_x, head_y - 1),
			Direction::Down => (head_x, head_y + 1),
			Direction::Left => (head_x - 1, head_y),
			Direction::Right => (head_x + 1, head_y)
		}
	}

	pub fn increase_body_length(&mut self)
	{
		let new_block = self.tail.clone().unwrap();

		self.body.push_back(new_block)
	}

	pub fn overlap_body(&self, x: i32, y: i32) -> bool
	{
		let mut counted_body_blocks = 0;

		let tail_index = &self.body.len() - 1;

		for block in &self.body
		{
			if x == block.x && y == block.y
			{
				return true;
			}

			counted_body_blocks += 1;

			if counted_body_blocks == tail_index
			{
				break;
			}
		}

		return false;
	}
}
