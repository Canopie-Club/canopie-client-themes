document.addEventListener("DOMContentLoaded", () => {
	const items = document.querySelectorAll<HTMLElement>("section[id]");
	const visibleItems = new Set();

	const observer = new IntersectionObserver(
		(entries) => {
			entries.forEach((entry) => {
				if (entry.isIntersecting) {
					visibleItems.add(entry.target);
				} else {
					visibleItems.delete(entry.target);
				}
			});

			console.log([...visibleItems]); // Always correct
		},
		{ threshold: 0 },
	);

	// Observe + initial visibility check
	items.forEach((item) => {
		observer.observe(item);

		const rect = item.getBoundingClientRect();
		const inView =
			rect.bottom > 0 &&
			rect.right > 0 &&
			rect.top < window.innerHeight &&
			rect.left < window.innerWidth;

		if (inView) {
			visibleItems.add(item);
		}
	});
});
