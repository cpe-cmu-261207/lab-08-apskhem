#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_json;

use std::fs;
use std::io::Cursor;
use std::io::Error;
use std::io::prelude::*;
use std::result::Result;
use rocket::{Data, Response};
use rocket::request::Form;
use rocket::response::content;
use rocket::http::{Status, ContentType};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
mod gpax;

#[derive(Deserialize)]
struct Course {
    courseId: u32,
    courseName: String,
    credit: u8,
    gpa: f32,
}

#[get("/")]
fn index() -> Result<content::Html<String>, Error> {
    let html = fs::read_to_string("public/index.html")?;

    return Ok(content::Html(html));
}

#[get("/instruction")]
fn instruction() -> Result<content::Html<String>, Error> {
    let html = fs::read_to_string("public/instruction.html")?;

    return Ok(content::Html(html));
}

#[get("/courses")]
fn get_courses() -> Result<content::Json<String>, Error> {
    let my_courses = fs::read_to_string("myCourses.json")?;
    let json: serde_json::Value = serde_json::from_str(my_courses.as_str())?;

    let res = json!({ "success": true, "data": json });

    return Ok(content::Json(res.to_string()));
}

#[get("/courses/<id>")]
fn get_courses_id(id: i64) -> Result<Response<'static>, Error> {
    let my_courses = fs::read_to_string("myCourses.json")?;
    let json: serde_json::Value = serde_json::from_str(my_courses.as_str())?;
    let course_arr = &json["courses"];

    let mut res = Response::new();
    res.set_header(ContentType::JSON);

    for course in course_arr.as_array().unwrap().iter() {
        if id == course["courseId"].as_i64().unwrap() {
            res.set_sized_body(Cursor::new(course.to_string()));
            return Ok(res);
        }
    }

    res.set_status(Status::NotFound);
    return Ok(res);
}

#[delete("/courses/<id>")]
fn delete_courses_id(id: i64) -> Result<content::Json<String>, Error> {
    let my_courses = fs::read_to_string("myCourses.json")?;
    let json: serde_json::Value = serde_json::from_str(my_courses.as_str())?;
    let course_arr = &json["courses"];

    // filter out the requested id
    let mut new_arr = vec![];

    for course in course_arr.as_array().unwrap().iter() {
        if id != course["courseId"].as_i64().unwrap() {
            new_arr.push(course);
        }
    }

    // group json
    let res = json!({ "success": true, "data": new_arr });
    let sav = json!({ "courses": new_arr, "gpax": gpax::cal_gpax(&new_arr) });

    // overwrite old file
    let mut file = fs::File::create("myCourses.json")?;
    file.write_all(sav.to_string().as_bytes())?;

    return Ok(content::Json(res.to_string()));
}

#[post("/addCourse", data = "<course>")]
fn add_course(course: Json<Course>) -> Result<Response<'static>, Error> {
    let my_courses = fs::read_to_string("myCourses.json")?;
    let json: serde_json::Value = serde_json::from_str(my_courses.as_str())?;
    let course_arr = &json["courses"];

    let mut res = Response::new();
    res.set_header(ContentType::JSON);
    
    // format req body
    let formatted_course_json = json!({
        "courseId": course.courseId,
        "courseName": course.courseName,
        "credit": course.credit,
        "gpa": course.gpa
    });

    // create new arr
    let mut new_arr = vec![];
    for course in course_arr.as_array().unwrap().iter() {
        new_arr.push(course);
    }

    new_arr.push(&formatted_course_json);

    // group json
    let res_text = json!({ "success": true, "data": formatted_course_json });
    let sav = json!({ "courses": new_arr, "gpax": gpax::cal_gpax(&new_arr) });

    // overwrite old file
    let mut file = fs::File::create("myCourses.json")?;
    file.write_all(sav.to_string().as_bytes())?;

    res.set_status(Status::Created);
    res.set_sized_body(Cursor::new(res_text.to_string()));
    return Ok(res);
}

fn main() {
    rocket::ignite()
    .mount("/", StaticFiles::from("assets"))
    .mount("/", routes![index, instruction, get_courses, get_courses_id, delete_courses_id, add_course])
    .launch();
}