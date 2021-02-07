#[post("/addCourse", data = "<course>")]
fn add_course(course: String) -> Response<'static> {
    let my_courses = fs::read_to_string("myCourses.json").expect("Cannot find the specific file");
    let json: serde_json::Value = serde_json::from_str(my_courses.as_str()).expect("JSON was not well-formatted");
    let course_arr = &json["courses"];

    let mut res = Response::new();
    res.set_header(ContentType::JSON);

    // validate
    if !course.contains("courseId")
    || !course.contains("courseName")
    || !course.contains("credit")
    || !course.contains("gpa") {
        res.set_status(Status::new(422, "Unprocessable Entity"));
        return res;
    }
    
    // format req body
    let req_course_json: serde_json::Value = serde_json::from_str(course.as_str()).expect("JSON was not well-formatted");
    let formatted_course_json = json!({
        "courseId": req_course_json["courseId"].to_string().replace("\"", "").parse::<u32>().unwrap(),
        "courseName": req_course_json["courseName"],
        "credit": req_course_json["credit"].to_string().replace("\"", "").parse::<u32>().unwrap(),
        "gpa": req_course_json["gpa"].to_string().replace("\"", "").parse::<u32>().unwrap()
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