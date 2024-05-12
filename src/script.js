
document.addEventListener("DOMContentLoaded", () => {
  const tauri = window.__TAURI__;

  if (!tauri) {
    console.log("DECORUM: Tauri API not found. Exiting.");
    console.log(
      "DECORUM: Set withGlobalTauri: true in tauri.conf.json to enable.",
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

      let timer;
      const show_snap_overlay = () =>
        invoke("plugin:decorum|show_snap_overlay");

      switch (id) {
        case "minimize":
          btn.innerHTML = "\uE921";
          btn.addEventListener("click", () => win.minimize());
          btn.addEventListener("mouseleave", () => clearTimeout(timer));
          btn.addEventListener("mouseenter", () => {
            timer = setTimeout(show_snap_overlay, 500);
          });
          break;
        case "maximize":
          btn.innerHTML = "\uE922";
          btn.addEventListener("click", () => win.maximize());
          btn.addEventListener("mouseleave", () => clearTimeout(timer));
          btn.addEventListener("mouseenter", () => {
            timer = setTimeout(show_snap_overlay, 500);
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

    const style = document.createElement("style");
    document.head.appendChild(style);

    style.innerHTML = `
		.decorum-tb-btn {
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

    // finally add finished tbEl to the body
    document.body.appendChild(tbEl);
  }
});
