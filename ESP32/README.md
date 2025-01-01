# ESP32 嵌入式开发

## 环境配置

### 针对 RISC-V 架构

1. 需要安装 nightly 工具链和 rust-src 组件

   ```cmd
   rustup toolchain install nightly --component rust-src --target riscv32imc-unknown-none-elf
   ```

2. 设置编译目标

   ```cmd
   rustup target add riscv32imc-unknown-none-elf # 针对 ESP32-C2 和 ESP32-C3
   rustup target add riscv32imac-unknown-none-elf # 针对 ESP32-C6 和 ESP32-H2
   ```

### 针对 Xtensa 架构

上述方法配置的环境不支持 Xtensa 架构，如果需要支持 Xtensa 架构，可以通过 espup 工具。

1. 安装 espup 工具

   ```cmd
   cargo install espup
   ```

2. 通过 espup 安装其它工具链

   ```cmd
   espup install
   ```

   > espup 安装了如下内容
   >
   > 1. 乐鑫 rust 分支，支持乐鑫目标；
   > 2. nightly 分支，支持 RISC-V 目标；
   > 3. GCC 工具链，用于链接最终的二进制文件；

## 项目预览

| 是否完成 | 名称                                    | 简述                                      | 支持芯片 | 是否需要 IDF |
| -------- | --------------------------------------- | ----------------------------------------- | -------- | ------------ |
| ✅       | [hello-world](./hello-world/)           | rust 开发 esp32 最小工程                  | ESP32    | ×            |
| ✅       | [blinky](./blinky/)                     | 通过 GPIO 控制 LED 闪烁                   | ESP32-C3 | ×            |
| ✅       | [button](./button/)                     | 以轮询的方式读取 Button 输入控制 LED 亮灭 | ESP32-C3 | ×            |
| ✅       | [button-interrupt](./button-interrupt/) | 以中断的方式读取 Button 输入控制 LED 亮灭 | ESP32-C3 | ×            |
| ✅       | [ledc](./ledc/)                         | ESP32 LEDC 功能                           | ESP32    | ×            |
| ✅       | [mcpwm](./mcpwm/)                       | ESP32 MCPWM 功能                          | ESP32    | ×            |
|          | uart                                    | ESP32 串口通信                            | ESP32-C3 | ×            |
|          | spi                                     | ESP32 SPI 通信                            | ESP32-C3 | ×            |
