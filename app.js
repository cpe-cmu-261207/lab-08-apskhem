const express = require("express");
const app = express();
const bodyParser = require("body-parser");
const fs = require("fs");

app.use(express.static("assets"));
app.use(express.static("build"));
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

/* @get[/] */
app.get("/", (req, res) => {
    res.end(fs.readFileSync("./pages/index.html"));
});

/* @get[/] */
app.get("/instruction", (req, res) => {
    res.end(fs.readFileSync("./pages/instruction.html"));
});
  
/* @get[/courses] */
app.get("/courses", (req, res) => {
    const rawdata = fs.readFileSync("myCourses.json");
    const courseData = JSON.parse(rawdata);
  
    res.json(courseData);
});
  
/* @get[/courses/:id] */
app.get("/courses/:id", (req, res) => {
    const rawdata = fs.readFileSync("myCourses.json");
    const courseData = JSON.parse(rawdata);
  
    const course = courseData.courses.find((x) => +x.courseId === +req.params.id);
  
    course
      ? res.json(course)
      : res.status(404).send("error 404, no course fouded");
});
  
/* @delete[/courses/:id] */
app.delete("/courses/:id", (req, res) => {
    const rawdata = fs.readFileSync("myCourses.json");
    const courseData = JSON.parse(rawdata);
  
    courseData.courses = courseData.courses.filter((x) => +x.courseId !== +req.params.id);
  
    // calculate gpax
    const [ weightSum, weight ] = courseData.courses.reduce((acc, course) => {
      acc[0] += +course.gpa * +course.credit;
      acc[1] += +course.credit;
  
      return acc;
    }, [ 0, 0 ]);
  
    courseData.gpax = +(weight ? weightSum / weight : 0).toFixed(2);
  
    // write file and response
    fs.writeFileSync("myCourses.json", JSON.stringify(courseData, null, 4));
    res.send("Delete successfully");
});
  
/* @post[/addCourse] */
app.post("/addCourse", (req, res) => {
    const rawdata = fs.readFileSync("myCourses.json");
    const courseData = JSON.parse(rawdata);
  
    // validate
    if (
        !req.body.courseId
        || !req.body.courseName
        || !req.body.credit
        || !req.body.gpa
        || req.body.courseId.length !== 6
    ) {
      // 400 BAD request
      res.send(400).send("Submit failed successfully");
    }
  
    // append new data
    courseData.courses.push({
        courseId: +req.body.courseId,
        courseName: req.body.courseName,
        credit: +req.body.credit,
        gpa: +req.body.gpa
    });
  
    // calculate gpax
    const [ weightSum, weight ] = courseData.courses.reduce((acc, course) => {
        acc[0] += +course.gpa * +course.credit;
        acc[1] += +course.credit;
  
        return acc;
    }, [ 0, 0 ]);
  
    courseData.gpax = +(weight ? weightSum / weight : 0).toFixed(2);
  
    // write file and response
    fs.writeFileSync("myCourses.json", JSON.stringify(courseData, null, 4));
    res.send("Submit successfully");
});
  
const port = process.env.PORT || 8000;
app.listen(port, () => console.log(`server started on http://localhost:${port}`));