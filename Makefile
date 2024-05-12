SRC = ./target/armv5te-unknown-linux-musleabi/release/ferret
BRICK_NAME = robot@ev3dev.local
NAME = ferret

build:
# Copy the source code to the brick, make folder if necessary
	cargo build --release
	ssh $(BRICK_NAME) "mkdir -p /home/robot/$(NAME)"
	scp $(SRC) $(BRICK_NAME):/home/robot/$(NAME)/run

# set default target
.DEFAULT_GOAL := build