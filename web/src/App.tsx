import { Routes, Route } from 'react-router-dom';
import LoginPage from "./authentication/LoginPage.tsx";
import HomePage from "./home/HomePage.tsx";
import PluginPage from "./plugins/PluginPage.tsx";

function App() {
    return (
        <Routes>
            <Route path="/" element={<HomePage/>} />
            <Route path="/login" element={<LoginPage/>}/>
            <Route path="/marketplace" element={<PluginPage/>}/>
        </Routes>
    );
}

export default App
