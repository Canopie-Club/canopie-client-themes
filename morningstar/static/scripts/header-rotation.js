// ../morningstar/frontend/javascript/header-rotation.ts
function updateHeaderRotation() {
  const bandSection = document.querySelector("section#band");
  if (!bandSection)
    return;
  const header = document.querySelector(".band-header");
  if (!header)
    return;
  const vw = window.innerWidth;
  let angle = 0;
  const [itemWidth, itemHeight] = [header.offsetWidth, header.offsetHeight];
  const left = header.offsetLeft;
  const padding = 100;
  const max = left + itemWidth + padding;
  const min = left + itemHeight + padding;
  if (vw >= max) {
    angle = 0;
  } else if (vw <= min) {
    angle = 90;
  } else {
    const t = (max - vw) / (max - min);
    angle = t * 90;
  }
  bandSection.style.setProperty("--header-angle", `${angle}deg`);
}
window.onload = updateHeaderRotation;
window.addEventListener("resize", updateHeaderRotation);
