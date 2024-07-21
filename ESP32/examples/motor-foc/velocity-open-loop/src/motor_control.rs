use core::f32::consts;

use esp_hal::time;

const PI_2:f32 = 2f32 * consts::PI;
const VOLTAGE_POWER_SUPPLY:f32 =12.6f32;
const ZERO_ELECTRIC_ANGLE:f32 = 0f32;
static mut SHAFT_ANGLE:f32 = 0f32;
static mut OPEN_LOOP_TIMESTAMP:u64 = 0;

//开环速度函数
pub fn velocity_openloop(target_velocity:f32) -> (f32, f32, f32) {
    let now_us = time::current_time().ticks();  // 获取从开启芯片以来的微秒数
    
    //计算当前每个Loop的运行时间间隔
    let mut peroid: f32 = (now_us - unsafe { OPEN_LOOP_TIMESTAMP }) as f32 * 1e-6f32;

    //由于 micros() 函数返回的时间戳会在大约 70 分钟之后重新开始计数，在由70分钟跳变到0时，TS会出现异常，因此需要进行修正。如果时间间隔小于等于零或大于 0.5 秒，则将其设置为一个较小的默认值，即 1e-3f
    if peroid <= 0f32 || peroid > 0.5f32 { peroid = 1e-3f32; }
    
    // 通过乘以时间间隔和目标速度来计算需要转动的机械角度，存储在 shaft_angle 变量中。在此之前，还需要对轴角度进行归一化，以确保其值在 0 到 2π 之间。
    unsafe { SHAFT_ANGLE = normalize_angle(SHAFT_ANGLE + target_velocity * peroid) };
    //以目标速度为 10 rad/s 为例，如果时间间隔是 1 秒，则在每个循环中需要增加 10 * 1 = 10 弧度的角度变化量，才能使电机转动到目标速度。
    //如果时间间隔是 0.1 秒，那么在每个循环中需要增加的角度变化量就是 10 * 0.1 = 1 弧度，才能实现相同的目标速度。因此，电机轴的转动角度取决于目标速度和时间间隔的乘积。

    // 使用早前设置的voltage_power_supply的1/3作为Uq值，这个值会直接影响输出力矩
    // 最大只能设置为Uq = voltage_power_supply/2，否则ua,ub,uc会超出供电电压限幅
    let u_q = VOLTAGE_POWER_SUPPLY / 3f32;
    
    let duty = set_phase_voltage(u_q, 0.0, electrical_angle(unsafe { SHAFT_ANGLE }, 7));
    
    unsafe { OPEN_LOOP_TIMESTAMP = now_us };  //用于计算下一个时间间隔

    return duty;
}

pub fn set_phase_voltage(uq:f32, ud:f32, angle_el:f32) -> (f32, f32, f32) {
    let angle_el = normalize_angle(angle_el + ZERO_ELECTRIC_ANGLE);
    // 帕克逆变换
    let u_alpha = -uq * libm::sinf(angle_el); 
    let u_beta  =  uq * libm::cosf(angle_el); 
    // 克拉克逆变换
    let u_a = (u_alpha + VOLTAGE_POWER_SUPPLY) * 0.5;
    let sqrt_3 = libm::sqrtf(3f32);
    let u_b = ((sqrt_3 * u_beta - u_alpha) + VOLTAGE_POWER_SUPPLY) * 0.5;
    let u_c = ((-u_alpha - sqrt_3 * u_beta) + VOLTAGE_POWER_SUPPLY) * 0.5;
    return set_pwm(u_a, u_b, u_c);
}

// 设置PWM到控制器输出
fn set_pwm(u_a: f32, u_b: f32, u_c:f32) -> (f32, f32, f32) {
    // 计算占空比
    // 限制占空比从0到1
    let dc_a = constrain(u_a / VOLTAGE_POWER_SUPPLY, 0.0f32 , 1.0f32 );
    let dc_b = constrain(u_b / VOLTAGE_POWER_SUPPLY, 0.0f32 , 1.0f32 );
    let dc_c = constrain(u_c / VOLTAGE_POWER_SUPPLY, 0.0f32 , 1.0f32 );

    return (dc_a, dc_b, dc_c);
}

// 电角度求解
fn electrical_angle(shaft_angle: f32, pole_pairs: i32) -> f32 {
    return shaft_angle * pole_pairs as f32;
}

// 归一化角度到 [0,2PI]
fn normalize_angle(angle: f32) -> f32 {
    let a = angle % PI_2;   //取余运算可以用于归一化，列出特殊值例子算便知
    return if a >= 0f32 { a } else { a + PI_2 };  
}

fn constrain(amt:f32, low:f32, high:f32) -> f32{
    if amt < low {
        low
    }
    else if amt > high {
        high
    }
    else {
        amt
    }
}