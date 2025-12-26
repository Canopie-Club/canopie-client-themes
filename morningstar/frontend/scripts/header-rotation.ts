function updateHeaderRotation() {
	// const header = document.querySelector<HTMLElement>("section#band::after");
	const bandSection = document.querySelector<HTMLElement>("section#band");
	if (!bandSection) return;

	const header = document.querySelector<HTMLElement>(".band-header");
	if (!header) return;

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
		// Linear interpolation between 880px -> 0° and 700px -> 90°
		const t = (max - vw) / (max - min); // value between 0 and 1
		angle = t * 90;
	}

	bandSection.style.setProperty("--header-angle", `${angle}deg`);
}

// Run once on load
window.onload = updateHeaderRotation;

// Update on resize
window.addEventListener("resize", updateHeaderRotation);
