/**
 * SPA navigation + scroll tracking for section-based sites
 * Works with links like <a href="/vids"> and sections with matching IDs
 */

interface SectionInfo {
	id: string;
	title: string;
	element: HTMLElement;
}

document.addEventListener("DOMContentLoaded", () => {
	const links = document.querySelectorAll<HTMLAnchorElement>("a[href^='/']");
	const sections: SectionInfo[] = [];

	// Collect sections and build title map
	document.querySelectorAll<HTMLElement>("section[id]").forEach((el) => {
		const id = el.id;
		const title = id.charAt(0).toUpperCase() + id.slice(1); // e.g. "band" → "Band"
		sections.push({ id, title, element: el });
	});

	let currentSectionId: string | null = null;
	let isProgrammaticScroll = false;

	/**
	 * Scroll smoothly to a section by ID.
	 */
	function scrollToSection(id: string, smooth = true) {
		const section = sections.find((s) => s.id === id);
		if (!section) return;
		isProgrammaticScroll = true;
		section.element.scrollIntoView({
			behavior: smooth ? "smooth" : "auto",
			block: "start",
		});
		setTimeout(() => {
			isProgrammaticScroll = false;
		}, 800); // reset after animation
	}

	/**
	 * Update URL and title based on current section.
	 */
	function updateHistoryAndTitle(id: string, title?: string, push = true) {
		const section = sections.find((s) => s.id === id);
		if (!section) return;

		const newTitle = title || section.title;
		const newUrl = `/${id === "home" ? "" : id}`;

		if (push) {
			history.pushState({ section: id }, newTitle, newUrl);
		} else {
			history.replaceState({ section: id }, newTitle, newUrl);
		}
		document.title = newTitle;
	}

	/**
	 * Handle internal link clicks.
	 */
	links.forEach((link) => {
		link.addEventListener("click", (event) => {
			const href = link.getAttribute("href");
			if (!href || href.startsWith("http")) return; // skip external links
			event.preventDefault();
			const id = href.replace(/^\//, "") || "home";
			const title = link.getAttribute("page-title") || undefined;
			updateHistoryAndTitle(id, title);
			scrollToSection(id);
		});
	});

	/**
	 * Handle browser back/forward navigation
	 */
	window.addEventListener("popstate", (event) => {
		const id =
			event.state?.section ||
			window.location.pathname.replace(/^\//, "") ||
			"home";
		scrollToSection(id, false);
		updateHistoryAndTitle(id, "", false);
	});

	/**
	 * Observe scrolling to update URL/title
	 */
	const visibleItems = new Set<HTMLElement>();
	const observer = new IntersectionObserver(
		(entries) => {
			if (isProgrammaticScroll) return;
			entries.forEach((entry) => {
				if (entry.isIntersecting) {
					visibleItems.add(entry.target as HTMLElement);
				} else {
					visibleItems.delete(entry.target as HTMLElement);
				}
			});

			const visibleElements = [...visibleItems];

			const lastItem = visibleElements[visibleElements.length - 1];
			const id = lastItem?.id;
			const title = lastItem?.getAttribute("page-title") || undefined;
			if (id && id !== currentSectionId) {
				currentSectionId = id;
				updateHistoryAndTitle(id, title);
			}
		},
		{ threshold: 0 },
	);

	function initObservers() {
		// Observe + initial visibility check
		sections.forEach(({ element }) => {
			observer.observe(element);

			const rect = element.getBoundingClientRect();
			const inView =
				rect.bottom > 0 &&
				rect.right > 0 &&
				rect.top < window.innerHeight &&
				rect.left < window.innerWidth;

			// console.log(element.id, {
			// 	bottom: rect.bottom,
			// 	right: rect.right,
			// 	top: rect.top,
			// 	left: rect.left,
			// 	inView,
			// });

			if (inView) {
				visibleItems.add(element);
			}
		});
	}

	/**
	 * On page load — scroll instantly to the section in URL
	 */
	const initialId = window.location.pathname.replace(/^\//, "") || "home";
	currentSectionId = initialId;
	scrollToSection(initialId, false);
	initObservers();
	updateHistoryAndTitle(initialId, undefined, false);
});
