document.addEventListener("DOMContentLoaded", () => {
  const tauri = window.__TAURI__;

  if (!tauri) {
    console.log("DECORUM: Tauri API not found. Exiting.");
    console.log(
      "DECORUM: Set withGlobalTauri to true in tauri.conf.json to enable Tauri API.",
    );
    return;
  }

  const win = tauri.window.getCurrent();
  const invoke = tauri.core.invoke;

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

    // Create button func
    const createButton = (id) => {
      const btn = document.createElement("button");
      btn.id = "decorum-tb-" + id;
      btn.classList.add("decorum-tb-btn")
      btn.style.padding = "0";
      btn.style.width = "58px";
      btn.style.height = "32px";
      btn.style.border = "none";
      btn.style.outline = "none";
      btn.style.display = "flex";
      btn.style.boxShadow = "none";
      btn.style.borderRadius = "0";
      btn.style.alignItems = "center";
      btn.style.justifyContent = "center";
      btn.style.backgroundColor = "transparent";

      let timer;

      const show_snap_overlay = () =>
        invoke("plugin:decorum|show_snap_overlay");

      switch (id) {
        case "minimize":
          btn.innerHTML = "\uE921";
          btn.addEventListener("click", () => win.minimize());
          btn.addEventListener("mouseleave", () => clearTimeout(timer));
          btn.addEventListener("mouseenter", () => {
            timer = setTimeout(show_snap_overlay, 600);
          });
          break;
        case "maximize":
          btn.innerHTML = "\uE922";
          btn.innerHTML = "\uE923";
          btn.addEventListener("click", () => win.maximize());
          btn.addEventListener("mouseleave", () => clearTimeout(timer));
          btn.addEventListener("mouseenter", () => {
            timer = setTimeout(show_snap_overlay, 600);
          });
          break;
        case "close":
          btn.innerHTML = "\uE8BB";
          btn.addEventListener("click", () => win.close());
          break;
      }

      tbEl.appendChild(btn);
    };

    ["minimize", "maximize", "close"].forEach(createButton);

    // add hover styles
    const style = document.createElement("style");
    document.head.appendChild(style);

    style.innerHTML = `
		.decorum-tb-btn {
      font-size: 10px;
      font-weight: 300;
      font-family: 'Segoe Fluent Icons', 'Segoe MDL2 Assets';
			transition: background 0.1s;
      text-rendering: optimizeLegibility;
      -webkit-font-smoothing: antialiased;
		}

		#decorum-tb-minimize:hover, #decorum-tb-maximize:hover {
			background: rgba(0,0,0,0.2) !important;
		}

		#decorum-tb-close:hover {
			background: rgba(255,0,0,0.7) !important;
		}
	`;

    // finally add finished tbEl to the body
    document.body.appendChild(tbEl);
  }
});
