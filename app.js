const express = require("express");
const app = express();
const bodyParser = require("body-parser");
const fs = require("fs");

//to post you must use bodyParser

app.use(express.static("assets"));

app.get("/", (req, res) => {
  res.end(fs.readFileSync("./instruction.html"));
});

//implement your api here
//follow instruction in http://localhost:8000/

const port = process.env.PORT || 8000;
app.listen(port, () => console.log(`server started on port:${port}`));
