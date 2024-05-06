import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrent, LogicalSize } from "@tauri-apps/api/window";
import CloseIcon from "./assets/close.svg";
import MaximizeIcon from "./assets/maximize.svg";
import MinimizeIcon from "./assets/minimize.svg";


const Button = ({ action }:
	{ action: "minimize" | "maximize" | "close" }) => {

	const isClose = action === "close";
	const icon = isClose ? CloseIcon : action === "maximize" ? MaximizeIcon : MinimizeIcon;

	useEffect(() => {

		const win = getCurrent();
		const buttonEl = document.querySelector(`#${action}`);
		let timer;

		switch (action) {
			case "minimize":
				buttonEl?.addEventListener("click", () => {
					win?.minimize();
				});

				buttonEl?.addEventListener("mouseenter", () => {
					timer = setTimeout(() => {
						invoke("show_snap_overlay");
					}, 600);
				});

				buttonEl?.addEventListener("mouseleave", () => {
					clearTimeout(timer);
				});
				break;

			case "maximize":
				buttonEl?.addEventListener("click", () => {
					// win?.maximize();
					win?.isMaximized().then((maximized) => {
						if (maximized) {
							win?.setSize(new LogicalSize(800, 600));
							win?.center();
						} else {
							win?.maximize();
						}
					});
				});

				buttonEl?.addEventListener("dblclick", (e) => {
					e.preventDefault();
					invoke("show_snap_overlay");
				});

				buttonEl?.addEventListener("contextmenu", (e) => {
					e.preventDefault();
				});

				buttonEl?.addEventListener("mouseenter", () => {
					timer = setTimeout(() => {
						invoke("show_snap_overlay");
					}, 600);
				});

				buttonEl?.addEventListener("mouseleave", () => {
					clearTimeout(timer);
				});
				break;

			case "close":
				buttonEl?.addEventListener("click", () => {
					win?.close();
				});
				break;
		}
	}, [])

	return <button
		id={action}
		style={{
			width: "46px",
			height: "32px",
			display: "flex",
			justifyContent: "center",
			alignItems: "center",
			background: "transparent",
			border: "none",
			outline: "none",
			boxShadow: "none",
			borderRadius: 0,
		}}
	>
		<img src={icon} />
	</button>
}

export default function Titlebar() {

	return <div
		style={{
			width: "100%",
			position: "fixed",
			top: 0,
			left: 0,
			right: 0,
			height: "32px",
			zIndex: 1000,
			display: "flex",
			justifyContent: "end",
			alignItems: "end",
		}}>
		<style>{
			`
			button {
				transition: background 0.1s;
			}

			button:hover {
				background: rgba(0,0,0,0.2) !important;
			}

			#close:hover {
				background: rgba(255,0,0,0.7) !important;
			}
		`}
		</style>
		<div
			data-tauri-drag-region
			style={{
				background: "transparent",
				width: "100%",
				height: "100%"
			}}
		/>
		<Button action="minimize" />
		<Button action="maximize" />
		<Button action="close" />
	</div>
}