trait Sleep: Sized {
    type Env: SleepEnv;

    fn sleep(&self) {
        Env::do_sleep(self);
    }

    fn get_name(&self) -> &str;
}

trait SleepEnv {
    fn do_sleep<T: Sleep>(&T);
}

enum SleepEnvType {
    Bed,
    Tent,
}

use SleepEnvType::*;

impl SleepEnv for SleepEnvType::Bed {
    fn do_sleep<T: Sleep>(person: &T) {
        println!("{} is sleeping in bed", person.get_name());
//        match *self {
//            Bed => {
//                println!("{} is sleeping in bed", person.get_name());
//            }
//            Tent => {
//                println!("{} is sleeping in tent", person.get_name());
//            }
//        }
    }
}

impl SleepEnv for SleepEnvType::Tent {
    fn do_sleep<T: Sleep>(person: &T) {
        println!("{} is sleeping in tent", person.get_name());
    }
}

struct Jim;

struct Jane;

impl Sleep for Jim {
    type Env = Bed;
    fn get_name(&self) -> &str {
        "Jim"
    }
}

impl Sleep for Jane {
    type Env = Tent;
    fn get_name(&self) -> &str {
        "Jane"
    }
}

fn main() {
    let jim = Jim;
    let jane = Jane;
    jim.sleep();
    jane.sleep();
}