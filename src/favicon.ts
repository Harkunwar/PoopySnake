const FAVICON_SIZE = 512;

export function initFavicon() {
  const faviconCanvas = document.createElement("canvas");
  faviconCanvas.width = FAVICON_SIZE;
  faviconCanvas.height = FAVICON_SIZE;

  const context = faviconCanvas.getContext("2d")!;

  context.beginPath();
  context.font = context.font.replace(/\d+px/, `${FAVICON_SIZE}px`);
  context.fillText("💩", 0, FAVICON_SIZE, FAVICON_SIZE);
  context.stroke();

  const image = faviconCanvas.toDataURL("image/png");

  const faviconElement = document.createElement("link");
  faviconElement.rel = "icon";
  const headElement = document.querySelector("head")!;
  headElement.appendChild(faviconElement);
  faviconElement.href = image;
  faviconCanvas.remove();
}
