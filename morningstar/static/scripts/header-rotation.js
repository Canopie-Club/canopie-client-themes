function updateHeaderRotation() {
    // const header = document.querySelector<HTMLElement>("section#band::after");
    var bandSection = document.querySelector("section#band");
    if (!bandSection) return;
    var header = document.querySelector(".band-header");
    if (!header) return;
    var vw = window.innerWidth;
    var angle = 0;
    var _ref = [
        header.offsetWidth,
        header.offsetHeight
    ], itemWidth = _ref[0], itemHeight = _ref[1];
    var left = header.offsetLeft;
    var padding = 100;
    var max = left + itemWidth + padding;
    var min = left + itemHeight + padding;
    if (vw >= max) {
        angle = 0;
    } else if (vw <= min) {
        angle = 90;
    } else {
        // Linear interpolation between 880px -> 0° and 700px -> 90°
        var t = (max - vw) / (max - min); // value between 0 and 1
        angle = t * 90;
    }
    bandSection.style.setProperty("--header-angle", "".concat(angle, "deg"));
}
// Run once on load
window.onload = updateHeaderRotation;
// Update on resize
window.addEventListener("resize", updateHeaderRotation);

