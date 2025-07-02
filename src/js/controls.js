function waitForElm(selector) {
	return new Promise((resolve) => {
		if (document.querySelector(selector)) {
			return resolve(document.querySelector(selector));
		}

		const observer = new MutationObserver((mutations) => {
			if (document.querySelector(selector)) {
				observer.disconnect();
				resolve(document.querySelector(selector));
			}
		});

		// If you get "parameter 1 is not of type 'Node'" error, see https://stackoverflow.com/a/77855838/492336
		observer.observe(document.body, {
			childList: true,
			subtree: true,
		});
	});
}

document.addEventListener("DOMContentLoaded", () => {
	const tauri = window.__TAURI__;

	if (!tauri) {
		console.log("DECORUM: Tauri API not found. Exiting.");
		console.log(
			"DECORUM: Set withGlobalTauri: true in tauri.conf.json to enable.",
		);
		return;
	}

	const win = tauri.window.getCurrentWindow();
	const invoke = tauri.core.invoke;

	console.log("DECORUM: Waiting for [data-tauri-decorum-tb] ...");

	// Add debounce function
	const debounce = (func, delay) => {
		let timeoutId;
		return (...args) => {
			clearTimeout(timeoutId);
			timeoutId = setTimeout(() => func(...args), delay);
		};
	};

	// Debounce the control creation
	const debouncedCreateControls = debounce(() => {
		const tbEl = document.querySelector("[data-tauri-decorum-tb]");
		if (!tbEl) return;

		// Check if controls already exist
		if (tbEl.querySelector(".decorum-tb-btn")) {
			console.log("DECORUM: Controls already exist. Skipping creation.");
			return;
		}

		// Create button func
		const createButton = (id) => {
			const btn = document.createElement("button");

			btn.id = "decorum-tb-" + id;
			btn.style.width = "58px";
			btn.style.height = "32px";
			btn.style.border = "none";
			btn.style.padding = "0px";
			btn.style.outline = "none";
			btn.style.display = "flex";
			btn.style.fontSize = "10px";
			btn.style.fontWeight = "300";
			btn.style.cursor = "default";
			btn.style.boxShadow = "none";
			btn.style.borderRadius = "0px";
			btn.style.alignItems = "center";
			btn.style.justifyContent = "center";
			btn.style.transition = "background 0.1s";
			btn.style.backgroundColor = "transparent";
			btn.style.textRendering = "optimizeLegibility";
			btn.style.fontFamily = "'Segoe Fluent Icons', 'Segoe MDL2 Assets'";

			let timer;
			const show_snap_overlay = () => {
				win.setFocus().then(() =>
					invoke("plugin:decorum|show_snap_overlay")
				);
			};

			// Setup hover events
			btn.addEventListener("mouseenter", () => {
				if (id === "close") {
					btn.style.backgroundColor = "rgba(255,0,0,0.7)";
				} else {
					btn.style.backgroundColor = "rgba(0,0,0,0.2)";
				}
			});

			btn.addEventListener("mouseleave", () => {
				btn.style.backgroundColor = "transparent";
			});
			switch (id) {
				case "minimize":
					btn.innerHTML = "\uE921";
					btn.setAttribute("aria-label", "Minimize window");

					btn.addEventListener("click", () => {
						clearTimeout(timer);
						win.minimize();
					});

					break;
				case "maximize":
					btn.innerHTML = "\uE922";
					btn.setAttribute("aria-label", "Maximize window");
					win.onResized(() => {
						win.isMaximized().then((maximized) => {
							if (maximized) {
								btn.innerHTML = "\uE923";
								btn.setAttribute(
									"aria-label",
									"Restore window size"
								);
							} else {
								btn.innerHTML = "\uE922";
								btn.setAttribute(
									"aria-label",
									"Maximize window size"
								);
							}
						});
					});

					btn.addEventListener("click", () => {
						clearTimeout(timer);
						win.toggleMaximize();
					});
					btn.addEventListener("mouseleave", () =>
						clearTimeout(timer)
					);
					btn.addEventListener("mouseenter", () => {
						timer = setTimeout(show_snap_overlay, 620);
					});
					break;
				case "close":
					btn.innerHTML = "\uE8BB";
					btn.setAttribute("aria-label", "Close window");
					btn.addEventListener("click", () => win.close());
					break;
			}

			tbEl.appendChild(btn);
		};

		// Before eval-ing, the line below is modified from the rust side
		// to only include the controls that are enabled on the window
		["minimize", "maximize", "close"].forEach(createButton);
	});

	// Use MutationObserver to watch for changes
	const observer = new MutationObserver((mutations) => {
		for (let mutation of mutations) {
			if (mutation.type === "childList") {
				const tbEl = document.querySelector("[data-tauri-decorum-tb]");
				if (tbEl) {
					debouncedCreateControls();
					break;
				}
			}
		}
	});

	// data-tauri-decorum-tb may be created before observer starts
	if (document.querySelector("[data-tauri-decorum-tb]")) {
		debouncedCreateControls();
		return;
	}

	observer.observe(document.body, {
		childList: true,
		subtree: true,
	});

	debouncedCreateControls();
});
