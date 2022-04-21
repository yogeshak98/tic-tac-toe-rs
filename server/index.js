const express = require("express");
const compression  = require("compression");
const path = require("path");

const app = express();
const port = process.env.PORT || 3000;

const public = path.join(__dirname, "..", "www", "dist");
app.use(compression());

app.use(express.static(public));

app.get("/", (req, res) => {
    res.sendFile("index.html");
});

app.get('/favicon.ico', (req, res) => {
    res.status(204).send();
});


app.listen(port, () => {
    console.log("server is running!");
});
