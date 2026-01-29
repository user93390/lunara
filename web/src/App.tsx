import './styles/style.scss';

import CredentialLoginPage from './pages/login'
import HomePage from './pages/home.tsx'

import { Routes, Route } from 'react-router-dom';

function App() {
    return (
        <Routes>
            <Route path="/" element={<HomePage/>} />
            <Route path="/login" element={<CredentialLoginPage/>} />
        </Routes>
    );
}

export default App;