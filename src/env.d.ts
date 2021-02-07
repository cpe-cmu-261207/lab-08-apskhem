interface MyCourseJSON {
    courses: Course[];
    gpax: number;
}

interface Course {
    courseId: number;
    courseName: string;
    credit: 1 | 2 | 3;
    gpa: number;
}