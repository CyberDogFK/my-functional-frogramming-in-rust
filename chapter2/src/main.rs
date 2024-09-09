extern crate floating_duration;

use std::{env, io, thread, time};
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use floating_duration::TimeAsFloat;

pub fn run_simulation() {
    // 1. Store location, velocity, and acceleration state
    let mut location: f64 = 0.0; //meters
    let mut velocity: f64 = 0.0; // meters per second
    let mut acceleration: f64 = 0.0; // meters per second squared
    
    // 2. Store motor input voltage
    let mut up_input_voltage: f64 = 0.0;
    let mut down_input_voltage: f64 = 0.0;
    
    // 3. Store input building description and floor requests
    let mut floor_count: u64 = 0;
    let mut floor_height: f64 = 0.0; // meters
    let mut floor_requests: Vec<u64> = Vec::new();
    
    // 4. Parse input and store as building description and floor requests
    let buffer = match env::args().nth(1) {
        Some(ref fp) if *fp == "-".to_string() => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)
                .expect("read_to_string failed");
            buffer
        },
        None => {
            let fp = "test1.txt";
            let mut buffer = String::new();
            File::open(fp)
                .expect("File::open failed")
                .read_to_string(&mut buffer)
                .expect("read_to_string failed");
            buffer
        },
        Some(fp) => {
            let mut buffer = String::new();
            File::open(fp)
                .expect("File::open failed")
                .read_to_string(&mut buffer)
                .expect("read to string failed");
            buffer
        }
    };
    
    for (li, l) in buffer.lines().enumerate() {
        if li == 0 {
            floor_count = l.parse::<u64>().unwrap();
        } else if li == 1 {
            floor_height = l.parse::<f64>().unwrap();
        } else {
            floor_requests.push(l.parse::<u64>().unwrap());
        }
    }
    
    // 5. Loop while there are remaining floor requests
    let mut prev_loop_time = Instant::now();
    while  floor_requests.len() > 0 {
        // 5.1. Update location, velocity, and acceleration
        let now = Instant::now();
        let dt = now.duration_since(prev_loop_time)
            .as_fractional_secs();
        prev_loop_time = now;
        
        location = location + velocity * dt;
        velocity = velocity + acceleration * dt;
        acceleration = {
            let F = (up_input_voltage - down_input_voltage) * 8.0;
            let m = 1200000.0;
            // -9.8 is and approximation of acceleration due to gravity
            -9.8 + F/m
        };

        // 5.2. If next floor request in queue is satisfied, then remove from queue
        let next_floor = floor_requests[0];
        if (location - (next_floor as f64) * floor_height).abs() < 0.01 
            && velocity.abs() < 0.01 {
            velocity = 0.0;
            floor_requests.remove(0);
        }
        
        // 5.3. Adjust motor control to process next floor request
        
        // it will take t seconds to decelerate from velocity v as -1 m/s^2
        let t = velocity.abs() / 1.0;
        
        // during which time, the carriage will travel d=t * v/2 meters
        // at an average velocity of v/2 before stopping
        let d = t * (velocity / 2.0);
        
        // l = distance to next floor
        let l = (location - (next_floor as f64) * floor_height).abs();
        
        let target_acceleration = {
            // are we going up?
            let going_up = location < (next_floor as f64) * floor_height;
            
            // Do not exceed maximum velocity
            if velocity.abs() >= 5.0 {
                // if we are going up and actually going up
                // or we are going down and actually going down
                if going_up && velocity > 0.0 
                    || !going_up && velocity < 0.0 {
                    0.0
                    // decelerate if going in wrong direction
                } else if going_up {
                    1.0
                } else {
                    -1.0
                }
                // if within comfortable deceleration range and moving in right direction, decelerate
            } else if l < d && going_up == (velocity > 0.0) {
                if going_up {
                    -1.0
                } else {
                    1.0
                }
                // else if not at peak velocity, accelerate
            } else {
                if going_up {
                    1.0
                } else {
                    -1.0
                }
            }
        };
        
        let gravity_adjusted_acceleration = target_acceleration + 9.8;
        let target_force = gravity_adjusted_acceleration * 1200000.0;
        let target_voltage = target_force / 8.0;
        if target_voltage > 0.0 {
            up_input_voltage = target_voltage;
            down_input_voltage = 0.0;
        } else {
            up_input_voltage = 0.0;
            down_input_voltage = target_voltage.abs();
        }
        
        // 5.4. Print realtime statistics 
        
        thread::sleep(time::Duration::from_millis(10));
    } 
    
    // 6. Print summary
    println!("summary");
}

fn main() {
    run_simulation();
}
