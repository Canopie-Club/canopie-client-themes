// frontend/javascript/theme/_generic/youtube.ts
function loadYoutubeIframe(el) {
  console.log("Loading YouTube iframe");
  const videoId = el.getAttribute("data-video-id");
  el.innerHTML = `
        <iframe
            class="absolute top-0 left-0 w-full h-full rounded-xl"
            src="https://www.youtube.com/embed/${videoId}?autoplay=1"
            title="YouTube video player"
            frameborder="0"
            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
            allowfullscreen
        ></iframe>`;
}
console.log("YouTube iframe loaded, ", loadYoutubeIframe);
