use crate::CourseRegistry;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env, String, Symbol, Vec};
use crate::alloc::string;
use crate::alloc::string::ToString;
use alloc::{format, string::String as RustString};
use crate::{
    functions::{
        get_course::course_registry_get_course,
        get_courses_by_instructor::course_registry_get_courses_by_instructor,
        get_prerequisites_by_course::get_prerequisites_by_course_id,
        remove_module::course_registry_remove_module,
    },
    schema::{Course, CourseModule, DataKey},
};

const COURSE_KEY: Symbol = symbol_short!("course");

// Helpers
fn create_test_env() -> Env {
    let env = Env::default();
    env.budget().reset_unlimited();
    env
}

fn create_sample_module(env: &Env) -> CourseModule {
    CourseModule {
        id: String::from_str(env, "test_module_123"),
        course_id: String::from_str(env, "test_course_123"),
        position: 0,
        title: String::from_str(env, "Introduction to Blockchain"),
        created_at: 0,
    }
}

fn create_sample_course(env: &Env, id: u128, creator: Address) -> Course {
    Course {
        id: String::from_str(env, &id.to_string()),
        title: String::from_str(env, &format!("Course {}", id)),
        description: String::from_str(env, "Test Description"),
        creator,
        published: true,
        price: 1000,
        category: Some(String::from_str(env, "Programming")),
        language: Some(String::from_str(env, "English")),
        thumbnail_url: None,
        prerequisites: Vec::new(&env),
    }
}

// 🔹 Tests

#[test]
fn test_remove_module_success() {
    let env = create_test_env();
    let contract = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let contract_id = contract.address();
    let module = create_sample_module(&env);
    let module_id = module.id.clone();

    env.mock_all_auths();

    env.as_contract(&contract_id, || {
        let storage = env.storage().persistent();
        let key = DataKey::Module(module_id.clone());
        storage.set(&key, &module);
    });

    let result = env.as_contract(&contract_id, || {
        course_registry_remove_module(&env, module_id.clone())
    });

    assert!(result.is_ok());

    env.as_contract(&contract_id, || {
        let storage = env.storage().persistent();
        let key = DataKey::Module(module_id.clone());
        assert!(!storage.has(&key));
    });
}

#[test]
fn test_remove_multiple_different_modules() {
    let env = create_test_env();
    let contract = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let contract_id = contract.address();

    env.mock_all_auths();

    let mut module1 = create_sample_module(&env);
    module1.id = String::from_str(&env, "module_1");

    let mut module2 = create_sample_module(&env);
    module2.id = String::from_str(&env, "module_2");

    env.as_contract(&contract_id, || {
        let storage = env.storage().persistent();
        storage.set(&DataKey::Module(module1.id.clone()), &module1);
        storage.set(&DataKey::Module(module2.id.clone()), &module2);
    });

    let result1 = env.as_contract(&contract_id, || {
        course_registry_remove_module(&env, module1.id.clone())
    });
    assert!(result1.is_ok());

    let result2 = env.as_contract(&contract_id, || {
        course_registry_remove_module(&env, module2.id.clone())
    });
    assert!(result2.is_ok());

    env.as_contract(&contract_id, || {
        let storage = env.storage().persistent();
        assert!(!storage.has(&DataKey::Module(module1.id.clone())));
        assert!(!storage.has(&DataKey::Module(module2.id.clone())));
    });
}

#[test]
fn test_remove_module_storage_isolation() {
    let env = create_test_env();
    let contract = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let contract_id = contract.address();
    let module = create_sample_module(&env);
    let module_id = module.id.clone();

    env.mock_all_auths();

    env.as_contract(&contract_id, || {
        let storage = env.storage().persistent();
        storage.set(&DataKey::Module(module_id.clone()), &module);

        let other_key = DataKey::Module(String::from_str(&env, "other_module"));
        let mut other_module = create_sample_module(&env);
        other_module.id = String::from_str(&env, "other_module");
        storage.set(&other_key, &other_module);
    });

    let result = env.as_contract(&contract_id, || {
        course_registry_remove_module(&env, module_id)
    });
    assert!(result.is_ok());

    env.as_contract(&contract_id, || {
        let storage = env.storage().persistent();
        assert!(!storage.has(&DataKey::Module(module.id.clone())));
        assert!(storage.has(&DataKey::Module(String::from_str(&env, "other_module"))));
    });
}

#[test]
fn test_get_course_success() {
    let env = Env::default();
    let course_id = String::from_str(&env, "course_123");
    let title = String::from_str(&env, "Test Course");
    let description = String::from_str(&env, "A test course description");
    let creator = Address::generate(&env);
    let published = true;
    let price = 23;

    let course = Course {
        id: course_id.clone(),
        title: title.clone(),
        description: description.clone(),
        price: price,
        creator: creator.clone(),
        published,
        category: None,
        language: None,
        thumbnail_url: None,
        prerequisites: Vec::new(&env),
    };

    let contract_id = env.register_contract(None, crate::CourseRegistry);

    let key = Symbol::new(&env, "course");
    env.as_contract(&contract_id, || {
        env.storage()
            .instance()
            .set(&(key, course_id.clone()), &course);
    });

    let retrieved = env.as_contract(&contract_id, || course_registry_get_course(&env, course_id));

    assert_eq!(retrieved.id, course.id);
    assert_eq!(retrieved.title, course.title);
    assert_eq!(retrieved.description, course.description);
    assert_eq!(retrieved.creator, course.creator);
    assert_eq!(retrieved.published, course.published);
}

#[test]
#[should_panic(expected = "Course not found")]
fn test_get_course_not_found() {
    let env = Env::default();
    let contract_id = env.register_contract(None, crate::CourseRegistry);

    let fake_id = String::from_str(&env, "not_found");

    env.as_contract(&contract_id, || course_registry_get_course(&env, fake_id));
}

#[test]
fn test_get_courses_by_instructor_empty() {
    let env = Env::default();
    let instructor = Address::generate(&env);
    let contract_id = env.register_contract(None, crate::CourseRegistry);

    let courses = env.as_contract(&contract_id, || {
        course_registry_get_courses_by_instructor(&env, instructor.clone())
    });

    assert_eq!(courses.len(), 0);
}

#[test]
fn test_get_courses_by_instructor_found() {
    let env = Env::default();
    let instructor = Address::generate(&env);
    let contract_id = env.register_contract(None, crate::CourseRegistry);

    let course_id = String::from_str(&env, "1");
    let course = Course {
        id: course_id.clone(),
        title: String::from_str(&env, "Rust 101"),
        description: String::from_str(&env, "Intro to Rust"),
        creator: instructor.clone(),
        published: true,
        price: 1500,
        category: Some(String::from_str(&env, "Programming")),
        language: Some(String::from_str(&env, "English")),
        thumbnail_url: None,
        prerequisites: Vec::new(&env),
    };

    let key = (symbol_short!("course"), course_id);
    env.as_contract(&contract_id, || {
        env.storage().persistent().set(&key, &course);
    });

    let results = env.as_contract(&contract_id, || {
        course_registry_get_courses_by_instructor(&env, instructor.clone())
    });

    assert_eq!(results.len(), 1);
    assert_eq!(results.get(0).unwrap().id, course.id);
}

#[test]
fn test_get_prerequisites_by_course_id() {
    let env = Env::default();
    let contract_id = env.register_contract(None, crate::CourseRegistry);
    //let contract_id: Address = env.register(CourseRegistry, {});
    let course_id = String::from_str(&env, "course_123");

    let course = Course {
        id: course_id.clone(),
        title: String::from_str(&env, "Test Course"),
        description: String::from_str(&env, "Test Description"),
        creator: Address::generate(&env),
        published: true,
        price: 1000,
        category: None,
        language: Some(String::from_str(&env, "English")),
        thumbnail_url: None,
        prerequisites: Vec::new(&env),
    };
    let key = (symbol_short!("course"), course_id.clone());
    env.as_contract(&contract_id, || {
        env.storage().persistent().set(&key, &course);
    });
    let prerequisites = env.as_contract(&contract_id, || {
        get_prerequisites_by_course_id(&env, course_id.clone())
    });
    assert!(prerequisites.is_empty());
}
