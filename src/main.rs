#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::fs;
use std::io::Cursor;
use std::io::prelude::*;
use rocket::response::content;
use rocket::Response;
use rocket::Data;
use rocket::http::{Status, ContentType};
use rocket_contrib::serve::StaticFiles;
use serde_json::{Result, Value, json};
use rocket::request::Form;
use rocket_contrib::json::Json;
mod gpax;

#[derive(FromForm)]
struct Course {
    #[form(field = "courseId")]
    course_id: String,
    #[form(field = "courseName")]
    course_name: String,
    #[form(field = "credit")]
    credit: String,
    #[form(field = "gpa")]
    gpa: String,
}

#[get("/")]
fn index() -> content::Html<String> {
    let html = fs::read_to_string("public/index.html").expect("Cannot find the specific file");

    return content::Html(html);
}

#[get("/instruction")]
fn instruction() -> content::Html<String> {
    let html = fs::read_to_string("public/instruction.html").expect("Cannot find the specific file");

    return content::Html(html);
}

#[get("/courses")]
fn get_courses() -> content::Json<String> {
    let my_courses = fs::read_to_string("myCourses.json").expect("Cannot find the specific file");
    let json: serde_json::Value = serde_json::from_str(my_courses.as_str()).expect("JSON was not well-formatted");

    let res = json!({ "success": true, "data": json });

    return content::Json(res.to_string());
}

#[get("/courses/<id>")]
fn get_courses_id(id: i64) -> Response<'static> {
    let my_courses = fs::read_to_string("myCourses.json").expect("Something went wrong reading the file");
    let json: serde_json::Value = serde_json::from_str(my_courses.as_str()).expect("JSON was not well-formatted");
    let course_arr = &json["courses"];

    let mut res = Response::new();
    res.set_header(ContentType::JSON);

    for course in course_arr.as_array().unwrap().iter() {
        if id == course["courseId"].as_i64().unwrap() {
            res.set_sized_body(Cursor::new(course.to_string()));
            return res;
        }
    }

    res.set_status(Status::NotFound);
    return res;
}

#[delete("/courses/<id>")]
fn delete_courses_id(id: i64) -> content::Json<String> {
    let my_courses = fs::read_to_string("myCourses.json").expect("Cannot find the specific file");
    let json: serde_json::Value = serde_json::from_str(my_courses.as_str()).expect("JSON was not well-formatted");
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
    let mut file = fs::File::create("myCourses.json").expect("err");
    file.write_all(sav.to_string().as_bytes()).expect("err");

    return content::Json(res.to_string());
}

#[post("/addCourse?<courseId>&<courseName>&<credit>&<gpa>")]
fn add_course(courseId: u32, courseName: String, credit: u8, gpa: u8) -> Response<'static> {
    let my_courses = fs::read_to_string("myCourses.json").expect("Cannot find the specific file");
    let json: serde_json::Value = serde_json::from_str(my_courses.as_str()).expect("JSON was not well-formatted");
    let course_arr = &json["courses"];

    let mut res = Response::new();
    res.set_header(ContentType::JSON);
    
    // format req body
    let formatted_course_json = json!({
        "courseId": courseId,
        "courseName": courseName,
        "credit": credit,
        "gpa": gpa
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
    let mut file = fs::File::create("myCourses.json").expect("err");
    file.write_all(sav.to_string().as_bytes()).expect("err");

    res.set_status(Status::Created);
    res.set_sized_body(Cursor::new(res_text.to_string()));
    return res;
}

fn main() {
    rocket::ignite()
    .mount("/", StaticFiles::from("assets"))
    .mount("/", routes![index, instruction, get_courses, get_courses_id, delete_courses_id, add_course])
    .launch();
}