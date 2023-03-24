use std::f32::consts::PI;

/// 为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同
trait LightStandTime {
    fn stand_time_in_seconds(&self) -> u8;
}

#[derive(Debug)]
enum TrafficLights {
    Red,
    Yellow,
    Green,
}

impl LightStandTime for TrafficLights {
    fn stand_time_in_seconds(&self) -> u8 {
        return match self {
            TrafficLights::Red => { 60u8 }
            TrafficLights::Yellow => { 3u8 }
            TrafficLights::Green => { 30u8 }
        };
    }
}

#[test]
fn test_traffic_light_time() {
    let all_lights = [TrafficLights::Red, TrafficLights::Yellow, TrafficLights::Green];
    for item in all_lights {
        println!("{:?} Stand Time: {}", item, item.stand_time_in_seconds());
    }
}


/// 实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None

fn sum(array: &[u32]) -> Option<u32> {
    let mut result = 0u32;
    for item in array {
        // 如果剩余可累加量 < 下一个累加值，则必定溢出
        if u32::MAX - result < *item {
            return None;
        }
        result += *item;
    }
    Some(result)
}

fn call_sum(array: &[u32]) -> Option<u32> {
    let r = sum(array);
    match r {
        None => {
            eprintln!("求和溢出")
        }
        Some(v) => {
            println!("求和结果:{}", v)
        }
    }
    r
}

#[test]
fn test_sum() {
    let to_sum_normal = [1u32, 2u32, 4u32];
    assert_eq!(call_sum(&to_sum_normal), Some(7u32));
    let to_sum_overflow = [1u32, 2u32, 4u32, u32::MAX];
    assert_eq!(call_sum(&to_sum_overflow), None)
}

/// 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束
trait AreaCalculable {
    fn calculate_area(&self) -> f32;
}

// 计算并打印面积
fn calculate_and_print_area<T: AreaCalculable>(shape: T) {
    let area = shape.calculate_area();
    println!("面积:{}", area);
}

struct Circle {
    radius: f32,
}

impl AreaCalculable for Circle {
    fn calculate_area(&self) -> f32 {
        PI * self.radius * self.radius
    }
}

#[test]
fn test_area_calculate() {
    calculate_and_print_area(Circle { radius: 2.0f32 })
}

fn main() {
    println!("Hello, world!");
}
