const handleSubmit = (event) => {
  event.preventDefault();
  const inputFile = document.getElementById("file");
  const inputFromSec = document.getElementById("from_sec");
  const inputToSec = document.getElementById("to_sec");
  const inputSpeed = document.getElementById("speed");

  const formData = new FormData();
  const headers = new Headers();

  for (const file of inputFile.files) {
    formData.append("files", file);
  }

  headers.append("from_sec", inputFromSec.value);
  headers.append("to_sec", inputToSec.value);
  headers.append("speed", inputSpeed.value);
  headers.append("Access-Control-Allow-Origin", "*");

  fetch("http://localhost:8080/make_boomerang", {
    method: "post",
    body: formData,
    headers: headers,
  }).catch((error) => ("Something went wrong!", error));
};

form.addEventListener("submit", handleSubmit);
