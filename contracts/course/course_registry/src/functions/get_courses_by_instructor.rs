use crate::schema::Course;
use soroban_sdk::{symbol_short, Address, Env, String, Symbol, Vec};
use crate::alloc::string::ToString;

const COURSE_KEY: Symbol = symbol_short!("course");

pub fn course_registry_get_courses_by_instructor(env: &Env, instructor: Address) -> Vec<Course> {
    let mut results: Vec<Course> = Vec::new(env);
    let mut id: u128 = 1;

    loop {
        let course_id = String::from_str(env, &id.to_string().trim());
        let key = (COURSE_KEY, course_id.clone());

        if !env.storage().persistent().has(&key) {
            break;
        }

        let course: Course = env.storage().persistent().get(&key).unwrap();

        if course.creator == instructor {
            results.push_back(course);
        }

        id += 1;
        if id > 1000 {
            break; // safety limit
        }
    }

    results
}
