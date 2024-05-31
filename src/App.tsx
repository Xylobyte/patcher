import {BrowserRouter, Navigate, Route, Routes} from "react-router-dom";
import "./App.scss";

function App() {
    return <BrowserRouter>
        <Routes>
            <Route index element={<Navigate to="/home"/>}/>
            <Route path="/home" element={<div>Home</div>}/>
            <Route path="/editor" element={<div>Editor</div>}/>
        </Routes>
    </BrowserRouter>;
}

export default App;
