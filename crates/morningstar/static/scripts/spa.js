/**
 * SPA navigation + scroll tracking for section-based sites
 * Works with links like <a href="/vids"> and sections with matching IDs
 */ function _array_like_to_array(arr, len) {
    if (len == null || len > arr.length) len = arr.length;
    for(var i = 0, arr2 = new Array(len); i < len; i++)arr2[i] = arr[i];
    return arr2;
}
function _array_without_holes(arr) {
    if (Array.isArray(arr)) return _array_like_to_array(arr);
}
function _iterable_to_array(iter) {
    if (typeof Symbol !== "undefined" && iter[Symbol.iterator] != null || iter["@@iterator"] != null) return Array.from(iter);
}
function _non_iterable_spread() {
    throw new TypeError("Invalid attempt to spread non-iterable instance.\\nIn order to be iterable, non-array objects must have a [Symbol.iterator]() method.");
}
function _to_consumable_array(arr) {
    return _array_without_holes(arr) || _iterable_to_array(arr) || _unsupported_iterable_to_array(arr) || _non_iterable_spread();
}
function _unsupported_iterable_to_array(o, minLen) {
    if (!o) return;
    if (typeof o === "string") return _array_like_to_array(o, minLen);
    var n = Object.prototype.toString.call(o).slice(8, -1);
    if (n === "Object" && o.constructor) n = o.constructor.name;
    if (n === "Map" || n === "Set") return Array.from(n);
    if (n === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(n)) return _array_like_to_array(o, minLen);
}
document.addEventListener("DOMContentLoaded", function() {
    var scrollToSection = /**
	 * Scroll smoothly to a section by ID.
	 */ function scrollToSection(id) {
        var smooth = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : true;
        var section = sections.find(function(s) {
            return s.id === id;
        });
        if (!section) return;
        isProgrammaticScroll = true;
        section.element.scrollIntoView({
            behavior: smooth ? "smooth" : "auto",
            block: "start"
        });
        setTimeout(function() {
            isProgrammaticScroll = false;
        }, 800); // reset after animation
    };
    var updateHistoryAndTitle = /**
	 * Update URL and title based on current section.
	 */ function updateHistoryAndTitle(id, title) {
        var push = arguments.length > 2 && arguments[2] !== void 0 ? arguments[2] : true;
        var section = sections.find(function(s) {
            return s.id === id;
        });
        if (!section) return;
        var newTitle = title || section.title;
        var newUrl = "/".concat(id === "home" ? "" : id);
        if (push) {
            history.pushState({
                section: id
            }, newTitle, newUrl);
        } else {
            history.replaceState({
                section: id
            }, newTitle, newUrl);
        }
        document.title = newTitle;
    };
    var initObservers = function initObservers() {
        // Observe + initial visibility check
        sections.forEach(function(param) {
            var element = param.element;
            observer.observe(element);
            var rect = element.getBoundingClientRect();
            var inView = rect.bottom > 0 && rect.right > 0 && rect.top < window.innerHeight && rect.left < window.innerWidth;
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
    };
    var links = document.querySelectorAll("a[href^='/']");
    var sections = [];
    // Collect sections and build title map
    document.querySelectorAll("section[id]").forEach(function(el) {
        var id = el.id;
        var title = id.charAt(0).toUpperCase() + id.slice(1); // e.g. "band" → "Band"
        sections.push({
            id: id,
            title: title,
            element: el
        });
    });
    var currentSectionId = null;
    var isProgrammaticScroll = false;
    /**
	 * Handle internal link clicks.
	 */ links.forEach(function(link) {
        link.addEventListener("click", function(event) {
            var href = link.getAttribute("href");
            if (!href || href.startsWith("http")) return; // skip external links
            event.preventDefault();
            var id = href.replace(/^\//, "") || "home";
            var title = link.getAttribute("page-title") || undefined;
            updateHistoryAndTitle(id, title);
            scrollToSection(id);
        });
    });
    /**
	 * Handle browser back/forward navigation
	 */ window.addEventListener("popstate", function(event) {
        var _event_state;
        var id = ((_event_state = event.state) === null || _event_state === void 0 ? void 0 : _event_state.section) || window.location.pathname.replace(/^\//, "") || "home";
        scrollToSection(id, false);
        updateHistoryAndTitle(id, "", false);
    });
    /**
	 * Observe scrolling to update URL/title
	 */ var visibleItems = new Set();
    var observer = new IntersectionObserver(function(entries) {
        if (isProgrammaticScroll) return;
        entries.forEach(function(entry) {
            if (entry.isIntersecting) {
                visibleItems.add(entry.target);
            } else {
                visibleItems.delete(entry.target);
            }
        });
        var visibleElements = _to_consumable_array(visibleItems);
        var lastItem = visibleElements[visibleElements.length - 1];
        var id = lastItem === null || lastItem === void 0 ? void 0 : lastItem.id;
        var title = (lastItem === null || lastItem === void 0 ? void 0 : lastItem.getAttribute("page-title")) || undefined;
        if (id && id !== currentSectionId) {
            currentSectionId = id;
            updateHistoryAndTitle(id, title);
        }
    }, {
        threshold: 0
    });
    /**
	 * On page load — scroll instantly to the section in URL
	 */ var initialId = window.location.pathname.replace(/^\//, "") || "home";
    currentSectionId = initialId;
    scrollToSection(initialId, false);
    initObservers();
    updateHistoryAndTitle(initialId, undefined, false);
});

