
document.addEventListener("DOMContentLoaded", () => {
	let tbEl = document.querySelector("[data-tauri-decorum-tb]");

	if (!tbEl) {
		console.log(
			"DECORUM: Element with data-tauri-decorum-tb not found. Creating one.",
		);

		// Create titlebar element
		tbEl = document.createElement("div");
		tbEl.setAttribute("data-tauri-decorum-tb", "");
		tbEl.style.top = 0;
		tbEl.style.left = 0;
		tbEl.style.zIndex = 100;
		tbEl.style.width = "100%";
		tbEl.style.height = "32px";
		tbEl.style.display = "flex";
		tbEl.style.position = "fixed";
		tbEl.style.alignItems = "end";
		tbEl.style.justifyContent = "end";
		tbEl.style.backgroundColor = "transparent";

		// Create draggable area
		const drag = document.createElement("div");
		drag.style.width = "100%";
		drag.style.height = "100%";
		drag.style.background = "transparent";
		drag.setAttribute("data-tauri-drag-region", "");
		tbEl.appendChild(drag);

		// add tbEl to the body
		document.body.appendChild(tbEl);
	}
});