import * as wasm from "foto-web";

const button = document.getElementById("button");
const canvas = document.getElementById("myCanvas");
const context = canvas.getContext("2d");
let imageData;

button.addEventListener("click", edge);

window.onload = function () {
  const image = new Image();
  image.onload = function () {
    context.drawImage(image, 0, 0);
    imageData = context.getImageData(0, 0, canvas.width, canvas.height).data;
  };
  image.src = "img/unknown.png";
};

function edge() {
  const grayData = wasm.rgb_to_gray(imageData, canvas.width, canvas.height);
  const edgeData = wasm.sobel_edge_detection(
    grayData,
    canvas.width,
    canvas.height
  );
  const gray = document.getElementById("gray");
  const ctx = gray.getContext("2d");

  var uint8clamped = new Uint8ClampedArray(edgeData.buffer);
  console.log(uint8clamped);
  let grayImageData = new ImageData(uint8clamped, canvas.width, canvas.height);
  ctx.putImageData(grayImageData, 0, 0);
}
