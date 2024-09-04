/**
 * @param {string} selector
 * @returns {Promise<HTMLElement>}
 */
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
  // All of this tags will be replaced by the found system theme icons
  const windowCloseSvg = `@win-close`;
  const windowMinimizeSvg = `@win-minimize`;
  const windowMaximizeSvg = `@win-maximize`;
  const windowRestoreSvg = `@win-restore`;

  const tauri = window.__TAURI__;

  if (!tauri) {
    console.log("DECORUM: Tauri API not found. Exiting.");
    console.log(
      "DECORUM: Set withGlobalTauri: true in tauri.conf.json to enable.",
    );
    return;
  }

  const win = tauri.window.getCurrentWindow();

  console.log("DECORUM: Waiting for [data-tauri-decorum-tb] ...");

  waitForElm("[data-tauri-decorum-tb]").then((tbEl) => {
    const actions = document.createElement("div");
    actions.className = "decorum-tb-actions";
    actions.style.width = "fit-content";
    actions.style.display = "flex";
    actions.style.paddingRight = "0.5em";
    actions.style.gap = "0.8125em";

    // Create button func
    const createButton = (id) => {
      console.debug("createButton", id);
      const btn = document.createElement("button");
      btn.id = "decorum-tb-" + id;
      btn.classList.add("decorum-tb-btn");

      switch (id) {
        case "minimize":
          btn.innerHTML = windowMinimizeSvg;

          btn.addEventListener("click", () => {
            clearTimeout(timer);
            win.minimize();
          });

          break;
        case "maximize":
          btn.innerHTML = windowMaximizeSvg;
          win.onResized(() => {
            win.isMaximized().then((maximized) => {
              if (maximized) {
                btn.innerHTML = windowRestoreSvg;
              } else {
                btn.innerHTML = windowMaximizeSvg;
              }
            });
          });

          btn.addEventListener("click", () => {
            btn.blur();
            win.toggleMaximize();
          });

          break;
        case "close":
          btn.innerHTML = windowCloseSvg;
          btn.addEventListener("click", () => win.close());
          break;
      }

      actions.appendChild(btn);
    };

    // Before eval-ing, the line below is modified from the rust side
    // to only include the controls that are enabled on the window
    ["minimize", "maximize", "close"].forEach(createButton);

    tbEl.appendChild(actions);

    const style = document.createElement("style");
    document.head.appendChild(style);

    style.innerHTML = `
			.decorum-tb-btn {
        color: white;
				cursor: default;
				border: none;
				padding: 0px;
      	width: 1.5em;
				height: 1.5em;
				outline: none;
				display: flex;
				box-shadow: none;
				align-items: center;
				justify-content: center;
				transition: background 0.1s;
      	border-radius: 50%;
				background-color: var(--decorum-tb-actions-icon-bg, rgba(255, 255, 255, 0.2));
			}

      .decorum-tb-btn:hover {
        background-color: var(--decorum-tb-actions-icon-active-bg, rgba(255, 255, 255, 0.4));
      }

      .decorum-tb-btn svg {
      	width: 16px;
				height: 16px;
      }

      .decorum-tb-btn svg path {
        fill: var(--decorum-tb-actions-icon-fg, #ffffff);
      }
		`;
  });
});
