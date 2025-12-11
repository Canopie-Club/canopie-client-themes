// ../morningstar/frontend/javascript/spa.ts
document.addEventListener("DOMContentLoaded", () => {
  const links = document.querySelectorAll("a[href^='/']");
  const sections = [];
  document.querySelectorAll("section[id]").forEach((el) => {
    const id = el.id;
    const title = id.charAt(0).toUpperCase() + id.slice(1);
    sections.push({ id, title, element: el });
  });
  let currentSectionId = null;
  let isProgrammaticScroll = false;
  function scrollToSection(id, smooth = true) {
    const section = sections.find((s) => s.id === id);
    if (!section)
      return;
    isProgrammaticScroll = true;
    section.element.scrollIntoView({
      behavior: smooth ? "smooth" : "auto",
      block: "start"
    });
    setTimeout(() => {
      isProgrammaticScroll = false;
    }, 800);
  }
  function updateHistoryAndTitle(id, title, push = true) {
    const section = sections.find((s) => s.id === id);
    if (!section)
      return;
    const newTitle = title || section.title;
    const newUrl = `/${id === "home" ? "" : id}`;
    if (push) {
      history.pushState({ section: id }, newTitle, newUrl);
    } else {
      history.replaceState({ section: id }, newTitle, newUrl);
    }
    document.title = newTitle;
  }
  links.forEach((link) => {
    link.addEventListener("click", (event) => {
      const href = link.getAttribute("href");
      if (!href || href.startsWith("http"))
        return;
      event.preventDefault();
      const id = href.replace(/^\//, "") || "home";
      const title = link.getAttribute("page-title") || undefined;
      updateHistoryAndTitle(id, title);
      scrollToSection(id);
    });
  });
  window.addEventListener("popstate", (event) => {
    const id = event.state?.section || window.location.pathname.replace(/^\//, "") || "home";
    scrollToSection(id, false);
    updateHistoryAndTitle(id, "", false);
  });
  const visibleItems = new Set;
  const observer = new IntersectionObserver((entries) => {
    if (isProgrammaticScroll)
      return;
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        visibleItems.add(entry.target);
      } else {
        visibleItems.delete(entry.target);
      }
    });
    const visibleElements = [...visibleItems];
    const lastItem = visibleElements[visibleElements.length - 1];
    const id = lastItem.id;
    const title = lastItem.getAttribute("page-title") || undefined;
    if (id !== currentSectionId) {
      currentSectionId = id;
      updateHistoryAndTitle(id, title);
    }
  }, { threshold: 0 });
  function initObservers() {
    sections.forEach(({ element }) => {
      observer.observe(element);
      const rect = element.getBoundingClientRect();
      const inView = rect.bottom > 0 && rect.right > 0 && rect.top < window.innerHeight && rect.left < window.innerWidth;
      if (inView) {
        visibleItems.add(element);
      }
    });
  }
  const initialId = window.location.pathname.replace(/^\//, "") || "home";
  currentSectionId = initialId;
  scrollToSection(initialId, false);
  initObservers();
  updateHistoryAndTitle(initialId, undefined, false);
});
