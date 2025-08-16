#![no_std]
extern crate alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod functions;
pub mod schema;
pub mod utils;

#[cfg(test)]
mod test;

use crate::schema::{Course, CourseModule, CourseGoal};
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

#[contract]
pub struct CourseRegistry;

#[contractimpl]
impl CourseRegistry {
    pub fn create_course(
        env: Env,
        title: String,
        description: String,
        price: u128,
        category: Option<String>,
        language: Option<String>,
        thumbnail_url: Option<String>,
    ) -> Course {
        functions::create_course::course_registry_create_course(
            env,
            title,
            description,
            price,
            category,
            language,
            thumbnail_url,
        )
    }

    pub fn get_course(env: Env, course_id: String) -> Course {
        functions::get_course::course_registry_get_course(&env, course_id)
    }

    pub fn get_courses_by_instructor(env: Env, instructor: Address) -> Vec<Course> {
        functions::get_courses_by_instructor::course_registry_get_courses_by_instructor(
            &env, instructor,
        )
    }

    pub fn remove_module(env: Env, module_id: String) -> () {
        functions::remove_module::course_registry_remove_module(&env, module_id)
            .unwrap_or_else(|e| panic!("{}", e))
    }

    pub fn add_module(env: Env, course_id: String, position: u32, title: String) -> CourseModule {
        functions::add_module::course_registry_add_module(env, course_id, position, title)
    }

    pub fn delete_course(env: Env, course_id: String) -> () {
        functions::delete_course::course_registry_delete_course(&env, course_id)
            .unwrap_or_else(|e| panic!("{}", e))
    }

    pub fn hello_world(_env: Env) -> String {
        String::from_str(&_env, "Hello from Web3 👋")
    }

    pub fn add_goal(env: Env, course_id: String, content: String) -> CourseGoal {
    functions::add_goal::course_registry_add_goal(env, course_id, content)
}
  
    pub fn remove_prerequisite(env: Env, course_id: String, prerequisite_course_id: String) {
        functions::remove_prerequisite::course_registry_remove_prerequisite(env, course_id, prerequisite_course_id)
    }

    pub fn edit_prerequisite(env: Env, course_id: String, new_prerequisites: Vec<String>) {
        functions::edit_prerequisite::course_registry_edit_prerequisite(env, course_id, new_prerequisites)
    }

    pub fn is_course_creator(env: &Env, course_id: String, user: Address) -> bool {
        functions::is_course_creator::is_course_creator(env, course_id, user)
    }

}
