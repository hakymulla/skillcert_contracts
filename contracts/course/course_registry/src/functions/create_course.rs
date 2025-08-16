use crate::schema::Course;
use crate::utils::{to_lowercase, trim};
use soroban_sdk::{symbol_short, Address, Env, String, Symbol, Vec};
use crate::alloc::string::ToString;

const COURSE_KEY: Symbol = symbol_short!("course");
const TITLE_KEY: Symbol = symbol_short!("title");
const COURSE_ID: Symbol = symbol_short!("course");

pub fn course_registry_create_course(
    env: Env,
    title: String,
    description: String,
    price: u128,
    category: Option<String>,
    language: Option<String>,
    thumbnail_url: Option<String>,
) -> Course {
    let caller: Address = env.current_contract_address();

    let trimmed_title = trim(&env, &title);

    if title.is_empty() || trimmed_title.is_empty() {
        panic!("Course error: Course Title cannot be empty");
    }

    // ensure the price is greater than 0
    if price == 0 {
        panic!("Course error: Price must be greater than 0");
    }
     let lowercase_title = to_lowercase(&env, &title);

    // to avoid duplicate title,
    let title_key: (Symbol, String) = (
        TITLE_KEY,
        lowercase_title
    );

    if env.storage().persistent().has(&title_key) {
        panic!("Course error: Course Title already exists");
    }

    // generate the unique id
    let id: u128 = generate_course_id(&env);
    let converted_id: String = String::from_str(&env, id.to_string().as_str());

    let storage_key: (Symbol, String) = (COURSE_KEY, converted_id.clone());

    if env.storage().persistent().has(&storage_key) {
        panic!("Course with this ID already exists");
    }

    // create a new course
    let new_course: Course = Course {
        id: converted_id.clone(),
        title,
        description,
        creator: caller,
        price,
        category,
        language,
        thumbnail_url,
        published: false,
        prerequisites: Vec::new(&env),
    };

    // save to the storage
    env.storage().persistent().set(&storage_key, &new_course);
    env.storage().persistent().set(&title_key, &true);

    new_course
}

pub fn generate_course_id(env: &Env) -> u128 {
    let current_id: u128 = env.storage().persistent().get(&COURSE_ID).unwrap_or(0);
    let new_id = current_id + 1;
    env.storage().persistent().set(&COURSE_ID, &new_id);
    new_id
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::schema::Course;
    use crate::CourseRegistry;
    use soroban_sdk::{Address, Env, String};

    #[test]
    fn test_generate_course_id() {
        let env = Env::default();

        let contract_id: Address = env.register(CourseRegistry, {});
        env.as_contract(&contract_id, || {
            generate_course_id(&env);
            let id: u128 = generate_course_id(&env);
            assert_eq!(id, 2);
        });
    }

    #[test]
    fn test_add_module_success() {
        let env = Env::default();

        let contract_id: Address = env.register(CourseRegistry, {});
        let title: String = String::from_str(&env, "title");
        let description: String = String::from_str(&env, "A description");
        let price: u128 = 1000;
        let category: Option<String> = Some(String::from_str(&env, "Programming"));
        let language: Option<String> = Some(String::from_str(&env, "English"));
        let thumbnail_url: Option<String> =
            Some(String::from_str(&env, "https://example.com/thumb.jpg"));

        env.as_contract(&contract_id, || {
            course_registry_create_course(
                env.clone(),
                title.clone(),
                description.clone(),
                price,
                category.clone(),
                language.clone(),
                thumbnail_url.clone(),
            );
            // Verify course storage
            let storage_key: (Symbol, String) = (COURSE_KEY, String::from_str(&env, "1"));
            let stored_course: Option<Course> = env.storage().persistent().get(&storage_key);
            let course = stored_course.expect("Course should be stored");
            assert_eq!(course.title, title);
            assert_eq!(course.description, description);
            assert_eq!(course.id, String::from_str(&env, "1"));
            assert_eq!(course.price, price);
            assert_eq!(course.category, category);
            assert_eq!(course.language, language);
            assert_eq!(course.thumbnail_url, thumbnail_url);
            assert!(!course.published);
        });
    }

    #[test]
    fn test_add_module_success_multiple() {
        let env: Env = Env::default();

        let contract_id: Address = env.register(CourseRegistry, {});
        let title: String = String::from_str(&env, "title");
        let description: String = String::from_str(&env, "A description");
        let price: u128 = 1000;

        let another_course_title: String = String::from_str(&env, "another title");
        let another_course_description: String = String::from_str(&env, "another description");
        let another_price: u128 = 2000;

        env.as_contract(&contract_id, || {
            course_registry_create_course(
                env.clone(),
                title.clone(),
                description.clone(),
                price,
                None,
                None,
                None,
            );

            //create a second course
            course_registry_create_course(
                env.clone(),
                another_course_title.clone(),
                another_course_description.clone(),
                another_price,
                None,
                None,
                None,
            );

            let storage_key: (Symbol, String) = (COURSE_KEY, String::from_str(&env, "2"));

            let stored_course: Option<Course> = env.storage().persistent().get(&storage_key);

            let course: Course = stored_course.expect("Course should be stored");

            assert_eq!(course.title, another_course_title);
            assert_eq!(course.description, another_course_description);
            assert_eq!(course.id, String::from_str(&env, "2"));
            assert_eq!(course.price, another_price);
        });
    }

    #[test]
    #[should_panic(expected = "Course error: Course Title already exists")]
    fn test_cannot_create_courses_with_duplicate_title() {
        let env: Env = Env::default();
        let contract_id: Address = env.register(CourseRegistry, {});
        let title: String = String::from_str(&env, "title");
        let description: String = String::from_str(&env, "A description");
        let another_description: String = String::from_str(&env, "another description");
        let price: u128 = 1000;

        env.as_contract(&contract_id, || {
            course_registry_create_course(
                env.clone(),
                title.clone(),
                description.clone(),
                price,
                None,
                None,
                None,
            );

            // create another course with the same title
            course_registry_create_course(
                env.clone(),
                title.clone(),
                another_description.clone(),
                price,
                None,
                None,
                None,
            );
        })
    }

    #[test]
    #[should_panic(expected = "Course error: Course Title cannot be empty")]
    fn test_cannot_create_courses_with_empty_title() {
        let env: Env = Env::default();
        let contract_id: Address = env.register(CourseRegistry, {});
        let title: String = String::from_str(&env, "");
        let description: String = String::from_str(&env, "A description");
        let price: u128 = 1000;

        env.as_contract(&contract_id, || {
            course_registry_create_course(
                env.clone(),
                title.clone(),
                description.clone(),
                price,
                None,
                None,
                None,
            );
        })
    }

    #[test]
    #[should_panic(expected = "Course error: Price must be greater than 0")]
    fn test_cannot_create_courses_with_zero_price() {
        let env: Env = Env::default();
        let contract_id: Address = env.register(CourseRegistry, {});
        let title: String = String::from_str(&env, "Valid Title");
        let description: String = String::from_str(&env, "A description");
        let price: u128 = 0;

        env.as_contract(&contract_id, || {
            course_registry_create_course(
                env.clone(),
                title.clone(),
                description.clone(),
                price,
                None,
                None,
                None,
            );
        })
    }

    #[test]
    #[should_panic(expected = "Course error: Course Title cannot be empty")]
    fn test_cannot_create_courses_with_whitespace_only_title() {
        let env: Env = Env::default();
        let contract_id: Address = env.register(CourseRegistry, {});
        let title: String = String::from_str(&env, "   ");
        let description: String = String::from_str(&env, "A description");
        let price: u128 = 1000;

        env.as_contract(&contract_id, || {
            course_registry_create_course(
                env.clone(),
                title.clone(),
                description.clone(),
                price,
                None,
                None,
                None,
            );
        })
    }

    #[test]
    #[should_panic(expected = "Course error: Course Title already exists")]
    fn test_duplicate_title_case_insensitive() {
        let env: Env = Env::default();
        let contract_id: Address = env.register(CourseRegistry, {});
        let title1: String = String::from_str(&env, "Programming Basics");
        let title2: String = String::from_str(&env, "PROGRAMMING BASICS");
        let description: String = String::from_str(&env, "A description");
        let price: u128 = 1000;

        env.as_contract(&contract_id, || {
            course_registry_create_course(
                env.clone(),
                title1.clone(),
                description.clone(),
                price,
                None,
                None,
                None,
            );
            course_registry_create_course(
                env.clone(),
                title2.clone(),
                description.clone(),
                price,
                None,
                None,
                None,
            );
        })
    }

    #[test]
    fn test_create_course_with_long_title() {
        let env: Env = Env::default();
        let contract_id: Address = env.register(CourseRegistry, {});
        let long_title: String = String::from_str(&env, "This is a very long course title that contains many words and should still be valid for course creation as long as it is not empty");
        let description: String = String::from_str(&env, "A description");
        let price: u128 = 1500;

        env.as_contract(&contract_id, || {
            let course = course_registry_create_course(
                env.clone(),
                long_title.clone(),
                description.clone(),
                price,
                None,
                None,
                None,
            );
            assert_eq!(course.title, long_title);
            assert_eq!(course.price, price);
            assert_eq!(course.id, String::from_str(&env, "1"));
        })
    }

    #[test]
    fn test_create_course_with_special_characters() {
        let env: Env = Env::default();
        let contract_id: Address = env.register(CourseRegistry, {});
        let title: String = String::from_str(&env, "C++ & JavaScript: Advanced Programming!");
        let description: String = String::from_str(
            &env,
            "Learn C++ and JavaScript with special symbols: @#$%^&*()",
        );
        let price: u128 = 2500;

        env.as_contract(&contract_id, || {
            let course = course_registry_create_course(
                env.clone(),
                title.clone(),
                description.clone(),
                price,
                None,
                None,
                None,
            );
            assert_eq!(course.title, title);
            assert_eq!(course.description, description);
            assert_eq!(course.price, price);
        })
    }

    #[test]
    fn test_create_course_with_maximum_price() {
        let env: Env = Env::default();
        let contract_id: Address = env.register(CourseRegistry, {});
        let title: String = String::from_str(&env, "Premium Course");
        let description: String = String::from_str(&env, "Most expensive course");
        let max_price: u128 = u128::MAX;

        env.as_contract(&contract_id, || {
            let course = course_registry_create_course(
                env.clone(),
                title.clone(),
                description.clone(),
                max_price,
                None,
                None,
                None,
            );
            assert_eq!(course.price, max_price);
            assert_eq!(course.title, title);
        })
    }

    #[test]
    fn test_create_course_with_all_optional_fields() {
        let env: Env = Env::default();
        let contract_id: Address = env.register(CourseRegistry, {});
        let title: String = String::from_str(&env, "Complete Course");
        let description: String = String::from_str(&env, "Course with all fields");
        let price: u128 = 3000;
        let category: Option<String> = Some(String::from_str(&env, "Web Development"));
        let language: Option<String> = Some(String::from_str(&env, "Spanish"));
        let thumbnail_url: Option<String> = Some(String::from_str(
            &env,
            "https://example.com/course-thumbnail.png",
        ));

        env.as_contract(&contract_id, || {
            let course = course_registry_create_course(
                env.clone(),
                title.clone(),
                description.clone(),
                price,
                category.clone(),
                language.clone(),
                thumbnail_url.clone(),
            );
            assert_eq!(course.title, title);
            assert_eq!(course.description, description);
            assert_eq!(course.price, price);
            assert_eq!(course.category, category);
            assert_eq!(course.language, language);
            assert_eq!(course.thumbnail_url, thumbnail_url);
            assert!(!course.published);
        })
    }

    #[test]
    fn test_create_course_with_partial_optional_fields() {
        let env: Env = Env::default();
        let contract_id: Address = env.register(CourseRegistry, {});
        let title: String = String::from_str(&env, "Partial Course");
        let description: String = String::from_str(&env, "Course with some optional fields");
        let price: u128 = 1800;
        let category: Option<String> = Some(String::from_str(&env, "Data Science"));

        env.as_contract(&contract_id, || {
            let course = course_registry_create_course(
                env.clone(),
                title.clone(),
                description.clone(),
                price,
                category.clone(),
                None,
                None,
            );
            assert_eq!(course.title, title);
            assert_eq!(course.price, price);
            assert_eq!(course.category, category);
            assert_eq!(course.language, None);
            assert_eq!(course.thumbnail_url, None);
        })
    }

    #[test]
    fn test_create_course_empty_description() {
        let env: Env = Env::default();
        let contract_id: Address = env.register(CourseRegistry, {});
        let title: String = String::from_str(&env, "Course with Empty Description");
        let description: String = String::from_str(&env, "");
        let price: u128 = 1200;

        env.as_contract(&contract_id, || {
            let course = course_registry_create_course(
                env.clone(),
                title.clone(),
                description.clone(),
                price,
                None,
                None,
                None,
            );
            assert_eq!(course.title, title);
            assert_eq!(course.description, description);
            assert_eq!(course.price, price);
        })
    }

    #[test]
    fn test_create_multiple_courses_sequential_ids() {
        let env: Env = Env::default();
        let contract_id: Address = env.register(CourseRegistry, {});
        let price: u128 = 1000;

        env.as_contract(&contract_id, || {
            let course1 = course_registry_create_course(
                env.clone(),
                String::from_str(&env, "Course One"),
                String::from_str(&env, "First course"),
                price,
                None,
                None,
                None,
            );

            let course2 = course_registry_create_course(
                env.clone(),
                String::from_str(&env, "Course Two"),
                String::from_str(&env, "Second course"),
                price,
                None,
                None,
                None,
            );

            let course3 = course_registry_create_course(
                env.clone(),
                String::from_str(&env, "Course Three"),
                String::from_str(&env, "Third course"),
                price,
                None,
                None,
                None,
            );

            assert_eq!(course1.id, String::from_str(&env, "1"));
            assert_eq!(course2.id, String::from_str(&env, "2"));
            assert_eq!(course3.id, String::from_str(&env, "3"));
        })
    }

    #[test]
    fn test_create_course_with_unicode_characters() {
        let env: Env = Env::default();
        let contract_id: Address = env.register(CourseRegistry, {});
        let title: String = String::from_str(&env, "Programación en Español 🚀");
        let description: String = String::from_str(
            &env,
            "Curso de programación con caracteres especiales: áéíóú ñ",
        );
        let price: u128 = 2000;
        let language: Option<String> = Some(String::from_str(&env, "Español"));

        env.as_contract(&contract_id, || {
            let course = course_registry_create_course(
                env.clone(),
                title.clone(),
                description.clone(),
                price,
                None,
                language.clone(),
                None,
            );
            assert_eq!(course.title, title);
            assert_eq!(course.description, description);
            assert_eq!(course.language, language);
        })
    }
}
