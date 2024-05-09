SRC = ./target/armv5te-unknown-linux-musleabi/release/soccer-kicker
BRICK_NAME = robot@ev3dev.local
NAME = soccer-kicker

build:
# Copy the source code to the brick
	cargo build --release
	scp $(SRC) $(BRICK_NAME):/home/robot/$(NAME)

# set default target
.DEFAULT_GOAL := build