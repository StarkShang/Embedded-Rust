openocd -f openocd.cfg -c "program target/thumbv7m-none-eabi/debug/blinky preverify verify reset exit 0x08000000"