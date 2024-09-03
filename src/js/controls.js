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
			btn.classList.add("decorum-tb-btn");

			let timer;
			const show_snap_overlay = () => {
				win.setFocus().then(() => invoke("plugin:decorum|show_snap_overlay"));
			};

			switch (id) {
				case "minimize":
					btn.innerHTML = "\uE921";

					btn.addEventListener("click", () => {
						clearTimeout(timer);
						win.minimize();
					});

					break;
				case "maximize":
					btn.innerHTML = "\uE922";
					win.onResized(() => {
						win.isMaximized().then((maximized) => {
							if (maximized) {
								btn.innerHTML = "\uE923";
							} else {
								btn.innerHTML = "\uE922";
							}
						});
					});

					btn.addEventListener("click", () => {
						clearTimeout(timer);
						win.toggleMaximize();
					});
					btn.addEventListener("mouseleave", () => clearTimeout(timer));
					btn.addEventListener("mouseenter", () => {
						timer = setTimeout(show_snap_overlay, 620);
					});
					break;
				case "close":
					btn.innerHTML = "\uE8BB";
					btn.addEventListener("click", () => win.close());
					break;
			}

			tbEl.appendChild(btn);
		};

		// Before eval-ing, the line below is modified from the rust side
		// to only include the controls that are enabled on the window
		["minimize", "maximize", "close"].forEach(createButton);

		const style = document.createElement("style");
		document.head.appendChild(style);

		style.innerHTML = `
			.decorum-tb-btn {
				cursor: default;
				width: 58px;
				height: 32px;
				border: none;
				padding: 0px;
				outline: none;
				display: flex;
				font-size: 10px;
				font-weight: 300;
				box-shadow: none;
				border-radius: 0;
				align-items: center;
				justify-content: center;
				transition: background 0.1s;
				background-color: transparent;
				text-rendering: optimizeLegibility;
				-webkit-font-smoothing: antialiased;
				font-family: 'Segoe Fluent Icons', 'Segoe MDL2 Assets';
			}

			.decorum-tb-btn:hover {
				background-color: rgba(0,0,0,0.2);
			}

			#decorum-tb-close:hover {
				background-color: rgba(255,0,0,0.7) !important;
			}
		`;
	});

	// Use MutationObserver to watch for changes
	const observer = new MutationObserver((mutations) => {
		for (let mutation of mutations) {
			if (mutation.type === 'childList') {
				const tbEl = document.querySelector("[data-tauri-decorum-tb]");
				if (tbEl) {
					debouncedCreateControls();
					break;
				}
			}
		}
	});

	observer.observe(document.body, {
		childList: true,
		subtree: true,
	});
});
