/// 为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同
trait LightStandTime {
    fn stand_time_in_seconds(&self) -> u32;
}

#[derive(Debug)]
enum Traffic_Lights {
    Red,
    Yellow,
    Green,
}

impl LightStandTime for Traffic_Lights {
    fn stand_time_in_seconds(&self) -> u32 {
        return match self {
            Traffic_Lights::Red => { 30u32 }
            Traffic_Lights::Yellow => { 3u32 }
            Traffic_Lights::Green => { 30u32 }
        };
    }
}

#[test]
fn test_traffic_light_time() {
    let all_lights = [Traffic_Lights::Red, Traffic_Lights::Yellow, Traffic_Lights::Green];
    for item in all_lights {
        println!("{:?} Stand Time: {}", item, item.stand_time_in_seconds());
    }
}

const u32Max: u32 = 0xFFFFFFFFu32;

/// 实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None
fn sum(array: &[u32]) -> Option<u32> {
    let mut result = 0u32;
    for item in array {
        // 如果剩余可累加量,< 下一个累加值，则必定溢出
        if u32Max - result < *item {
            return None;
        }
        result += *item;
    }
    Some(result)
}

fn call_sum(array: &[u32]) {
    match sum(array) {
        None => {
            eprintln!("求和溢出")
        }
        Some(v) => {
            println!("求和结果:{}", v)
        }
    }
}

#[test]
fn test_sum() {
    let to_sum_normal = [1u32, 2u32, 4u32];
    call_sum(&to_sum_normal);
    let to_sum_overflow = [1u32, 2u32, 4u32, u32Max];
    call_sum(&to_sum_overflow);
}

/// 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束

fn main() {
    println!("Hello, world!");
}
