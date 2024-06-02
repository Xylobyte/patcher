import {BrowserRouter, Navigate, Route, Routes} from "react-router-dom";
import "./App.scss";
import DragHeader from "./components/drag-header/DragHeader.tsx";
import RecentProjects from "./pages/recent-projects/RecentProjects.tsx";

function App() {
    return <BrowserRouter>
        <DragHeader/>

        <Routes>
            <Route index element={<Navigate to="/recent-projects"/>}/>
            <Route path="/recent-projects" element={<RecentProjects/>}/>
            <Route path="/main" element={<div>Editor</div>}/>
        </Routes>
    </BrowserRouter>;
}

export default App;
