use super::errors::EzyTutorError;
use super::models::Course;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_tutor_db(
    pool: &PgPool,
    tutor_id: i32,
) -> Result<Vec<Course>, EzyTutorError> {
    // Prepare SQL statement
    let course_rows = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c4 where tutor_id = $1",
        tutor_id
    ).fetch_all(pool).await?; // Execute the query

    // Extract results into a Rust vector
    let courses: Vec<Course> = course_rows
        .iter() // Convert the retrieved DB records into a Rust iterator
        .map(|course_row| Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
        })
        .collect(); // Accumulate the Course structs returned from the map() into a Rust Vec

    match courses.len() {
        0 => Err(EzyTutorError::NotFound(
            "Courses not found for tutor".into(),
        )),
        _ => Ok(courses),
    }
}

pub async fn get_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<Course, EzyTutorError> {
    // Prepare SQL statement
    let course_row = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM
           ezy_course_c4 where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id
    )
    .fetch_one(pool) // Execute the query - note the use of fetch_one to get only 1 row
    .await; // We are making an asynchronous call to the DB

    if let Ok(course_row) = course_row {
        Ok(Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
        })
    } else {
        Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Course {
    // Prepare the query to insert a new course into the DB and return it in a single operation
    let course_row = sqlx::query!(
        "insert into ezy_course_c4 (course_id,tutor_id, course_name) values ($1,$2,$3)
            returning tutor_id, course_id,course_name, posted_time",
        new_course.course_id,
        new_course.tutor_id,
        new_course.course_name
    )
    .fetch_one(pool) // After inserting, fetch the inserted course
    .await
    .unwrap();

    //Retrieve result and return the newly inserted Course
    Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
    }
}
