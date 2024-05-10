const closeIcon = `<svg width="10" height="10" viewBox="0 0 10 10" fill="none" xmlns="http://www.w3.org/2000/svg">
<path d="M5 5.70801L0.854492 9.85352C0.756836 9.95117 0.639648 10 0.50293 10C0.359701 10 0.239258 9.9528 0.141602 9.8584C0.0472005 9.76074 0 9.6403 0 9.49707C0 9.36035 0.0488281 9.24316 0.146484 9.14551L4.29199 5L0.146484 0.854492C0.0488281 0.756836 0 0.638021 0 0.498047C0 0.429688 0.0130208 0.364583 0.0390625 0.302734C0.0651042 0.240885 0.100911 0.188802 0.146484 0.146484C0.192057 0.100911 0.245768 0.0651042 0.307617 0.0390625C0.369466 0.0130208 0.43457 0 0.50293 0C0.639648 0 0.756836 0.0488281 0.854492 0.146484L5 4.29199L9.14551 0.146484C9.24316 0.0488281 9.36198 0 9.50195 0C9.57031 0 9.63379 0.0130208 9.69238 0.0390625C9.75423 0.0651042 9.80794 0.100911 9.85352 0.146484C9.89909 0.192057 9.9349 0.245768 9.96094 0.307617C9.98698 0.366211 10 0.429688 10 0.498047C10 0.638021 9.95117 0.756836 9.85352 0.854492L5.70801 5L9.85352 9.14551C9.95117 9.24316 10 9.36035 10 9.49707C10 9.56543 9.98698 9.63053 9.96094 9.69238C9.9349 9.75423 9.89909 9.80794 9.85352 9.85352C9.8112 9.89909 9.75911 9.9349 9.69727 9.96094C9.63542 9.98698 9.57031 10 9.50195 10C9.36198 10 9.24316 9.95117 9.14551 9.85352L5 5.70801Z" fill="white"/>
</svg>
`;


const maximizeIcon = `<svg width="10" height="10" viewBox="0 0 10 10" fill="none" xmlns="http://www.w3.org/2000/svg">
<path
d="M1.47461 10C1.2793 10 1.09212 9.96094 0.913086 9.88281C0.734049 9.80143 0.576172 9.69401 0.439453 9.56055C0.30599 9.42383 0.198568 9.26595 0.117188 9.08691C0.0390625 8.90788 0 8.7207 0 8.52539V1.47461C0 1.2793 0.0390625 1.09212 0.117188 0.913086C0.198568 0.734049 0.30599 0.577799 0.439453 0.444336C0.576172 0.307617 0.734049 0.200195 0.913086 0.12207C1.09212 0.0406901 1.2793 0 1.47461 0H8.52539C8.7207 0 8.90788 0.0406901 9.08691 0.12207C9.26595 0.200195 9.4222 0.307617 9.55566 0.444336C9.69238 0.577799 9.7998 0.734049 9.87793 0.913086C9.95931 1.09212 10 1.2793 10 1.47461V8.52539C10 8.7207 9.95931 8.90788 9.87793 9.08691C9.7998 9.26595 9.69238 9.42383 9.55566 9.56055C9.4222 9.69401 9.26595 9.80143 9.08691 9.88281C8.90788 9.96094 8.7207 10 8.52539 10H1.47461ZM8.50098 8.99902C8.56934 8.99902 8.63281 8.986 8.69141 8.95996C8.75326 8.93392 8.80697 8.89811 8.85254 8.85254C8.89811 8.80697 8.93392 8.75488 8.95996 8.69629C8.986 8.63444 8.99902 8.56934 8.99902 8.50098V1.49902C8.99902 1.43066 8.986 1.36719 8.95996 1.30859C8.93392 1.24674 8.89811 1.19303 8.85254 1.14746C8.80697 1.10189 8.75326 1.06608 8.69141 1.04004C8.63281 1.014 8.56934 1.00098 8.50098 1.00098H1.49902C1.43066 1.00098 1.36556 1.014 1.30371 1.04004C1.24512 1.06608 1.19303 1.10189 1.14746 1.14746C1.10189 1.19303 1.06608 1.24674 1.04004 1.30859C1.014 1.36719 1.00098 1.43066 1.00098 1.49902V8.50098C1.00098 8.56934 1.014 8.63444 1.04004 8.69629C1.06608 8.75488 1.10189 8.80697 1.14746 8.85254C1.19303 8.89811 1.24512 8.93392 1.30371 8.95996C1.36556 8.986 1.43066 8.99902 1.49902 8.99902H8.50098Z"
fill="white" />
</svg>`;

const minimizeIcon = `<svg width="10" height="1" viewBox="0 0 10 1" fill="none" xmlns="http://www.w3.org/2000/svg">
<path
d="M0.498047 1C0.429688 1 0.364583 0.986979 0.302734 0.960938C0.244141 0.934896 0.192057 0.899089 0.146484 0.853516C0.100911 0.807943 0.0651042 0.755859 0.0390625 0.697266C0.0130208 0.635417 0 0.570312 0 0.501953C0 0.433594 0.0130208 0.370117 0.0390625 0.311523C0.0651042 0.249674 0.100911 0.195964 0.146484 0.150391C0.192057 0.101562 0.244141 0.0641276 0.302734 0.0380859C0.364583 0.0120443 0.429688 -0.000976562 0.498047 -0.000976562H9.50195C9.57031 -0.000976562 9.63379 0.0120443 9.69238 0.0380859C9.75423 0.0641276 9.80794 0.101562 9.85352 0.150391C9.89909 0.195964 9.9349 0.249674 9.96094 0.311523C9.98698 0.370117 10 0.433594 10 0.501953C10 0.570312 9.98698 0.635417 9.96094 0.697266C9.9349 0.755859 9.89909 0.807943 9.85352 0.853516C9.80794 0.899089 9.75423 0.934896 9.69238 0.960938C9.63379 0.986979 9.57031 1 9.50195 1H0.498047Z"
fill="white" />
</svg>`

const pageAccessedByReload = (
	(window.performance.navigation && window.performance.navigation.type === 1) ||
	  window.performance
		.getEntriesByType('navigation')
		.map((nav) => nav.type)
		.includes('reload')
);

window.onload = () => {
	let tbEl = document.querySelector('[data-tauri-decorum-tb]');

	if (!tbEl) {
		// Create titlebar element
		tbEl = document.createElement('div');
		tbEl.setAttribute('data-tauri-decorum-tb', '');
		tbEl.style.top = 0;
		tbEl.style.left = 0;
		tbEl.style.zIndex = 100;
		tbEl.style.width = '100%';
		tbEl.style.height = '32px';
		tbEl.style.display = 'flex';
		tbEl.style.position = 'fixed';
		tbEl.style.alignItems = 'end';
		tbEl.style.justifyContent = 'end';
		tbEl.style.backgroundColor = 'transparent';

		// Create draggable area
		const drag = document.createElement('div');
		drag.style.width = '100%';
		drag.style.height = '100%';
		drag.style.background = 'transparent'
		drag.setAttribute('data-tauri-drag-region', '');
		tbEl.appendChild(drag);

		// Create button func
		const createButton = (id) => {
			const btn = document.createElement('button');
			btn.id = "decorum-tb-" + id;
			btn.style.width = 'fit-content';
			btn.style.height = '32px';
			btn.style.border = 'none';
			btn.style.outline = 'none';
			btn.style.display = 'flex';
			btn.style.boxShadow = 'none';
			btn.style.borderRadius = '0';
			btn.style.alignItems = 'center';
			btn.style.justifyContent = 'center';
			btn.style.backgroundColor = 'transparent';

			switch (id) {
				case 'minimize':
					btn.innerHTML = minimizeIcon
					break;
				case 'maximize':
					btn.innerHTML = maximizeIcon
					break;
				case 'close':
					btn.innerHTML = closeIcon
					break;
			}

			btn.addEventListener('click', () => {
				console.log('button clicked', id);
			});
			tbEl.appendChild(btn);
		}

		createButton('minimize');
		createButton('maximize');
		createButton('close');

		// add hover styles
		const style = document.createElement('style');
		document.head.appendChild(style);

		style.innerHTML = `
		#decorum-tb-minimize, #decorum-tb-maximize, #decorum-tb-close {
			transition: background 0.1s;
		}
		#decorum-tb-minimize:hover, #decorum-tb-maximize:hover {
			background: rgba(0,0,0,0.2) !important;
		}
		
		#decorum-tb-close:hover {
			background: rgba(255,0,0,0.7) !important;
		}

		#decorum-tb-minimize svg,
		#decorum-tb-maximize svg,
		#decorum-tb-close svg {
			width: auto;
		}
	`;

		// finally add finished tbEl to the body
		document.body.appendChild(tbEl);
	}
}