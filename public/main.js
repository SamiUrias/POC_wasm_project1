async function init() {
  let rustApp = null;
  try {
    rustApp = await import("../pkg");
  } catch (e) {
    console.log(e);
    return;
  }

  console.log(rustApp);
  const input = document.getElementById("upload");
  const fileReader = new FileReader();

  fileReader.onloadend = () => {
    // let base64 = fileReader.result;
    let base64 = fileReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/,
      ""
    );

    // console.log(input.files[0]);
    // console.log(base64);
    const image_data_url = rustApp.grayscale(base64);
    document.getElementById("new-img").setAttribute("src", image_data_url);
  };
  input.addEventListener("change", () => {
    // Reading the file and store it our javascript app
    // input.files[0]; // Returns a file object
    fileReader.readAsDataURL(input.files[0]);
    // fileReader.result; // Return s string

    // Base64 was introduced to move binary data as string
  });
}

init();
