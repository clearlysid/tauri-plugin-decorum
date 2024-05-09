import reactLogo from "./assets/react.svg";
import Titlebar from "./Titlebar";

function App() {


	return (
		<div className="container">
			{/* <Titlebar /> */}
			<h1>Welcome to Tauri!</h1>

			<div className="row">
				<a href="https://vitejs.dev" target="_blank">
					<img src="/vite.svg" className="logo vite" alt="Vite logo" />
				</a>
				<a href="https://tauri.app" target="_blank">
					<img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
				</a>
				<a href="https://reactjs.org" target="_blank">
					<img src={reactLogo} className="logo react" alt="React logo" />
				</a>
			</div>
			<p>Click on the Tauri, Vite, and React logos to learn more.</p>
		</div>
	);
}

export default App;