#!/bin/bash

prog=target/thumbv7em-none-eabihf/release/clock
openocd -f flash.cfg -c "flash_elf $prog"
